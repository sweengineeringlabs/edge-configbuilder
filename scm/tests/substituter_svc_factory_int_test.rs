use swe_edge_configbuilder::SUBSTITUTER_SVC_FACTORY;

#[test]
fn test_substituter_svc_factory_has_constant() {
    assert_eq!(SUBSTITUTER_SVC_FACTORY, "Substituter");
}
