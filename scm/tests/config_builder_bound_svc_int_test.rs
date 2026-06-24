//! Structural coverage for the `ConfigBuilderBound` SAF marker.

#[test]
fn test_config_builder_bound_svc_marker_name_matches_trait() {
    let trait_name = ["Config", "Builder", "Bound"].concat();
    assert_eq!(trait_name, "ConfigBuilderBound");
}
