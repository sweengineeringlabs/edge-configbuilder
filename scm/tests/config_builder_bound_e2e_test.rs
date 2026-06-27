//! Contract tests for the default config builder constants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::{ConfigBuilder as _, ConfigLoaderFactory};
/// @covers: api/config_builder_bound::DEFAULT_VERSION
#[test]
fn test_create_config_builder_default_version_is_semver() {
    let version = ConfigLoaderFactory::create_config_builder()
        .version()
        .to_owned();
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

/// @covers: api/config_builder_bound::DEFAULT_VERSION
#[test]
fn test_create_config_builder_default_name_is_package_name() {
    assert_eq!(
        ConfigLoaderFactory::create_config_builder().name(),
        env!("CARGO_PKG_NAME")
    );
}
