use swe_edge_configbuilder::LOADER_OPS_SVC_FACTORY;

#[test]
fn test_loader_ops_svc_factory_has_constant() {
    assert_eq!(LOADER_OPS_SVC_FACTORY, "LoaderOps");
}
