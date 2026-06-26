//! End-to-end tests for `PreflightReport`, `PreflightIssue`, and `PreflightIssueKind`.

use swe_edge_configbuilder::{
    ConfigLoaderFactory, PreflightIssue, PreflightIssueKind, PreflightReport,
};

// ── PreflightReport construction ──────────────────────────────────────────────

#[test]
fn test_preflight_report_new_has_no_issues() {
    let report = ConfigLoaderFactory::create_preflight_report();
    assert!(ConfigLoaderFactory::preflight_report_is_ok(&report));
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issue_count(&report),
        0
    );
    assert!(ConfigLoaderFactory::preflight_report_issues(&report).is_empty());
}

#[test]
fn test_preflight_report_default_matches_new() {
    let report = PreflightReport::default();
    assert!(ConfigLoaderFactory::preflight_report_is_ok(&report));
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issue_count(&report),
        0
    );
}

#[test]
fn test_preflight_report_push_increments_issue_count() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "cache".into(),
            kind: PreflightIssueKind::LoadError,
            message: "file not found".into(),
        },
    );
    assert!(!ConfigLoaderFactory::preflight_report_is_ok(&report));
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issue_count(&report),
        1
    );
}

#[test]
fn test_preflight_report_push_multiple_issues_all_accessible() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "a".into(),
            kind: PreflightIssueKind::LoadError,
            message: "io error".into(),
        },
    );
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "b".into(),
            kind: PreflightIssueKind::ValidationError,
            message: "invalid config".into(),
        },
    );
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "dep_graph".into(),
            kind: PreflightIssueKind::DependencyMissing,
            message: "requires 'c'".into(),
        },
    );
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issue_count(&report),
        3
    );
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issues(&report)[0].section,
        "a"
    );
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issues(&report)[1].section,
        "b"
    );
    assert_eq!(
        ConfigLoaderFactory::preflight_report_issues(&report)[2].section,
        "dep_graph"
    );
}

// ── PreflightIssueKind variants ───────────────────────────────────────────────

#[test]
fn test_preflight_issue_kind_load_error_distinct_from_others() {
    assert_ne!(
        PreflightIssueKind::LoadError,
        PreflightIssueKind::ValidationError
    );
    assert_ne!(
        PreflightIssueKind::LoadError,
        PreflightIssueKind::DependencyMissing
    );
    assert_ne!(
        PreflightIssueKind::LoadError,
        PreflightIssueKind::DependencyCycle
    );
}

#[test]
fn test_preflight_issue_kind_all_four_variants_are_clone_eq() {
    let kinds = [
        PreflightIssueKind::LoadError,
        PreflightIssueKind::ValidationError,
        PreflightIssueKind::DependencyMissing,
        PreflightIssueKind::DependencyCycle,
    ];
    for kind in &kinds {
        assert_eq!(kind.clone(), *kind);
    }
}

// ── PreflightIssue fields ─────────────────────────────────────────────────────

#[test]
fn test_preflight_issue_fields_are_publicly_constructible() {
    let issue = PreflightIssue {
        section: "broker".to_owned(),
        kind: PreflightIssueKind::ValidationError,
        message: "cert_path missing".to_owned(),
    };
    assert_eq!(issue.section, "broker");
    assert_eq!(issue.kind, PreflightIssueKind::ValidationError);
    assert_eq!(issue.message, "cert_path missing");
}

#[test]
fn test_preflight_issue_clone_preserves_all_fields() {
    let original = PreflightIssue {
        section: "analytics".to_owned(),
        kind: PreflightIssueKind::DependencyMissing,
        message: "requires 'cache'".to_owned(),
    };
    let cloned = original.clone();
    assert_eq!(cloned.section, original.section);
    assert_eq!(cloned.kind, original.kind);
    assert_eq!(cloned.message, original.message);
}

// ── Display ───────────────────────────────────────────────────────────────────

#[test]
fn test_preflight_report_display_ok_shows_ok() {
    let report = ConfigLoaderFactory::create_preflight_report();
    assert_eq!(report.to_string(), "preflight: OK");
}

#[test]
fn test_preflight_report_display_with_issues_shows_issue_count() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "cache".into(),
            kind: PreflightIssueKind::LoadError,
            message: "boom".into(),
        },
    );
    let output = report.to_string();
    assert!(output.contains("1 issue"), "got: {output}");
}

#[test]
fn test_preflight_report_display_includes_section_name_and_message() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "broker".into(),
            kind: PreflightIssueKind::ValidationError,
            message: "cert_path required".into(),
        },
    );
    let output = report.to_string();
    assert!(output.contains("broker"), "section name missing: {output}");
    assert!(
        output.contains("cert_path required"),
        "message missing: {output}"
    );
    assert!(output.contains("VALIDATION"), "kind tag missing: {output}");
}

#[test]
fn test_preflight_report_display_cycle_tag_appears() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    ConfigLoaderFactory::preflight_report_push(
        &mut report,
        PreflightIssue {
            section: "dependency_graph".into(),
            kind: PreflightIssueKind::DependencyCycle,
            message: "cycle detected".into(),
        },
    );
    assert!(report.to_string().contains("CYCLE"));
}
