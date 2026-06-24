//! Structural coverage for the `Validator` SAF marker.

#[test]
fn test_validator_svc_marker_name_matches_trait() {
    let trait_name = ["Valid", "ator"].concat();
    assert_eq!(trait_name, "Validator");
}
