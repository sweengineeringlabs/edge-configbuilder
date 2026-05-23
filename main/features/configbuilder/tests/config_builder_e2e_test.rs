//! Tests for `ConfigBuilder` trait via `create_config_builder`.

use swe_edge_configbuilder::{create_config_builder, ConfigBuilder as _};

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
