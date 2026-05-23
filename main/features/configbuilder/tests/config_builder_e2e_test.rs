//! Tests for `ConfigBuilder` trait via `create_config_builder`.

use std::io::Write as _;
use swe_edge_configbuilder::{create_config_builder, ConfigBuilder as _, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_empty_name_and_default_version() {
    let b = create_config_builder();
    assert_eq!(b.name(), "");
    assert_eq!(b.version(), "0.1.0");
}

/// @covers: create_config_builder
#[test]
fn test_with_name_sets_application_name() {
    let b = create_config_builder().with_name("swe-edge");
    assert_eq!(b.name(), "swe-edge");
}

/// @covers: create_config_builder
#[test]
fn test_with_version_sets_application_version() {
    let b = create_config_builder().with_version("2.0.0");
    assert_eq!(b.version(), "2.0.0");
}

/// @covers: create_config_builder
#[test]
fn test_name_returns_configured_application_name() {
    let b = create_config_builder().with_name("edge-config");
    assert_eq!(b.name(), "edge-config");
}

/// @covers: create_config_builder
#[test]
fn test_version_returns_configured_application_version() {
    let b = create_config_builder().with_version("1.2.3");
    assert_eq!(b.version(), "1.2.3");
}

/// @covers: create_config_builder / build_loader
#[test]
fn test_build_loader_with_explicit_dir_reads_written_section() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(f, "[my_svc]\nvalue = \"found\"").unwrap();

    let cfg: Cfg = create_config_builder()
        .with_name("my-svc")
        .with_config_dir(dir.path())
        .build_loader()
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

    let cfg: MessageBrokerConfig = create_config_builder()
        .with_name("message-broker")
        .with_config_dir(dir.path())
        .build_loader()
        .load_section("message_broker")
        .unwrap();

    assert_eq!(cfg.backend, "inmemory");
    assert_eq!(cfg.nats_url, "nats://localhost:4222");
}

/// @covers: create_config_builder / build_loader
///
/// XDG path with unknown app name — no file on disk, loader returns defaults.
#[test]
fn test_build_loader_with_name_unknown_app_returns_default() {
    let cfg: Cfg = create_config_builder()
        .with_name("swe-edge-configbuilder-nonexistent-xyz")
        .build_loader()
        .load_section("any_section")
        .unwrap();
    assert_eq!(cfg, Cfg::default());
}

/// @covers: create_config_builder / build_loader
///
/// No name, no dir — falls back to `SWE_EDGE_CONFIG_DIR` or `config/`;
/// absent section returns default without panicking.
#[test]
fn test_build_loader_no_name_no_dir_returns_default_for_absent_section() {
    let cfg: Cfg = create_config_builder()
        .build_loader()
        .load_section("nonexistent_section_xyz")
        .unwrap();
    assert_eq!(cfg, Cfg::default());
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

    let cfg: Cfg = create_config_builder()
        .with_config_dir(low.path())
        .with_config_dir(high.path())
        .build_loader()
        .load_section("s")
        .unwrap();

    assert_eq!(cfg.value, "high");
}
