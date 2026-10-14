# Source provenance and license notices

Derived from [awatcher](https://github.com/2e3s/awatcher) at commit
`d5669d3d8510415f87a6e09a456e220cc7aa3d84`.

Reconciled with upstream `main` at
`36a98fcb0b8bee90804558beefd2be889b65734a`, retaining version 0.4.0, API-key
resolution and its tests, TOML escape handling, and packaging fixes.

The original authors retain their copyright and attribution. The upstream
[Mozilla Public License 2.0](LICENSE) and existing file notices are
preserved; modifications to covered files remain under MPL-2.0. Provide the
corresponding covered source and notices when distributing these modifications,
as required by that license.

Scoped, renewable local capture credentials through aw-client-rust and source-specific consent before starting window and idle watchers.

Build the collector in the PeakActivity source graph alongside the matching aw-server-rust workspace. watchers/Cargo.toml consumes the sibling aw-server-rust/aw-client-rust source; retain its exact recorded revision.

The read-only source checks use public [aw-server-rust](https://github.com/ahmetcemkaraca/aw-server-rust)
at `78502ccd11f30dc0f19b7100ef66f71bdc57718f` as that sibling. The bundled upstream
server dependencies remain separately pinned in Cargo.toml; the collector checks
exercise the default features. Release publishing is outside this workflow.

PeakActivity is an independent project and is not affiliated with or endorsed by
the upstream authors. Release-specific source locations, revisions, notices and
publisher information must be recorded before distribution.
