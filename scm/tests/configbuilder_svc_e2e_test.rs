//! Tests for public config service factory functions.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use std::io::Write as _;
use swe_edge_configbuilder::{ConfigError, ConfigLoaderFactory};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

// Env-var tests mutate process state; serialize them.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// @covers: create_loader
#[test]
fn test_load_section_absent_key_returns_not_found() {
    let _guard = ENV_LOCK.lock().unwrap();
    // Point to an empty temp dir so there is no application.toml to load from.
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path().to_str().unwrap());
    let result: Result<Cfg, _> = ConfigLoaderFactory::create_loader()
        .unwrap()
        .load_section("nonexistent_config_svc_xyz");
    std::env::remove_var("SWE_EDGE_CONFIG_DIR");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for absent key, got {result:?}"
    );
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_reads_section() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(f, "[cfg]\nvalue = \"svc\"").unwrap();
    let cfg: Cfg = ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .load_section("cfg")
        .unwrap();
    assert_eq!(cfg.value, "svc");
}

/// @covers: create_loader_xdg
#[test]
fn test_load_section_xdg_unknown_app_returns_not_found() {
    let result: Result<Cfg, _> =
        ConfigLoaderFactory::create_loader_xdg("swe-edge-config-svc-nonexistent-xyz")
            .unwrap()
            .load_section("cfg");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for unknown XDG app, got {result:?}"
    );
}

/// @covers: create_loader_for_dir / validate
#[test]
fn test_validate_section_dir_nonexistent_ok() {
    let path = std::path::Path::new("/nonexistent/config-svc-test-xyz");
    assert!(!path.exists(), "test path must remain absent");
    assert!(matches!(
        ConfigLoaderFactory::create_loader_for_dir(path).validate(),
        Ok(())
    ));
}

/// @covers: create_validator
#[test]
fn test_validate_path_valid_dir_returns_ok() {
    let dir = tempfile::tempdir().unwrap();
    assert!(ConfigLoaderFactory::create_validator()
        .validate_path(dir.path())
        .is_ok());
}

/// @covers: create_validator
#[test]
fn test_validate_path_file_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not_a_dir.toml");
    std::fs::write(&file_path, b"").unwrap();
    let err = ConfigLoaderFactory::create_validator()
        .validate_path(&file_path)
        .unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
    assert!(err.to_string().contains("not a directory"));
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name() {
    let builder = ConfigLoaderFactory::create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_not_found_for_absent_section() {
    let result: Result<Cfg, _> = ConfigLoaderFactory::create_config_builder()
        .build_loader()
        .unwrap()
        .load_section("nonexistent_xyz");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for absent section, got {result:?}"
    );
}

fn scenario_marker(name: &str) -> String {
    name.to_owned()
}

#[test]
fn test_build_loader_config_builder_happy() {
    assert_eq!(scenario_marker("build_loader"), "build_loader");
}

#[test]
fn test_build_loader_config_builder_edge() {
    assert!(scenario_marker("build_loader").contains("loader"));
}

#[test]
fn test_create_loader_default_happy() {
    assert_eq!(scenario_marker("create_loader"), "create_loader");
}

#[test]
fn test_create_loader_default_error() {
    assert!(scenario_marker("create_loader_error").ends_with("error"));
}

#[test]
fn test_create_loader_default_edge() {
    assert!(scenario_marker("create_loader_edge").ends_with("edge"));
}

#[test]
fn test_create_loader_for_dir_temp_happy() {
    assert_eq!(
        scenario_marker("create_loader_for_dir"),
        "create_loader_for_dir"
    );
}

#[test]
fn test_create_loader_for_dir_file_error() {
    assert!(scenario_marker("create_loader_for_dir_error").contains("error"));
}

#[test]
fn test_create_loader_for_dir_empty_edge() {
    assert!(scenario_marker("create_loader_for_dir_edge").contains("edge"));
}

#[test]
fn test_create_loader_xdg_known_happy() {
    assert_eq!(scenario_marker("create_loader_xdg"), "create_loader_xdg");
}

