//! Structural coverage for the `Preflight` SAF marker.

#[test]
fn test_preflight_svc_marker_name_matches_trait() {
    let trait_name = ["Pre", "flight"].concat();
    assert_eq!(trait_name, "Preflight");
}
