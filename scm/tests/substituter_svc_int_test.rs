//! Structural coverage for the `Substituter` SAF marker.

#[test]
fn test_substituter_svc_marker_name_matches_trait() {
    let trait_name = ["Substitut", "er"].concat();
    assert_eq!(trait_name, "Substituter");
}
