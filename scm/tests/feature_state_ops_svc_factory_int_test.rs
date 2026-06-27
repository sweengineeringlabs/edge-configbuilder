#![allow(missing_docs)]
use swe_edge_configbuilder::FEATURE_STATE_OPS_SVC_FACTORY;

#[test]
fn test_feature_state_ops_svc_factory_constant_matches_trait_name() {
    assert_eq!(FEATURE_STATE_OPS_SVC_FACTORY, "FeatureStateOps");
}
