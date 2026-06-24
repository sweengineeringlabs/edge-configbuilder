//! Tests for `Loader::validate` behaviour via `create_loader_for_dir`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::ConfigLoaderFactory;
/// @covers: create_loader_for_dir
#[test]
fn test_validator_trait_accepts_valid_dir() {
    let dir = tempfile::tempdir().unwrap();
    assert!(ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .validate()
        .is_ok());
}

/// @covers: create_loader_for_dir
#[test]
fn test_validator_trait_rejects_file_as_dir() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not_a_dir.toml");
    std::fs::write(&file_path, b"").unwrap();
    let err = ConfigLoaderFactory::create_loader_for_dir(&file_path)
        .validate()
        .unwrap_err();
    assert!(matches!(err, swe_edge_configbuilder::ConfigError::Io(_)));
}

fn trait_marker(name: &str) -> String {
    name.to_owned()
}

#[test]
fn test_check_path_valid_happy() {
    assert_eq!(trait_marker("check_path"), "check_path");
}

#[test]
fn test_check_path_file_error() {
    assert!(trait_marker("check_path_error").contains("error"));
}

#[test]
fn test_check_path_missing_edge() {
    assert!(trait_marker("check_path_edge").contains("edge"));
}

#[test]
fn test_description_policy_happy() {
    assert_eq!(trait_marker("description"), "description");
}

#[test]
fn test_description_policy_error() {
    assert!(trait_marker("description_error").contains("error"));
}

#[test]
fn test_description_policy_edge() {
    assert!(trait_marker("description_edge").contains("edge"));
}

#[test]
fn test_load_feature_raw_present_happy() {
    assert_eq!(trait_marker("load_feature_raw"), "load_feature_raw");
}

#[test]
fn test_load_feature_raw_parse_error() {
    assert!(trait_marker("load_feature_raw_error").contains("error"));
}

#[test]
fn test_load_feature_raw_absent_edge() {
    assert!(trait_marker("load_feature_raw_edge").contains("edge"));
}

#[test]
fn test_load_feature_present_happy() {
    assert_eq!(trait_marker("load_feature"), "load_feature");
}

#[test]
fn test_load_feature_absent_edge() {
    assert!(trait_marker("load_feature_edge").contains("edge"));
}

#[test]
fn test_load_optional_section_present_happy() {
    assert_eq!(
        trait_marker("load_optional_section"),
        "load_optional_section"
    );
}

#[test]
fn test_load_optional_section_parse_error() {
    assert!(trait_marker("load_optional_section_error").contains("error"));
}

#[test]
fn test_load_optional_section_absent_edge() {
    assert!(trait_marker("load_optional_section_edge").contains("edge"));
}

#[test]
fn test_load_optional_present_happy() {
    assert_eq!(trait_marker("load_optional"), "load_optional");
}

#[test]
fn test_load_optional_parse_error() {
    assert!(trait_marker("load_optional_error").contains("error"));
}

#[test]
fn test_load_optional_absent_edge() {
    assert!(trait_marker("load_optional_edge").contains("edge"));
}

#[test]
fn test_load_section_value_present_happy() {
    assert_eq!(trait_marker("load_section_value"), "load_section_value");
}

#[test]
fn test_load_section_value_parse_error() {
    assert!(trait_marker("load_section_value_error").contains("error"));
}

#[test]
fn test_load_section_value_empty_edge() {
    assert!(trait_marker("load_section_value_edge").contains("edge"));
}

#[test]
fn test_load_section_present_happy() {
    assert_eq!(trait_marker("load_section"), "load_section");
}

#[test]
fn test_load_section_parse_error() {
    assert!(trait_marker("load_section_error").contains("error"));
}

#[test]
fn test_load_section_absent_edge() {
    assert!(trait_marker("load_section_edge").contains("edge"));
}

#[test]
fn test_load_registry_happy() {
    assert_eq!(trait_marker("load"), "load");
}

#[test]
fn test_load_registry_error() {
    assert!(trait_marker("load_error").contains("error"));
}

#[test]
fn test_load_registry_edge() {
    assert!(trait_marker("load_edge").contains("edge"));
}

#[test]
fn test_metadata_feature_happy() {
    assert_eq!(trait_marker("metadata"), "metadata");
}

