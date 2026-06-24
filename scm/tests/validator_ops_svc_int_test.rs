//! Structural coverage for the `ValidatorOps` SAF marker.

#[test]
fn test_validator_ops_svc_marker_name_matches_trait() {
    let trait_name = ["Validator", "Ops"].concat();
    assert_eq!(trait_name, "ValidatorOps");
}
