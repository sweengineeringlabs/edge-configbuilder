//! Tests for `Validator::validate_path` via `create_loader_for_dir`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::{ConfigLoaderFactory, Loader as _};
/// @covers: create_loader_for_dir
#[test]
fn test_validate_section_dir_nonexistent_path_succeeds() {
    let path = std::path::Path::new("/nonexistent/swe-edge-test-path-xyz");
    assert!(!path.exists(), "test path must remain absent");
    assert!(matches!(
        ConfigLoaderFactory::create_loader_for_dir(path).validate(),
        Ok(())
    ));
}

/// @covers: create_loader_for_dir
#[test]
fn test_validate_section_dir_valid_dir_succeeds() {
    let dir = tempfile::tempdir().unwrap();
    assert!(ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .validate()
        .is_ok());
}

/// @covers: create_loader_for_dir
#[test]
fn test_validate_section_dir_file_path_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not_a_dir.toml");
    std::fs::write(&file_path, b"").unwrap();
    let err = ConfigLoaderFactory::create_loader_for_dir(&file_path)
        .validate()
        .unwrap_err();
    assert!(matches!(err, swe_edge_configbuilder::ConfigError::Io(_)));
    assert!(err.to_string().contains("not a directory"));
}
