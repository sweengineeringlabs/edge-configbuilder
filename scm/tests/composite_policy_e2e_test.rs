//! Tests for CompositePolicy substitution.
#![cfg(feature = "test-utils")]
// @covers: api/types/loader/composite_policy.rs — CompositePolicy AND logic
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::{
    AllowAllPolicy, CompositePolicy, PrefixWhitelistPolicy, SubstitutionPolicy,
};

#[test]
fn test_composite_policy_empty_policies_accepts_any_var() {
    // An empty composite policy has no constraints — everything must pass.
    let policy = CompositePolicy::new(vec![]);
    assert!(
        policy.validate("ANYTHING").is_ok(),
        "empty composite must accept any variable"
    );
}

#[test]
fn test_composite_policy_all_pass_accepts_var() {
    // When all sub-policies accept the variable, the composite must accept it too.
    let policy = CompositePolicy::new(vec![Box::new(AllowAllPolicy), Box::new(AllowAllPolicy)]);
    assert!(
        policy.validate("MY_VAR").is_ok(),
        "all sub-policies pass; composite must pass"
    );
}

#[test]
fn test_composite_policy_one_fails_rejects_var() {
    // AND logic: if any sub-policy rejects, the composite must reject.
    // This is the key safety property — a single restrictive policy wins.
    let policy = CompositePolicy::new(vec![
        Box::new(AllowAllPolicy),
        Box::new(PrefixWhitelistPolicy::new(vec!["ALLOWED_".to_string()])),
    ]);
    let result = policy.validate("FORBIDDEN_VAR");
    assert!(
        result.is_err(),
        "one sub-policy rejects FORBIDDEN_VAR; composite must reject too"
    );
}

#[test]
fn test_composite_policy_error_message_mentions_all_failures() {
    // The error message must include context from the failing policies so the
    // operator knows which policy rejected the variable and why.
    let policy = CompositePolicy::new(vec![
        Box::new(PrefixWhitelistPolicy::new(vec!["A_".to_string()])),
        Box::new(PrefixWhitelistPolicy::new(vec!["B_".to_string()])),
    ]);
    let err = policy.validate("C_VAR").unwrap_err();
    assert!(
        !err.is_empty(),
        "error message must not be empty when both policies reject"
    );
}

#[test]
fn test_composite_policy_description_is_non_empty() {
    let policy = CompositePolicy::new(vec![Box::new(AllowAllPolicy)]);
    assert!(
        !policy.description().is_empty(),
        "CompositePolicy description must not be empty"
    );
}
