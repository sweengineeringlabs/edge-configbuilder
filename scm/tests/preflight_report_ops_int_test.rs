//! Integration tests for `PreflightReportOps` — `is_ok`, `issues`, `issue_count`.
#![allow(missing_docs)]
use swe_edge_configbuilder::{
    ConfigLoaderFactory, PreflightIssue, PreflightIssueKind, PreflightReport, PreflightReportOps as _,
};

fn push_load_error(report: &mut PreflightReport) {
    ConfigLoaderFactory::preflight_report_push(
        report,
        PreflightIssue {
            section: "db".into(),
            kind: PreflightIssueKind::LoadError,
            message: "file missing".into(),
        },
    );
}

// ── is_ok ─────────────────────────────────────────────────────────────────────

#[test]
fn test_is_ok_returns_true_when_no_issues_happy() {
    let report = ConfigLoaderFactory::create_preflight_report();
    assert!(report.is_ok(), "fresh report must be ok");
    assert_eq!(report.issue_count(), 0, "is_ok must imply zero issues");
}

#[test]
fn test_is_ok_returns_false_after_push_error() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    push_load_error(&mut report);
    assert!(!report.is_ok(), "report with a pushed issue must not be ok");
    assert_eq!(report.issue_count(), 1);
}

#[test]
fn test_is_ok_resets_to_false_after_multiple_pushes_edge() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    for _ in 0..5 {
        push_load_error(&mut report);
    }
    assert!(!report.is_ok());
    assert_eq!(report.issue_count(), 5, "must track all pushed issues");
}

// ── issues ────────────────────────────────────────────────────────────────────

#[test]
fn test_issues_returns_empty_slice_on_fresh_report_happy() {
    let report = ConfigLoaderFactory::create_preflight_report();
    assert_eq!(report.issues().len(), 0, "fresh report must have no issues");
}

#[test]
fn test_issues_contains_pushed_issue_with_correct_section_error() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    push_load_error(&mut report);
    assert_eq!(report.issues().len(), 1);
    assert_eq!(report.issues()[0].section, "db", "issue section must match pushed value");
}

#[test]
fn test_issues_preserves_insertion_order_edge() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    for name in ["a", "b", "c"] {
        ConfigLoaderFactory::preflight_report_push(
            &mut report,
            PreflightIssue {
                section: name.into(),
                kind: PreflightIssueKind::LoadError,
                message: String::new(),
            },
        );
    }
    let sections: Vec<&str> = report.issues().iter().map(|i| i.section.as_str()).collect();
    assert_eq!(sections, ["a", "b", "c"], "issues must be stored in insertion order");
}

// ── issue_count ───────────────────────────────────────────────────────────────

#[test]
fn test_issue_count_equals_number_of_pushed_issues_happy() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    for _ in 0..3 {
        push_load_error(&mut report);
    }
    assert_eq!(report.issue_count(), 3);
}

#[test]
fn test_issue_count_increments_with_each_push_error() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    assert_eq!(report.issue_count(), 0);
    push_load_error(&mut report);
    assert_eq!(report.issue_count(), 1, "count must increase after push");
    push_load_error(&mut report);
    assert_eq!(report.issue_count(), 2, "count must increase after second push");
}

#[test]
fn test_issue_count_matches_issues_slice_length_edge() {
    let mut report = ConfigLoaderFactory::create_preflight_report();
    for _ in 0..5 {
        push_load_error(&mut report);
    }
    assert_eq!(
        report.issue_count(),
        report.issues().len(),
        "issue_count and issues().len() must always agree"
    );
}
