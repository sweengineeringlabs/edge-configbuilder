//! Integration tests for `FeatureRecordBuilderOps` — `new`, `enabled`, `override_source`.
#![allow(missing_docs)]
use swe_edge_configbuilder::{FeatureRecordBuilder, FeatureRecordBuilderOps as _, OverrideSource};

// ── new ───────────────────────────────────────────────────────────────────────

#[test]
fn test_new_sets_section_name_and_disabled_by_default_happy() {
    let r = FeatureRecordBuilder::new("broker").build();
    assert_eq!(r.section_name, "broker");
    assert!(!r.enabled, "new record must default to disabled");
}

#[test]
fn test_new_rejects_no_section_on_empty_string_passes_through_error() {
    // An empty section name is accepted structurally — downstream validation may
    // reject it. This test proves new() doesn't panic on empty input.
    let r = FeatureRecordBuilder::new("").build();
    assert_eq!(r.section_name, "");
}

#[test]
fn test_new_unicode_section_name_is_stored_verbatim_edge() {
    let r = FeatureRecordBuilder::new("配置_section").build();
    assert_eq!(r.section_name, "配置_section");
}

// ── enabled ───────────────────────────────────────────────────────────────────

#[test]
fn test_enabled_true_marks_feature_as_enabled_happy() {
    let r = FeatureRecordBuilder::new("cache").enabled(true).build();
    assert!(r.enabled);
}

#[test]
fn test_enabled_false_after_true_resets_to_disabled_error() {
    let r = FeatureRecordBuilder::new("cache").enabled(true).enabled(false).build();
    assert!(!r.enabled, "last enabled(false) must win");
}

#[test]
fn test_enabled_default_without_calling_enabled_is_false_edge() {
    let r = FeatureRecordBuilder::new("cache").build();
    assert!(!r.enabled);
}

// ── override_source ───────────────────────────────────────────────────────────

#[test]
fn test_override_source_env_var_is_stored_in_record_happy() {
    let r = FeatureRecordBuilder::new("cache")
        .override_source(OverrideSource::EnvVar {
            var_name: "CACHE_ENABLED".into(),
            value: "true".into(),
        })
        .build();
    match r.override_source {
        Some(OverrideSource::EnvVar { var_name, value }) => {
            assert_eq!(var_name, "CACHE_ENABLED");
            assert_eq!(value, "true");
        }
        other => panic!("expected EnvVar override source, got {other:?}"),
    }
}

#[test]
fn test_override_source_can_be_set_after_enabled_error() {
    // Regression: chaining override_source after enabled must not drop the flag.
    let r = FeatureRecordBuilder::new("cache")
        .enabled(true)
        .override_source(OverrideSource::EnvVar {
            var_name: "X".into(),
            value: "1".into(),
        })
        .build();
    assert!(r.enabled);
    assert!(r.override_source.is_some());
}

#[test]
fn test_override_source_without_setting_is_none_edge() {
    let r = FeatureRecordBuilder::new("cache").build();
    assert!(r.override_source.is_none());
}
