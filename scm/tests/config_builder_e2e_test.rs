//! Tests for `ConfigBuilder` trait via `create_config_builder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigError, ConfigLoaderFactory, Loader as _};

use std::io::Write as _;

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

// ── builder fluent API ───────────────────────────────────────────────────────

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_package_name_and_version() {
    let b = ConfigLoaderFactory::create_config_builder();
    assert_eq!(b.name(), env!("CARGO_PKG_NAME"));
    assert_eq!(b.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: create_config_builder
#[test]
fn test_with_name_sets_application_name() {
    let b = ConfigLoaderFactory::create_config_builder().with_name("swe-edge");
    assert_eq!(b.name(), "swe-edge");
}

/// @covers: create_config_builder
#[test]
fn test_with_version_sets_application_version() {
    let b = ConfigLoaderFactory::create_config_builder().with_version("2.0.0");
    assert_eq!(b.version(), "2.0.0");
}

// ── build_loader happy paths ─────────────────────────────────────────────────

/// @covers: create_config_builder / build_loader
#[test]
fn test_build_loader_with_explicit_dir_reads_written_section() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(f, "[my_svc]\nvalue = \"found\"").unwrap();

    let cfg: Cfg = ConfigLoaderFactory::create_config_builder()
        .with_name("my-svc")
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap()
        .load_section("my_svc")
        .unwrap();

    assert_eq!(cfg.value, "found");
}

/// @covers: create_config_builder / build_loader
///
/// Simulates the message-broker use-case: named app, explicit config dir,
/// loads `[message_broker]` section from `application.toml`.
#[test]
fn test_build_loader_loads_message_broker_style_config() {
    #[derive(Debug, Default, serde::Deserialize, PartialEq)]
    #[serde(default)]
    struct MessageBrokerConfig {
        backend: String,
        nats_url: String,
    }

    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(
        f,
        "[message_broker]\nbackend = \"inmemory\"\nnats_url = \"nats://localhost:4222\""
    )
    .unwrap();

    let cfg: MessageBrokerConfig = ConfigLoaderFactory::create_config_builder()
        .with_name("message-broker")
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap()
        .load_section("message_broker")
        .unwrap();

    assert_eq!(cfg.backend, "inmemory");
    assert_eq!(cfg.nats_url, "nats://localhost:4222");
}

/// @covers: create_config_builder / build_loader
///
/// XDG path with unknown app name — no file on disk, loader returns NotFound.
#[test]
fn test_build_loader_with_name_unknown_app_returns_not_found() {
    let result: Result<Cfg, _> = ConfigLoaderFactory::create_config_builder()
        .with_name("swe-edge-configbuilder-nonexistent-xyz")
        .build_loader()
        .unwrap()
        .load_section("any_section");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for unknown app, got {result:?}"
    );
}

/// @covers: create_config_builder / build_loader
///
/// No name, no dir — falls back to `SWE_EDGE_CONFIG_DIR` or `config/`;
/// absent application.toml returns NotFound.
#[test]
fn test_build_loader_no_name_no_dir_returns_not_found_for_absent_section() {
    let result: Result<Cfg, _> = ConfigLoaderFactory::create_config_builder()
        .build_loader()
        .unwrap()
        .load_section("nonexistent_section_xyz");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound when no config files exist, got {result:?}"
    );
}

/// @covers: create_config_builder / build_loader
///
/// Multiple `with_config_dir` calls layer configs — later directory wins on conflict.
#[test]
fn test_build_loader_later_config_dir_wins_on_conflict() {
    let low = tempfile::tempdir().unwrap();
    let high = tempfile::tempdir().unwrap();

    let mut f = std::fs::File::create(low.path().join("application.toml")).unwrap();
    writeln!(f, "[s]\nvalue = \"low\"").unwrap();

    let mut f = std::fs::File::create(high.path().join("application.toml")).unwrap();
    writeln!(f, "[s]\nvalue = \"high\"").unwrap();

    let cfg: Cfg = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(low.path())
        .with_config_dir(high.path())
        .build_loader()
        .unwrap()
        .load_section("s")
        .unwrap();

    assert_eq!(cfg.value, "high");
}

// ── build_loader sad paths ───────────────────────────────────────────────────

/// @covers: create_config_builder / build_loader
#[test]
fn test_build_loader_load_section_returns_parse_error_for_malformed_toml() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), b"not = [broken toml").unwrap();

    let err = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap()
        .load_section::<Cfg>("any")
        .unwrap_err();

    assert!(matches!(err, ConfigError::Parse(_)));
}

/// @covers: create_config_builder / build_loader
#[test]
fn test_build_loader_load_section_returns_io_error_for_oversized_file() {
    let dir = tempfile::tempdir().unwrap();
    let oversized = vec![b'#'; 1_048_577];
    std::fs::write(dir.path().join("application.toml"), &oversized).unwrap();

    let err = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap()
        .load_section::<Cfg>("any")
        .unwrap_err();

    assert!(matches!(err, ConfigError::Io(_)));
    assert!(err.to_string().contains("1 MiB"));
}

/// @covers: create_config_builder / build_loader / validate
#[test]
fn test_build_loader_returns_error_when_config_dir_is_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("not_a_dir.toml");
    std::fs::write(&file, b"").unwrap();

    let err = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(&file)
        .build_loader()
        .err()
        .expect("expected build_loader to fail when config_dir is a file");

    assert!(matches!(err, ConfigError::Io(_)));
    assert!(err.to_string().contains("not a directory"));
}
