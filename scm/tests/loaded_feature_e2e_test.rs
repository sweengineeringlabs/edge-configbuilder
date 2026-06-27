//! Tests for `LoadedFeature<T>` — full result of loading an optional config section.

use swe_edge_configbuilder::{FeatureMetadata, FeatureRecord, FeatureState, FeatureStateOps as _, LoadedFeature};

#[test]
fn test_loaded_feature_enabled_state_and_record_accessible() {
    let loaded = LoadedFeature {
        state: FeatureState::Enabled(42u32),
        record: Box::new(FeatureRecord {
            section_name: "my_section".to_owned(),
            enabled: true,
            override_source: None,
            requires: &[],
            metadata: Box::new(FeatureMetadata::default()),
        }),
    };
    assert!(loaded.state.is_enabled());
    assert_eq!(loaded.state.into_option(), Some(42u32));
}

#[test]
fn test_loaded_feature_record_section_name_matches_state() {
    let loaded = LoadedFeature {
        state: FeatureState::<u32>::Disabled,
        record: Box::new(FeatureRecord {
            section_name: "feature_x".to_owned(),
            enabled: false,
            override_source: None,
            requires: &[],
            metadata: Box::new(FeatureMetadata::default()),
        }),
    };
    assert!(loaded.state.is_disabled());
    assert_eq!(loaded.record.section_name, "feature_x");
    assert!(!loaded.record.enabled);
}

#[test]
fn test_loaded_feature_enabled_record_flag_matches_state() {
    let loaded = LoadedFeature {
        state: FeatureState::Enabled("on".to_owned()),
        record: Box::new(FeatureRecord {
            section_name: "broker".to_owned(),
            enabled: true,
            override_source: None,
            requires: &[],
            metadata: Box::new(FeatureMetadata::default()),
        }),
    };
    // The record.enabled field must agree with the state variant.
    assert_eq!(loaded.state.is_enabled(), loaded.record.enabled);
}
