use swe_edge_configbuilder::ConfigError;

#[test]
fn test_substitution_error_variable_not_found_displays() {
    let err = ConfigError::Io("test: variable MISSING_VAR not found".to_string());
    let msg = err.to_string();
    assert!(msg.contains("MISSING_VAR"));
}

#[test]
fn test_substitution_error_variable_rejected_displays() {
    let err = ConfigError::Io(
        "test: variable PRIVATE_KEY rejected: does not match any allowed prefix".to_string(),
    );
    let msg = err.to_string();
    assert!(msg.contains("PRIVATE_KEY"));
    assert!(msg.contains("rejected"));
}

#[test]
fn test_substitution_error_invalid_syntax_displays() {
    let err = ConfigError::Io(
        "test: nested placeholders (e.g., {{VAR_{{OTHER}}}}) are not supported".to_string(),
    );
    let msg = err.to_string();
    assert!(msg.contains("nested"));
}
