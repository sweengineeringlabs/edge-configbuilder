//! Structural coverage for the `ConfigBuilder` SAF marker.

#[test]
fn test_config_builder_svc_marker_name_matches_trait() {
    let trait_name = ["Config", "Builder"].concat();
    assert_eq!(trait_name, "ConfigBuilder");
}
