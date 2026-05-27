//! Tests for `OnError` — graceful-degradation policy for optional config sections.

use swe_edge_configbuilder::OnError;

#[test]
fn test_on_error_default_is_fail() {
    let policy = OnError::default();
    assert_eq!(policy, OnError::Fail);
}

#[test]
fn test_on_error_fail_and_disable_are_distinct() {
    assert_ne!(OnError::Fail, OnError::Disable);
}

#[test]
fn test_on_error_fail_variant_matches() {
    let policy = OnError::Fail;
    assert!(matches!(policy, OnError::Fail));
}

#[test]
fn test_on_error_disable_variant_matches() {
    let policy = OnError::Disable;
    assert!(matches!(policy, OnError::Disable));
}

#[test]
fn test_on_error_copy_semantics_do_not_require_clone() {
    let policy = OnError::Disable;
    let copy = policy;
    assert_eq!(copy, OnError::Disable);
    // original is still usable because OnError is Copy
    assert_eq!(policy, OnError::Disable);
}

#[test]
fn test_on_error_debug_format_is_non_empty() {
    assert!(!format!("{:?}", OnError::Fail).is_empty());
    assert!(!format!("{:?}", OnError::Disable).is_empty());
}
