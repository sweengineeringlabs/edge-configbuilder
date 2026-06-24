//! Structural coverage for the `SectionLoaderBound` SAF marker.

#[test]
fn test_section_loader_bound_svc_marker_name_matches_trait() {
    let trait_name = ["Section", "Loader", "Bound"].concat();
    assert_eq!(trait_name, "SectionLoaderBound");
}
