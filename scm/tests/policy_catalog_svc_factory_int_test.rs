#![allow(missing_docs)]
use swe_edge_configbuilder::POLICY_CATALOG_SVC_FACTORY;

#[test]
fn test_policy_catalog_svc_factory_has_constant() {
    assert_eq!(POLICY_CATALOG_SVC_FACTORY, "PolicyCatalog");
}
