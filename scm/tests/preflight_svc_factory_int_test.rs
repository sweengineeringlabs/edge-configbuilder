use swe_edge_configbuilder::PREFLIGHT_SVC_FACTORY;

#[test]
fn test_preflight_svc_factory_has_constant() {
    assert_eq!(PREFLIGHT_SVC_FACTORY, "Preflight");
}
