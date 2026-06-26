//! @covers: api/types/loader/pattern_whitelist_policy.rs — PatternWhitelistPolicy
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::{ConfigLoaderFactory, SubstitutionPolicy};

#[test]
fn test_pattern_whitelist_policy_accepts_matching_variable() {
    // The policy must accept any variable whose name matches the configured regex.
    let policy =
        ConfigLoaderFactory::create_pattern_whitelist_policy("^APP_.*".to_string()).unwrap();
    assert!(
        policy.validate("APP_HOST").is_ok(),
        "APP_HOST matches ^APP_.* and must be accepted"
    );
}

#[test]
fn test_pattern_whitelist_policy_rejects_non_matching_variable() {
    // The policy must reject variables that don't match the regex — this is the
    // core security contract: only allow known patterns.
    let policy =
        ConfigLoaderFactory::create_pattern_whitelist_policy("^APP_.*".to_string()).unwrap();
    let result = policy.validate("DB_PASSWORD");
    assert!(
        result.is_err(),
        "DB_PASSWORD does not match ^APP_.* and must be rejected"
    );
}

#[test]
fn test_pattern_whitelist_policy_new_rejects_invalid_regex() {
    // An invalid regex must be caught at construction time, not at validation time,
    // so misconfigured policies fail fast on startup.
    let result = ConfigLoaderFactory::create_pattern_whitelist_policy("[invalid".to_string());
    assert!(
        result.is_err(),
        "invalid regex must be rejected at construction"
    );
}

#[test]
fn test_pattern_whitelist_policy_pattern_accessor_returns_configured_pattern() {
    // The description must include the configured pattern so callers can log it.
    let pat = "^SWE_.*".to_string();
    let policy = ConfigLoaderFactory::create_pattern_whitelist_policy(pat.clone()).unwrap();
    assert!(policy.description().contains(&pat));
}

#[test]
fn test_pattern_whitelist_policy_description_contains_pattern() {
    // The description is included in rejection error messages. It must contain
    // the pattern so operators know what rule was violated.
    let policy =
        ConfigLoaderFactory::create_pattern_whitelist_policy("^SWE_.*".to_string()).unwrap();
    assert!(
        policy.description().contains("^SWE_.*"),
        "description must include the pattern; got: {}",
        policy.description()
    );
}
