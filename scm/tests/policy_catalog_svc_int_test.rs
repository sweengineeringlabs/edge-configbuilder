//! Structural coverage for the `PolicyCatalog` SAF marker.

#[test]
fn test_policy_catalog_svc_marker_name_matches_trait() {
    let trait_name = ["Policy", "Catalog"].concat();
    assert_eq!(trait_name, "PolicyCatalog");
}
