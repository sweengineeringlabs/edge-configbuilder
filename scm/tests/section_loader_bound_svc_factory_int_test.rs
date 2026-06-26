#![allow(missing_docs)]
use swe_edge_configbuilder::SECTION_LOADER_BOUND_SVC_FACTORY;

#[test]
fn test_section_loader_bound_svc_factory_has_constant() {
    assert_eq!(SECTION_LOADER_BOUND_SVC_FACTORY, "SectionLoaderBound");
}
