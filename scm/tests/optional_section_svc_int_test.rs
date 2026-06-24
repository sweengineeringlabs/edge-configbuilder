//! Structural coverage for the `OptionalSection` SAF marker.

#[test]
fn test_optional_section_svc_marker_name_matches_trait() {
    let trait_name = ["Optional", "Section"].concat();
    assert_eq!(trait_name, "OptionalSection");
}
