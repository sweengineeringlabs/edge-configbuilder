//! Tests for `OverrideSource` — records which control overrode natural TOML state.

use swe_edge_configbuilder::OverrideSource;

#[test]
fn test_override_source_explicit_toml_flag_variant_is_constructible() {
    let src = OverrideSource::ExplicitTomlFlag;
    assert!(matches!(src, OverrideSource::ExplicitTomlFlag));
}

#[test]
fn test_override_source_env_var_variant_stores_name_and_value() {
    let src = OverrideSource::EnvVar {
        var_name: "SWE_EDGE_FEATURE_CACHE".to_owned(),
        value: "false".to_owned(),
    };
    let OverrideSource::EnvVar { var_name, value } = &src else {
        panic!("expected EnvVar variant");
    };
    assert_eq!(var_name, "SWE_EDGE_FEATURE_CACHE");
    assert_eq!(value, "false");
}

#[test]
fn test_override_source_env_var_clone_preserves_contents() {
    let original = OverrideSource::EnvVar {
        var_name: "SWE_EDGE_FEATURE_X".to_owned(),
        value: "true".to_owned(),
    };
    let cloned = original.clone();
    let OverrideSource::EnvVar { var_name, value } = cloned else {
        panic!("expected EnvVar variant after clone");
    };
    assert_eq!(var_name, "SWE_EDGE_FEATURE_X");
    assert_eq!(value, "true");
}

#[test]
fn test_override_source_explicit_toml_flag_clone_is_same_variant() {
    let original = OverrideSource::ExplicitTomlFlag;
    let cloned = original.clone();
    assert!(matches!(cloned, OverrideSource::ExplicitTomlFlag));
}

#[test]
fn test_override_source_debug_format_is_non_empty() {
    let src = OverrideSource::ExplicitTomlFlag;
    assert!(!format!("{src:?}").is_empty());
}

#[test]
fn test_override_source_validation_error_variant_stores_reason() {
    let src = OverrideSource::ValidationError {
        reason: "cert_path required when tls_enabled = true".to_owned(),
    };
    let OverrideSource::ValidationError { reason } = &src else {
        panic!("expected ValidationError variant");
    };
    assert!(reason.contains("cert_path"));
}

#[test]
fn test_override_source_validation_error_clone_preserves_reason() {
    let original = OverrideSource::ValidationError {
        reason: "port out of range".to_owned(),
    };
    let cloned = original.clone();
    let OverrideSource::ValidationError { reason } = cloned else {
        panic!("expected ValidationError variant after clone");
    };
    assert_eq!(reason, "port out of range");
}

#[test]
fn test_override_source_validation_error_debug_contains_reason() {
    let src = OverrideSource::ValidationError {
        reason: "missing required field".to_owned(),
    };
    let debug = format!("{src:?}");
    assert!(
        debug.contains("missing required field"),
        "debug output must include the reason, got: {debug}"
    );
}
