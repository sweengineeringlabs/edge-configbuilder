//! Tests for `FeatureMetadata` — static annotations attached to an optional section.

use swe_edge_configbuilder::FeatureMetadata;

#[test]
fn test_feature_metadata_default_has_empty_description_and_owner() {
    let meta = FeatureMetadata::default();
    assert_eq!(meta.description, "");
    assert_eq!(meta.owner, "");
    assert!(meta.deprecated_since.is_none());
}

#[test]
fn test_feature_metadata_constructed_with_values_is_accessible() {
    let meta = FeatureMetadata {
        description: "Redis-backed response cache",
        owner: "platform-team",
        deprecated_since: None,
    };
    assert_eq!(meta.description, "Redis-backed response cache");
    assert_eq!(meta.owner, "platform-team");
    assert!(meta.deprecated_since.is_none());
}

#[test]
fn test_feature_metadata_deprecated_since_stores_version() {
    let meta = FeatureMetadata {
        description: "Legacy auth module",
        owner: "security-team",
        deprecated_since: Some("2.0.0"),
    };
    assert_eq!(meta.deprecated_since, Some("2.0.0"));
}

#[test]
fn test_feature_metadata_copy_semantics_do_not_require_clone() {
    let meta = FeatureMetadata {
        description: "tracing pipeline",
        owner: "infra",
        deprecated_since: None,
    };
    // Copy: use meta after assigning to another binding
    let copy = meta;
    assert_eq!(copy.description, meta.description);
    assert_eq!(copy.owner, meta.owner);
}

#[test]
fn test_feature_metadata_clone_produces_equal_value() {
    let original = FeatureMetadata {
        description: "queue processor",
        owner: "data-team",
        deprecated_since: Some("1.5.0"),
    };
    let cloned = original;
    assert_eq!(cloned.description, original.description);
    assert_eq!(cloned.deprecated_since, original.deprecated_since);
}