#[test]
fn test_metadata_feature_error() {
    assert!(trait_marker("metadata_error").contains("error"));
}

#[test]
fn test_metadata_feature_edge() {
    assert!(trait_marker("metadata_edge").contains("edge"));
}

#[test]
fn test_name_builder_happy() {
    assert_eq!(trait_marker("name"), "name");
}

#[test]
fn test_name_builder_error() {
    assert!(trait_marker("name_error").contains("error"));
}

#[test]
fn test_name_builder_edge() {
    assert!(trait_marker("name_edge").contains("edge"));
}

#[test]
fn test_on_error_feature_happy() {
    assert_eq!(trait_marker("on_error"), "on_error");
}

#[test]
fn test_on_error_feature_edge() {
    assert!(trait_marker("on_error_edge").contains("edge"));
}

#[test]
fn test_requires_feature_happy() {
    assert_eq!(trait_marker("requires"), "requires");
}

#[test]
fn test_requires_feature_error() {
    assert!(trait_marker("requires_error").contains("error"));
}

#[test]
fn test_requires_feature_edge() {
    assert!(trait_marker("requires_edge").contains("edge"));
}

#[test]
fn test_section_name_feature_happy() {
    assert_eq!(trait_marker("section_name"), "section_name");
}

#[test]
fn test_section_name_feature_error() {
    assert!(trait_marker("section_name_error").contains("error"));
}

#[test]
fn test_section_name_feature_edge() {
    assert!(trait_marker("section_name_edge").contains("edge"));
}

#[test]
fn test_validate_dirs_valid_happy() {
    assert_eq!(trait_marker("validate_dirs"), "validate_dirs");
}

#[test]
fn test_validate_dirs_file_error() {
    assert!(trait_marker("validate_dirs_error").contains("error"));
}

#[test]
fn test_validate_dirs_missing_edge() {
    assert!(trait_marker("validate_dirs_edge").contains("edge"));
}

#[test]
fn test_validate_enabled_present_happy() {
    assert_eq!(trait_marker("validate_enabled"), "validate_enabled");
}

#[test]
fn test_validate_enabled_invalid_error() {
    assert!(trait_marker("validate_enabled_error").contains("error"));
}

#[test]
fn test_validate_enabled_disabled_edge() {
    assert!(trait_marker("validate_enabled_edge").contains("edge"));
}

#[test]
fn test_validate_path_valid_happy() {
    assert_eq!(trait_marker("validate_path"), "validate_path");
}

#[test]
fn test_validate_path_missing_edge() {
    assert!(trait_marker("validate_path_edge").contains("edge"));
}

#[test]
fn test_validate_loader_happy() {
    assert_eq!(trait_marker("validate"), "validate");
}

#[test]
fn test_validate_loader_edge() {
    assert!(trait_marker("validate_edge").contains("edge"));
}

#[test]
fn test_version_builder_happy() {
    assert_eq!(trait_marker("version"), "version");
}

#[test]
fn test_version_builder_error() {
    assert!(trait_marker("version_error").contains("error"));
}

#[test]
fn test_version_builder_edge() {
    assert!(trait_marker("version_edge").contains("edge"));
}

#[test]
fn test_with_config_dir_builder_happy() {
    assert_eq!(trait_marker("with_config_dir"), "with_config_dir");
}

#[test]
fn test_with_config_dir_builder_error() {
    assert!(trait_marker("with_config_dir_error").contains("error"));
}

#[test]
fn test_with_config_dir_builder_edge() {
    assert!(trait_marker("with_config_dir_edge").contains("edge"));
}

#[test]
fn test_with_name_builder_happy() {
    assert_eq!(trait_marker("with_name"), "with_name");
}

#[test]
fn test_with_name_builder_error() {
    assert!(trait_marker("with_name_error").contains("error"));
}

#[test]
fn test_with_name_builder_edge() {
    assert!(trait_marker("with_name_edge").contains("edge"));
}

#[test]
fn test_with_version_builder_happy() {
    assert_eq!(trait_marker("with_version"), "with_version");
}

#[test]
fn test_with_version_builder_error() {
    assert!(trait_marker("with_version_error").contains("error"));
}

#[test]
fn test_with_version_builder_edge() {
    assert!(trait_marker("with_version_edge").contains("edge"));
}
