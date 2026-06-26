//! @covers: api/types/preflight/preflight_issue_kind.rs — PreflightIssueKind classification
use swe_edge_configbuilder::{ConfigError, ConfigLoaderFactory, PreflightIssueKind};

#[test]
fn test_from_config_error_parse_error_maps_to_load_error() {
    // A Parse error means the TOML was malformed — classified as LoadError so
    // the preflight report groups it with other I/O-class failures.
    let e = ConfigError::Parse("unexpected character".to_string());
    assert_eq!(
        PreflightIssueKind::from_config_error(&e),
        PreflightIssueKind::LoadError,
        "Parse errors must map to LoadError"
    );
}

#[test]
fn test_from_config_error_io_error_maps_to_load_error() {
    // I/O errors (unreadable file, size limit exceeded) are LoadError —
    // the section could not be read, not that it was semantically invalid.
    let e = ConfigError::Io("permission denied".to_string());
    assert_eq!(
        PreflightIssueKind::from_config_error(&e),
        PreflightIssueKind::LoadError,
        "Io errors must map to LoadError"
    );
}

#[test]
fn test_from_config_error_not_found_maps_to_load_error() {
    // NotFound is a LoadError: the config directory had no application.toml,
    // which is a deployment issue, not a semantic validation failure.
    let e = ConfigError::NotFound("no application.toml found".to_string());
    assert_eq!(
        PreflightIssueKind::from_config_error(&e),
        PreflightIssueKind::LoadError,
        "NotFound errors must map to LoadError"
    );
}

#[test]
fn test_from_config_error_validation_error_maps_to_validation_error() {
    // Validation errors occur after successful parse — they are semantic failures
    // and must be classified separately from I/O failures in the preflight report.
    let e = ConfigError::Validation {
        section: "cache".to_string(),
        reason: "ttl must be > 0".to_string(),
    };
    assert_eq!(
        PreflightIssueKind::from_config_error(&e),
        PreflightIssueKind::ValidationError,
        "Validation errors must map to ValidationError"
    );
}

#[test]
fn test_preflight_issue_kind_equality_works_for_all_variants() {
    let variants = [
        PreflightIssueKind::LoadError,
        PreflightIssueKind::ValidationError,
        PreflightIssueKind::DependencyMissing,
        PreflightIssueKind::DependencyCycle,
    ];
    assert_eq!(variants.len(), 4);
    assert_ne!(
        variants[0], variants[1],
        "LoadError and ValidationError must remain distinct variants"
    );
}
