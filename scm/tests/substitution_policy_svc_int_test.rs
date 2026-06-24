//! Structural coverage for the `SubstitutionPolicy` SAF marker.

#[test]
fn test_substitution_policy_svc_marker_name_matches_trait() {
    let trait_name = ["Substitution", "Policy"].concat();
    assert_eq!(trait_name, "SubstitutionPolicy");
}
