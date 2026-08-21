#![allow(missing_docs)]
use configbuilder::FEATURE_REGISTRY_OPS_SVC_FACTORY;

#[test]
fn test_feature_registry_ops_svc_factory_constant_matches_trait_name() {
    assert_eq!(FEATURE_REGISTRY_OPS_SVC_FACTORY, "FeatureRegistryOps");
}
