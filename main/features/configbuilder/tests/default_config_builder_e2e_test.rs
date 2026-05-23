//! Contract tests for the default config builder constants.

use swe_edge_configbuilder::{create_config_builder, ConfigBuilder as _};

/// @covers: api/default_config_builder::DEFAULT_VERSION
#[test]
fn test_create_config_builder_default_version_is_semver() {
    let version = create_config_builder().version().to_owned();
    let parts: Vec<&str> = version.split('.').collect();
    assert_eq!(
        parts.len(),
        3,
        "default version '{version}' must be semver X.Y.Z"
    );
    for part in &parts {
        part.parse::<u64>()
            .unwrap_or_else(|_| panic!("semver part '{part}' is not a number"));
    }
}

/// @covers: api/default_config_builder::DEFAULT_VERSION
#[test]
fn test_create_config_builder_default_name_is_empty() {
    assert_eq!(create_config_builder().name(), "");
}
