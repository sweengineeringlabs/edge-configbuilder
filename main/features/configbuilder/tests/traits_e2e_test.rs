//! Tests for `Loader::validate` behaviour via `create_loader_for_dir`.

use swe_edge_configbuilder::{create_loader_for_dir, Loader as _};

/// @covers: create_loader_for_dir
#[test]
fn test_validator_trait_accepts_valid_dir() {
    let dir = tempfile::tempdir().unwrap();
    assert!(create_loader_for_dir(dir.path()).validate().is_ok());
}

/// @covers: create_loader_for_dir
#[test]
fn test_validator_trait_rejects_file_as_dir() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not_a_dir.toml");
    std::fs::write(&file_path, b"").unwrap();
    let err = create_loader_for_dir(&file_path).validate().unwrap_err();
    assert!(matches!(err, swe_edge_configbuilder::ConfigError::Io(_)));
}
