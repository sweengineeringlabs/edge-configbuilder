#![allow(missing_docs)]
use configbuilder::OPTIONAL_SECTION_SVC_FACTORY;

#[test]
fn test_optional_section_svc_factory_has_constant() {
    assert_eq!(OPTIONAL_SECTION_SVC_FACTORY, "OptionalSection");
}
