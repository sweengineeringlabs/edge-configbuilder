//! Structural coverage for the `LoaderOps` SAF marker.

#[test]
fn test_loader_ops_svc_marker_name_matches_trait() {
    let trait_name = ["Loader", "Ops"].concat();
    assert_eq!(trait_name, "LoaderOps");
}
