# Changelog

## [0.5.2] - 2026-08-16

### Fixed

- Routed the last 36 direct `api::` re-exports out of `lib.rs` and through `saf/` — `lib.rs`'s entire `pub use crate::api::` block is gone, and `saf/` is now the crate's only path to any `api::` type. Non-breaking: every `swe_edge_configbuilder::TypeName` path resolves identically.

## [0.5.1] - 2026-08-16

### Changed

- Split `api/*/types/` into `dto/` and `vo/` per domain (internal reorg, no public API change), mirrored into `core/*/vo/`.

### Fixed

- Moved 12 direct `api::` re-exports out of `saf/mod.rs` into their dedicated `_svc(_factory).rs` files, restoring the one-svc-per-trait composition structure the structural audit enforces.

## [0.5.0] - 2026-08-16

### Added

- `ValueResolver` trait and default `EnvValueResolver` implementor — `{{VAR_NAME}}` substitution values can now be sourced from something other than `std::env::var` (e.g. a secrets backend), via `ConfigLoaderFactory::create_loader_with_resolver` / `create_loader_for_dir_with_resolver` / `create_loader_xdg_with_resolver`.

## [0.4.3] - 2026-08-16

### Fixed

- Completed the SAF public-surface refactor left unfinished by the 0.4.2 compliance pass: wired 9 orphaned `saf/*_svc_factory.rs` marker files and re-exported `BuilderFinalizer`, `ConfigBuilderInit`, `FeatureRegistryOps`, `FeatureStateOps`, `FeatureRecordBuilderOps`, `FeatureSummaryOps`, `PreflightReportOps`, `PreflightIssueKindOps`, `TopologyOps`, `ConfigBuilderImpl`, `SectionLoaderImpl`, `PathValidatorImpl` through `saf/` — the missing wiring had left the test suite unable to compile.
- Fixed two `ConfigError`/`String` type-mismatch bugs in the `load_in_order!`/`preflight!` macros.
- Removed dead code: orphaned shim files, a duplicated `Topology::sort`, unused policy constructors.

## [0.4.2] - 2026-06-26

### Changed

- Structural-compliance refactor: moved impl blocks from `saf/` to `core/`, tightened core impl block visibility to `pub(crate)`, added trait impls, restricted `PreflightReport` construction to the SAF facade.
- Documented all modules; refreshed rustdoc cross-links.

## [0.4.1] - 2026-06-10

### Changed

- Bumped the `dirs` dependency 5 → 6.

## [0.4.0] - 2026-06-08

### Added

- `ConfigLoaderFactory::load_section_xdg` — one-shot XDG-resolved section load.

### Changed

- CI workflow and pre-commit `arch audit` filter fixes.

## [0.3.0] - 2026-06-06

### Changed

- Migrated to the formalized SEA package layout — the crate now lives under `scm/`.

## [0.2.0] - 2026-06-06

### Changed

- Housekeeping release ahead of the SEA layout migration.

## [0.1.0] - 2026-06-05

### Added

- Layered TOML section loader with XDG path resolution.
- `load_section`, `load_section_from`, `load_section_xdg` public functions.
- 1 MiB file size guard.
- `ConfigSection` trait for self-describing config types.
