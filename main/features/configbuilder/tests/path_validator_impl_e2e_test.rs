//! End-to-end tests for `PathValidatorImpl`.
#![allow(clippy::unwrap_used)]

use swe_edge_configbuilder::create_validator;

/// @covers: path_validator_impl::PathValidatorImpl::validate_path
#[test]
fn test_path_validator_impl_existing_dir_returns_ok() {
    let dir = tempfile::tempdir().unwrap();
    assert!(create_validator().validate_path(dir.path()).is_ok());
}

/// @covers: path_validator_impl::PathValidatorImpl::validate_path
#[test]
fn test_path_validator_impl_file_path_returns_err() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("not_a_dir.toml");
    std::fs::write(&file, b"").unwrap();
    assert!(create_validator().validate_path(&file).is_err());
}
