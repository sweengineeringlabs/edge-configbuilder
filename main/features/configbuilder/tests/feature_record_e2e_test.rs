//! Tests for `FeatureRecord` — startup feature state snapshot.

use swe_edge_configbuilder::{FeatureRecord, OverrideSource};

#[test]
fn test_feature_record_enabled_section_without_override() {
    let record = FeatureRecord {
        section_name: "cache".to_owned(),
        enabled: true,
        override_source: None,
    };
    assert_eq!(record.section_name, "cache");
    assert!(record.enabled);
    assert!(record.override_source.is_none());
}

#[test]
fn test_feature_record_disabled_section_without_override() {
    let record = FeatureRecord {
        section_name: "message_broker".to_owned(),
        enabled: false,
        override_source: None,
    };
    assert!(!record.enabled);
    assert!(record.override_source.is_none());
}

#[test]
fn test_feature_record_disabled_by_explicit_toml_flag() {
    let record = FeatureRecord {
        section_name: "analytics".to_owned(),
        enabled: false,
        override_source: Some(OverrideSource::ExplicitTomlFlag),
    };
    assert!(!record.enabled);
    assert!(matches!(
        record.override_source,
        Some(OverrideSource::ExplicitTomlFlag)
    ));
}

#[test]
fn test_feature_record_overridden_by_env_var() {
    let record = FeatureRecord {
        section_name: "tracing".to_owned(),
        enabled: false,
        override_source: Some(OverrideSource::EnvVar {
            var_name: "SWE_EDGE_FEATURE_TRACING".to_owned(),
            value: "false".to_owned(),
        }),
    };
    assert!(matches!(
        record.override_source,
        Some(OverrideSource::EnvVar { .. })
    ));
}

#[test]
fn test_feature_record_clone_produces_equal_record() {
    let original = FeatureRecord {
        section_name: "auth".to_owned(),
        enabled: true,
        override_source: None,
    };
    let cloned = original.clone();
    assert_eq!(original.section_name, cloned.section_name);
    assert_eq!(original.enabled, cloned.enabled);
}
