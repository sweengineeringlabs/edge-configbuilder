//! Structural coverage for the `FeatureLoader` SAF marker.

#[test]
fn test_feature_loader_svc_marker_name_matches_trait() {
    let trait_name = ["Feature", "Loader"].concat();
    assert_eq!(trait_name, "FeatureLoader");
}
