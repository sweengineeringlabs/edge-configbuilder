//! Integration tests for `PreflightIssueKindOps` — `variant_name`.
#![allow(missing_docs)]
use swe_edge_configbuilder::{PreflightIssueKind, PreflightIssueKindOps as _};

// ── variant_name ──────────────────────────────────────────────────────────────

#[test]
fn test_variant_name_load_error_returns_expected_string_happy() {
    assert_eq!(PreflightIssueKind::LoadError.variant_name(), "LoadError");
}

#[test]
fn test_variant_name_validation_error_not_same_as_load_error_error() {
    // Catches the bug where two match arms return the same string.
    let load = PreflightIssueKind::LoadError.variant_name();
    let val = PreflightIssueKind::ValidationError.variant_name();
    assert_ne!(load, val, "LoadError and ValidationError must have distinct names");
}

#[test]
fn test_variant_name_all_four_variants_are_unique_edge() {
    let names = [
        PreflightIssueKind::LoadError.variant_name(),
        PreflightIssueKind::ValidationError.variant_name(),
        PreflightIssueKind::DependencyMissing.variant_name(),
        PreflightIssueKind::DependencyCycle.variant_name(),
    ];
    let unique: std::collections::HashSet<_> = names.iter().collect();
    assert_eq!(unique.len(), 4, "all variant names must be distinct: {names:?}");
}
