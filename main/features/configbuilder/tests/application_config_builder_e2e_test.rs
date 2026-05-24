//! Contract tests for the application-level config builder identity constants.

use swe_edge_configbuilder::{create_config_builder, ConfigBuilder as _};

/// @covers: api/application_config_builder::APP_NAME
#[test]
fn test_create_config_builder_name_matches_cargo_package_name() {
    let b = create_config_builder();
    assert_eq!(b.name(), env!("CARGO_PKG_NAME"));
}

/// @covers: api/application_config_builder::APP_VERSION
#[test]
fn test_create_config_builder_version_matches_cargo_package_version() {
    let b = create_config_builder();
    assert_eq!(b.version(), env!("CARGO_PKG_VERSION"));
}

/// @covers: api/application_config_builder::APP_NAME
#[test]
fn test_create_config_builder_with_name_overrides_preset_name() {
    let b = create_config_builder().with_name("override-name");
    assert_eq!(b.name(), "override-name");
}
