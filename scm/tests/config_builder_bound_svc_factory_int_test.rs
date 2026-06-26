use swe_edge_configbuilder::CONFIG_BUILDER_BOUND_SVC_FACTORY;

#[test]
fn test_config_builder_bound_svc_factory_has_constant() {
    assert_eq!(CONFIG_BUILDER_BOUND_SVC_FACTORY, "ConfigBuilderBound");
}
