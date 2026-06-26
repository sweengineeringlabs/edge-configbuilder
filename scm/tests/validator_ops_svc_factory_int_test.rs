use swe_edge_configbuilder::VALIDATOR_OPS_SVC_FACTORY;

#[test]
fn test_validator_ops_svc_factory_has_constant() {
    assert_eq!(VALIDATOR_OPS_SVC_FACTORY, "ValidatorOps");
}
