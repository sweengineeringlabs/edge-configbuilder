#![allow(missing_docs)]
use configbuilder::LOADER_SVC_FACTORY;

#[test]
fn test_loader_svc_factory_has_constant() {
    assert_eq!(LOADER_SVC_FACTORY, "Loader");
}
