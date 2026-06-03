//! Smoke tests for the `ConfigLoaderFactory` public facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::ConfigLoaderFactory;

#[test]
fn test_section_loader_svc_trait_impl() {
    let loader = ConfigLoaderFactory::create_loader().expect("loader creation");
    assert!(loader.validate().is_ok());
}

#[test]
fn test_path_validator_svc_trait_impl() {
    let validator = ConfigLoaderFactory::create_validator();
    let result = validator.validate_path(std::path::Path::new("/tmp"));
    assert!(result.is_ok());
}

#[test]
fn test_config_builder_svc_trait_impl() {
    let builder = ConfigLoaderFactory::create_config_builder();
    let name = builder.name();
    assert!(!name.is_empty());
}
