//! Tests for the ValidatorOps internal trait via PathValidatorImpl.
#![allow(clippy::unwrap_used)]

use swe_edge_configbuilder::ConfigLoaderFactory;
/// @covers: validator_ops::ValidatorOps::check_path
#[test]
fn test_validator_ops_check_path_directory_returns_ok() {
    let dir = tempfile::tempdir().unwrap();
    assert!(ConfigLoaderFactory::create_validator()
        .validate_path(dir.path())
        .is_ok());
}

/// @covers: validator_ops::ValidatorOps::check_path
#[test]
fn test_validator_ops_check_path_file_returns_err() {
    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("f.toml");
    std::fs::write(&f, b"").unwrap();
    assert!(ConfigLoaderFactory::create_validator()
        .validate_path(&f)
        .is_err());
}
