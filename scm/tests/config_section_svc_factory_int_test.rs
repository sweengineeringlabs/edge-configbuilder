use swe_edge_configbuilder::CONFIG_SECTION_SVC_FACTORY;

#[test]
fn test_config_section_svc_factory_has_constant() {
    assert_eq!(CONFIG_SECTION_SVC_FACTORY, "ConfigSection");
}
