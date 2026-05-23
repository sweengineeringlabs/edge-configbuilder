//! Tests for public config service factory functions.

use std::io::Write as _;
use swe_edge_configbuilder::{
    create_loader, create_loader_for_dir, create_loader_xdg, create_validator, ConfigError,
    Loader as _, Validator as _,
};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

/// @covers: create_loader
#[test]
fn test_load_section_absent_key_returns_default() {
    let result: Result<Cfg, _> = create_loader().load_section("nonexistent_config_svc_xyz");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Cfg::default());
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_reads_section() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(f, "[cfg]\nvalue = \"svc\"").unwrap();
    let cfg: Cfg = create_loader_for_dir(dir.path())
        .load_section("cfg")
        .unwrap();
    assert_eq!(cfg.value, "svc");
}

/// @covers: create_loader_xdg
#[test]
fn test_load_section_xdg_unknown_app_returns_default() {
    let result: Result<Cfg, _> =
        create_loader_xdg("swe-edge-config-svc-nonexistent-xyz").load_section("cfg");
    assert!(result.is_ok());
}

/// @covers: create_loader_for_dir
#[test]
fn test_validate_section_dir_nonexistent_ok() {
    assert!(create_loader_for_dir("/nonexistent/config-svc-test-xyz")
        .validate()
        .is_ok());
}

/// @covers: create_validator
#[test]
fn test_validate_path_valid_dir_returns_ok() {
    let dir = tempfile::tempdir().unwrap();
    assert!(create_validator().validate_path(dir.path()).is_ok());
}

/// @covers: create_validator
#[test]
fn test_validate_path_file_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not_a_dir.toml");
    std::fs::write(&file_path, b"").unwrap();
    let err = create_validator().validate_path(&file_path).unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
    assert!(err.to_string().contains("not a directory"));
}
