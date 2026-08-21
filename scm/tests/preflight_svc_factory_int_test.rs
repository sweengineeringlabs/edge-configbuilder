#![allow(missing_docs)]
use configbuilder::PREFLIGHT_SVC_FACTORY;

#[test]
fn test_preflight_svc_factory_has_constant() {
    assert_eq!(PREFLIGHT_SVC_FACTORY, "Preflight");
}
