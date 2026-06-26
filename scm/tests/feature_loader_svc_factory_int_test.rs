use swe_edge_configbuilder::FEATURE_LOADER_SVC_FACTORY;

#[test]
fn test_feature_loader_svc_factory_has_constant() {
    assert_eq!(FEATURE_LOADER_SVC_FACTORY, "FeatureLoader");
}
