use swe_edge_configbuilder::{create_config_builder, create_loader, create_validator};

#[test]
fn test_section_loader_svc_trait_impl() {
    let loader =
        ConfigLoaderFactory::ConfigLoaderFactory::create_loader().expect("loader creation");
    assert!(loader.validate().is_ok());
}

#[test]
fn test_path_validator_svc_trait_impl() {
    let validator = ConfigLoaderFactory::ConfigLoaderFactory::create_validator();
    let result = validator.validate_path(std::path::Path::new("/tmp"));
    assert!(result.is_ok());
}

#[test]
fn test_config_builder_svc_trait_impl() {
    let builder = ConfigLoaderFactory::ConfigLoaderFactory::create_config_builder();
    let name = builder.name();
    assert!(!name.is_empty());
}
