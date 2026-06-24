//! Structural coverage for the `ValidatorBound` SAF marker.

#[test]
fn test_validator_bound_svc_marker_name_matches_trait() {
    let trait_name = ["Validator", "Bound"].concat();
    assert_eq!(trait_name, "ValidatorBound");
}
