//! Integration tests for [`ConfigLoaderFactory`].

use swe_edge_configbuilder::{ConfigBuilder as _, ConfigLoaderFactory};

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_that_can_build_loader() {
    let _loader = ConfigLoaderFactory::create_config_builder().build_loader();
}

/// @covers: create_loader_for_dir
#[test]
fn test_create_loader_for_dir_accepts_temp_dir() {
    let dir = std::env::temp_dir();
    let _loader = ConfigLoaderFactory::create_loader_for_dir(dir);
}

/// @covers: create_validator
#[test]
fn test_create_validator_returns_path_validator() {
    let _v = ConfigLoaderFactory::create_validator();
}
