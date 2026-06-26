#![allow(missing_docs)]
use swe_edge_configbuilder::VALIDATOR_SVC_FACTORY;

#[test]
fn test_validator_svc_factory_has_constant() {
    assert_eq!(VALIDATOR_SVC_FACTORY, "Validator");
}
