//! Tests for `ValidatorError` display and public conversion.

use swe_edge_configbuilder::{ConfigError, ValidatorError};

#[test]
fn test_validator_error_display_io() {
    let err = ValidatorError::Io("not a directory".to_owned());

    assert!(err.to_string().contains("io error"));
    assert!(err.to_string().contains("not a directory"));
}

#[test]
fn test_validator_error_converts_to_config_error() {
    let err = ValidatorError::Io("bad path".to_owned());
    let config_error = ConfigError::from(err);

    assert!(matches!(config_error, ConfigError::Io(_)));
    assert!(config_error.to_string().contains("bad path"));
}
