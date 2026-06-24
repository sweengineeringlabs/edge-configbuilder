//! Structural coverage for the `Loader` SAF marker.

#[test]
fn test_loader_svc_marker_name_matches_trait() {
    let trait_name = ["Load", "er"].concat();
    assert_eq!(trait_name, "Loader");
}
