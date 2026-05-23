//! Contract tests for the validator error message constant.

use swe_edge_configbuilder::{create_validator, Validator as _};

/// @covers: api/default_validator::NOT_A_DIR_MSG
#[test]
fn test_validate_path_file_error_contains_not_a_directory_phrase() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("config.toml");
    std::fs::write(&file, b"").unwrap();
    let err = create_validator().validate_path(&file).unwrap_err();
    assert!(
        err.to_string().contains("not a directory"),
        "error must contain 'not a directory': {err}"
    );
}

/// @covers: api/default_validator::NOT_A_DIR_MSG
#[test]
fn test_validate_path_file_error_includes_the_offending_path() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("config.toml");
    std::fs::write(&file, b"").unwrap();
    let err = create_validator().validate_path(&file).unwrap_err();
    assert!(
        err.to_string().contains("config.toml"),
        "error must name the offending file: {err}"
    );
}
