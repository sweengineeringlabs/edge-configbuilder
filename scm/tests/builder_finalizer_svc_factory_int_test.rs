#![allow(missing_docs)]
use swe_edge_configbuilder::BUILDER_FINALIZER_SVC_FACTORY;

#[test]
fn test_builder_finalizer_svc_factory_constant_matches_trait_name() {
    assert_eq!(BUILDER_FINALIZER_SVC_FACTORY, "BuilderFinalizer");
}
