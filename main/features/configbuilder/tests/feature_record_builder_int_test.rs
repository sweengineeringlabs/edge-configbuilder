//! Integration tests for [`FeatureRecordBuilder`].

use swe_edge_configbuilder::{FeatureMetadata, FeatureRecordBuilder, OverrideSource};

/// @covers: new
#[test]
fn test_new_sets_section_name_and_defaults() {
    let r = FeatureRecordBuilder::new("cache").build();
    assert_eq!(r.section_name, "cache");
    assert!(!r.enabled);
    assert!(r.override_source.is_none());
    assert!(r.requires.is_empty());
}

/// @covers: enabled
#[test]
fn test_enabled_sets_flag() {
    let r = FeatureRecordBuilder::new("cache").enabled(true).build();
    assert!(r.enabled);
}

/// @covers: override_source
#[test]
fn test_override_source_sets_field() {
    let r = FeatureRecordBuilder::new("cache")
        .override_source(OverrideSource::EnvVar {
            var: "CACHE_ENABLED".into(),
            value: "false".into(),
        })
        .build();
    assert!(r.override_source.is_some());
}

/// @covers: requires
#[test]
fn test_requires_sets_dependency_slice() {
    let r = FeatureRecordBuilder::new("analytics")
        .requires(&["cache", "broker"])
        .build();
    assert_eq!(r.requires, &["cache", "broker"]);
}

/// @covers: metadata
#[test]
fn test_metadata_sets_description() {
    let m = FeatureMetadata {
        description: "caching layer",
        owner: "platform",
        deprecated_since: None,
    };
    let r = FeatureRecordBuilder::new("cache").metadata(m).build();
    assert_eq!(r.metadata.description, "caching layer");
}

/// @covers: build
#[test]
fn test_build_produces_record_with_all_fields() {
    let r = FeatureRecordBuilder::new("broker")
        .enabled(true)
        .requires(&["cache"])
        .build();
    assert_eq!(r.section_name, "broker");
    assert!(r.enabled);
    assert_eq!(r.requires, &["cache"]);
    assert!(r.override_source.is_none());
}
