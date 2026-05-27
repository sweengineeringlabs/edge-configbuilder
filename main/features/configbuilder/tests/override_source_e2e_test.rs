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
