#![allow(missing_docs)]
use swe_edge_configbuilder::VALIDATOR_BOUND_SVC_FACTORY;

#[test]
fn test_validator_bound_svc_factory_has_constant() {
    assert_eq!(VALIDATOR_BOUND_SVC_FACTORY, "ValidatorBound");
}
