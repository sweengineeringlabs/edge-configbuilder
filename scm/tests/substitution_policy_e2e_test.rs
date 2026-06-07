//! Tests for substitution policy implementations.
#![cfg(feature = "test-utils")]
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::{
    AllowAllPolicy, CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy,
    SubstitutionPolicy,
};

#[test]
fn test_allow_all_policy_accepts_any_variable() {
    let policy = AllowAllPolicy;
    assert!(policy.validate("ANY_VAR").is_ok());
    assert!(policy.validate("_LEADING_UNDERSCORE").is_ok());
    assert!(policy.validate("123NUMBERS").is_ok());
}

#[test]
fn test_prefix_whitelist_policy_accepts_matching_prefix() {
    let policy = PrefixWhitelistPolicy::new(vec!["APP_".into(), "SERVICE_".into()]);

    assert!(policy.validate("APP_DEBUG").is_ok());
    assert!(policy.validate("SERVICE_URL").is_ok());
    assert!(policy.validate("SERVICE_PORT").is_ok());
}

#[test]
fn test_prefix_whitelist_policy_rejects_non_matching_prefix() {
    let policy = PrefixWhitelistPolicy::new(vec!["APP_".into()]);

    let result = policy.validate("DATABASE_PASSWORD");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("does not match any allowed prefix"));
}

#[test]
fn test_pattern_whitelist_policy_accepts_matching_pattern() {
    let policy =
        PatternWhitelistPolicy::new("^(APP|SERVICE)_[A-Z_]+$".into()).expect("valid regex");

    assert!(policy.validate("APP_DEBUG").is_ok());
    assert!(policy.validate("SERVICE_URL").is_ok());
}

#[test]
fn test_pattern_whitelist_policy_rejects_non_matching_pattern() {
    let policy = PatternWhitelistPolicy::new("^APP_[A-Z_]+$".into()).expect("valid regex");

    // Lowercase doesn't match
    let result = policy.validate("app_debug");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not match pattern"));
}

#[test]
fn test_pattern_whitelist_policy_rejects_invalid_regex() {
    let result = PatternWhitelistPolicy::new("[invalid(regex".into());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid regex pattern"));
}

#[test]
fn test_composite_policy_requires_all_policies_to_pass() {
    let policies: Vec<Box<dyn SubstitutionPolicy>> = vec![
        Box::new(PrefixWhitelistPolicy::new(vec!["APP_".into()])),
        Box::new(PatternWhitelistPolicy::new("^APP_[A-Z_]+$".into()).unwrap()),
    ];
    let policy = CompositePolicy::new(policies);

    // Must match both policies
    assert!(policy.validate("APP_DEBUG").is_ok());

    // Matches prefix but not pattern (lowercase)
    assert!(policy.validate("app_debug").is_err());

    // Matches pattern but not prefix
    assert!(policy.validate("SERVICE_DEBUG").is_err());
}

#[test]
fn test_policy_description() {
    let allow_all = AllowAllPolicy;
    assert!(allow_all.description().contains("AllowAll"));

    let prefix = PrefixWhitelistPolicy::new(vec!["APP_".into(), "SERVICE_".into()]);
    assert!(prefix.description().contains("APP_"));
    assert!(prefix.description().contains("SERVICE_"));

    let pattern = PatternWhitelistPolicy::new("^APP_.*".into()).unwrap();
    assert!(pattern.description().contains("APP_"));
}
