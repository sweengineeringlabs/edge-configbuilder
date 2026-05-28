// @covers: api/types/loader/allow_all_policy.rs — AllowAllPolicy accepts any var
use swe_edge_configbuilder::{AllowAllPolicy, SubstitutionPolicy};

#[test]
fn test_allow_all_policy_accepts_any_variable_name() {
    // AllowAllPolicy must never reject any variable. If it did, callers using it
    // in development would see unexpected rejections.
    let policy = AllowAllPolicy;
    assert!(
        policy.validate("ANY_VAR").is_ok(),
        "AllowAllPolicy must accept ANY_VAR"
    );
}

#[test]
fn test_allow_all_policy_accepts_empty_string() {
    // The policy must not special-case the empty string — it accepts everything.
    let policy = AllowAllPolicy;
    assert!(
        policy.validate("").is_ok(),
        "AllowAllPolicy must accept empty variable name"
    );
}

#[test]
fn test_allow_all_policy_accepts_lowercase_var() {
    let policy = AllowAllPolicy;
    assert!(
        policy.validate("lowercase_var").is_ok(),
        "AllowAllPolicy must accept lowercase names"
    );
}

#[test]
fn test_allow_all_policy_accepts_var_with_digits() {
    let policy = AllowAllPolicy;
    assert!(
        policy.validate("VAR_123").is_ok(),
        "AllowAllPolicy must accept names with digits"
    );
}

#[test]
fn test_allow_all_policy_description_is_non_empty() {
    // description() is used in error messages when another policy rejects a var
    // inside a CompositePolicy. It must not be empty so the message is useful.
    let policy = AllowAllPolicy;
    assert!(
        !policy.description().is_empty(),
        "AllowAllPolicy description must not be empty"
    );
}
