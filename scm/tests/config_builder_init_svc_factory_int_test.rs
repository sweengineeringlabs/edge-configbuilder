#![allow(missing_docs)]
use swe_edge_configbuilder::CONFIG_BUILDER_INIT_SVC_FACTORY;

#[test]
fn test_config_builder_init_svc_factory_constant_matches_trait_name() {
    assert_eq!(CONFIG_BUILDER_INIT_SVC_FACTORY, "ConfigBuilderInit");
}
