#![allow(missing_docs)]
use configbuilder::SUBSTITUTION_POLICY_SVC_FACTORY;

#[test]
fn test_substitution_policy_svc_factory_has_constant() {
    assert_eq!(SUBSTITUTION_POLICY_SVC_FACTORY, "SubstitutionPolicy");
}
