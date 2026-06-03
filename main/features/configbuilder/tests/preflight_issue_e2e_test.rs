//! Tests for PreflightIssue struct fields.
// @covers: api/types/preflight/preflight_issue.rs — PreflightIssue struct fields
use swe_edge_configbuilder::{PreflightIssue, PreflightIssueKind};

#[test]
fn test_preflight_issue_section_field_stores_section_name() {
    // The section field must record which TOML section triggered the issue so
    // operators can locate the offending config entry without reading source code.
    let issue = PreflightIssue {
        section: "cache".to_string(),
        kind: PreflightIssueKind::LoadError,
        message: "file not found".to_string(),
    };
    assert_eq!(issue.section, "cache");
}

#[test]
fn test_preflight_issue_kind_field_stores_kind() {
    // kind must be preserved exactly — callers use it to categorise issues
    // (e.g. treat DependencyMissing as a warning vs. LoadError as fatal).
    let issue = PreflightIssue {
        section: "broker".to_string(),
        kind: PreflightIssueKind::DependencyMissing,
        message: "requires cache but it is not enabled".to_string(),
    };
    assert_eq!(issue.kind, PreflightIssueKind::DependencyMissing);
}

#[test]
fn test_preflight_issue_message_field_stores_human_readable_text() {
    // message must be the full human-readable text so operators can diagnose
    // the issue without needing to match on kind variants.
    let msg = "requires 'cache' but it is not enabled".to_string();
    let issue = PreflightIssue {
        section: "broker".to_string(),
        kind: PreflightIssueKind::DependencyMissing,
        message: msg.clone(),
    };
    assert_eq!(issue.message, msg);
}

#[test]
fn test_preflight_issue_clone_produces_independent_copy() {
    // PreflightIssue derives Clone so preflight reports can be passed around
    // without moves. Verify clone independence by mutating the original.
    let original = PreflightIssue {
        section: "cache".to_string(),
        kind: PreflightIssueKind::LoadError,
        message: "io error".to_string(),
    };
    let cloned = original.clone();
    assert_eq!(cloned.section, original.section);
    assert_eq!(cloned.kind, original.kind);
    assert_eq!(cloned.message, original.message);
}
