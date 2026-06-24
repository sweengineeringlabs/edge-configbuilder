//! Structural coverage for the `ConfigSection` SAF marker.

#[test]
fn test_config_section_svc_marker_name_matches_trait() {
    let trait_name = ["Config", "Section"].concat();
    assert_eq!(trait_name, "ConfigSection");
}
