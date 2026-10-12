#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate log;

#[cfg(feature = "bundle")]
mod bundle;
mod config;

use std::error::Error;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
#[cfg(feature = "bundle")]
use tokio::sync::mpsc;
use watchers::{run_first_supported, ReportClient, WatcherType};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let config = config::from_cli()?;
    #[cfg(feature = "bundle")]
    let no_tray = config.no_tray;
    #[cfg(feature = "bundle")]
    let config_file = config.config_file;
    let config = config.watchers_config;

    if config.no_server {
        warn!(
            "Not sending to server {}:{}",
            config.client_host(),
            config.port
        );
    } else {
        info!("Sending to server {}:{}", config.client_host(), config.port);
    }
    info!(
        "Idle timeout: {} seconds",
        config.idle_timeout.num_seconds()
    );
    info!(
        "Idle polling period: {} seconds",
        config.poll_time_idle.num_seconds()
    );
    info!(
        "Window polling period: {} seconds",
        config.poll_time_window.num_seconds()
    );
    #[cfg(feature = "bundle")]
    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel();
    #[cfg(feature = "bundle")]
    let bundle_handle = tokio::spawn(bundle::run(
        config.host.clone(),
        config.port,
        config_file,
        no_tray,
        shutdown_send,
    ));

    let client = Arc::new(ReportClient::new(config).await?);

    let policy = if std::env::var("PEAKACTIVITY_CAPTURE").as_deref() == Ok("1") {
        let mut policy: watchers::CapturePolicy = serde_json::from_str(&std::env::var("PEAKACTIVITY_CAPTURE_POLICY")?)?;
        policy.validate()?;
        Some(policy)
    } else { None };
    let idle_enabled = policy.as_ref().is_none_or(|p| p.active(chrono::Utc::now()) && p.idle);
    let window_enabled = policy.as_ref().is_none_or(|p| p.active(chrono::Utc::now()) && (p.window || p.browser));
    let idle_client = Arc::clone(&client);
    let idle_future = async move {
        if idle_enabled { run_first_supported(idle_client, &WatcherType::Idle).await }
        else { std::future::pending().await }
    };
    let active_window_future = async move {
        if window_enabled { run_first_supported(client, &WatcherType::ActiveWindow).await }
        else { std::future::pending().await }
    };
    let sigterm = async {
        signal(SignalKind::terminate()).unwrap().recv().await;
        warn!("Caught SIGTERM, shutting down...");
    };
    let sigint = async {
        signal(SignalKind::interrupt()).unwrap().recv().await;
        warn!("Caught SIGINT, shutting down...");
    };

    #[cfg(not(feature = "bundle"))]
    {
        tokio::select!(
            _ = tokio::spawn(idle_future) => Ok(()),
            _ = tokio::spawn(active_window_future) => Ok(()),
            () = sigterm => Ok(()),
            () = sigint => Ok(()),
        )
    }

    #[cfg(feature = "bundle")]
    {
        tokio::select!(
            _ = bundle_handle => Ok(()),
            _ = tokio::spawn(idle_future) => Ok(()),
            _ = tokio::spawn(active_window_future) => Ok(()),
            () = sigterm => Ok(()),
            () = sigint => Ok(()),
            _ = shutdown_recv.recv() => Ok(()),
        )
    }
}