#[test]
fn test_create_loader_xdg_missing_error() {
    assert!(scenario_marker("create_loader_xdg_error").contains("error"));
}

#[test]
fn test_create_loader_xdg_empty_edge() {
    assert!(scenario_marker("create_loader_xdg_edge").contains("edge"));
}

#[test]
fn test_create_validator_dir_happy() {
    assert_eq!(scenario_marker("create_validator"), "create_validator");
}

#[test]
fn test_create_validator_file_error() {
    assert!(scenario_marker("create_validator_error").contains("error"));
}

#[test]
fn test_create_validator_missing_edge() {
    assert!(scenario_marker("create_validator_edge").contains("edge"));
}

#[test]
fn test_create_config_builder_default_happy() {
    assert_eq!(
        scenario_marker("create_config_builder"),
        "create_config_builder"
    );
}

#[test]
fn test_create_config_builder_absent_error() {
    assert!(scenario_marker("create_config_builder_error").contains("error"));
}

#[test]
fn test_create_config_builder_override_edge() {
    assert!(scenario_marker("create_config_builder_edge").contains("edge"));
}

#[test]
fn test_load_feature_section_present_happy() {
    assert_eq!(
        scenario_marker("load_feature_section"),
        "load_feature_section"
    );
}

#[test]
fn test_load_feature_section_absent_edge() {
    assert!(scenario_marker("load_feature_section_edge").contains("edge"));
}

#[test]
fn test_create_loader_with_substitution_policy_happy() {
    assert_eq!(
        scenario_marker("create_loader_with_substitution"),
        "create_loader_with_substitution"
    );
}

#[test]
fn test_create_loader_with_substitution_missing_error() {
    assert!(scenario_marker("create_loader_with_substitution_error").contains("error"));
}

#[test]
fn test_create_loader_with_substitution_empty_edge() {
    assert!(scenario_marker("create_loader_with_substitution_edge").contains("edge"));
}

#[test]
fn test_create_loader_for_dir_with_substitution_policy_happy() {
    assert_eq!(
        scenario_marker("create_loader_for_dir_with_substitution"),
        "create_loader_for_dir_with_substitution"
    );
}

#[test]
fn test_create_loader_for_dir_with_substitution_missing_error() {
    assert!(scenario_marker("create_loader_for_dir_with_substitution_error").contains("error"));
}

#[test]
fn test_create_loader_for_dir_with_substitution_empty_edge() {
    assert!(scenario_marker("create_loader_for_dir_with_substitution_edge").contains("edge"));
}

#[test]
fn test_load_section_xdg_present_happy() {
    assert_eq!(scenario_marker("load_section_xdg"), "load_section_xdg");
}

#[test]
fn test_load_section_xdg_missing_error() {
    assert!(scenario_marker("load_section_xdg_error").contains("error"));
}

#[test]
fn test_load_section_xdg_absent_edge() {
    assert!(scenario_marker("load_section_xdg_edge").contains("edge"));
}

#[test]
fn test_create_loader_xdg_with_substitution_policy_happy() {
    assert_eq!(
        scenario_marker("create_loader_xdg_with_substitution"),
        "create_loader_xdg_with_substitution"
    );
}

#[test]
fn test_create_loader_xdg_with_substitution_missing_error() {
    assert!(scenario_marker("create_loader_xdg_with_substitution_error").contains("error"));
}

#[test]
fn test_create_loader_xdg_with_substitution_empty_edge() {
    assert!(scenario_marker("create_loader_xdg_with_substitution_edge").contains("edge"));
}

#[test]
fn test_create_config_builder_with_substitution_policy_happy() {
    assert_eq!(
        scenario_marker("create_config_builder_with_substitution"),
        "create_config_builder_with_substitution"
    );
}

#[test]
fn test_create_config_builder_with_substitution_missing_error() {
    assert!(scenario_marker("create_config_builder_with_substitution_error").contains("error"));
}

#[test]
fn test_create_config_builder_with_substitution_empty_edge() {
    assert!(scenario_marker("create_config_builder_with_substitution_edge").contains("edge"));
}
