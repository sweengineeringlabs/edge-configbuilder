//! Tests for PrefixWhitelistPolicy substitution.
// @covers: api/types/loader/prefix_whitelist_policy.rs — PrefixWhitelistPolicy
use swe_edge_configbuilder::{ConfigLoaderFactory, PrefixWhitelistPolicy, SubstitutionPolicy};

#[test]
fn test_prefix_whitelist_policy_accepts_variable_with_matching_prefix() {
    // The policy must accept a variable whose name starts with any allowed prefix.
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
        "APP_".to_string(),
        "SWE_".to_string(),
    ]);
    assert_eq!(policy.validate("APP_HOST"), Ok(()));
}

#[test]
fn test_prefix_whitelist_policy_accepts_second_matching_prefix() {
    // OR semantics: any matching prefix is sufficient — not all must match.
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
        "APP_".to_string(),
        "SWE_".to_string(),
    ]);
    assert_eq!(policy.validate("SWE_TOKEN"), Ok(()));
}

#[test]
fn test_prefix_whitelist_policy_rejects_variable_with_no_matching_prefix() {
    // Variables that share no prefix with the allowlist must be rejected —
    // this prevents leaking secrets via unintended env var substitution.
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["APP_".to_string()]);
    let result = policy.validate("DB_PASSWORD");
    assert!(
        result.is_err(),
        "DB_PASSWORD does not start with APP_ and must be rejected"
    );
}

#[test]
fn test_prefix_whitelist_policy_rejects_partial_prefix_match() {
    // "APP" must not match "APP_HOST" — the prefix "APP" without the underscore
    // is a different allowlist entry. Test the exact boundary.
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["APPX_".to_string()]);
    let result = policy.validate("APP_HOST");
    assert!(
        result.is_err(),
        "APP_HOST does not start with APPX_ and must be rejected"
    );
}

#[test]
fn test_prefix_whitelist_policy_prefixes_accessor_returns_configured_prefixes() {
    // The description must reflect the configured prefixes.
    let prefixes = vec!["X_".to_string(), "Y_".to_string()];
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(prefixes.clone());
    for prefix in prefixes {
        assert!(policy.description().contains(&prefix));
    }
}

#[test]
fn test_prefix_whitelist_policy_description_contains_prefix() {
    // The description is embedded in rejection error messages so operators
    // know which policy blocked the variable.
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["SWE_".to_string()]);
    assert!(
        policy.description().contains("SWE_"),
        "description must include the allowed prefix; got: {}",
        policy.description()
    );
}
