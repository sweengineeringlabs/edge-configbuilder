#![allow(missing_docs)]
use configbuilder::PREFLIGHT_ISSUE_KIND_OPS_SVC_FACTORY;

#[test]
fn test_preflight_issue_kind_ops_svc_factory_constant_matches_trait_name() {
    assert_eq!(
        PREFLIGHT_ISSUE_KIND_OPS_SVC_FACTORY,
        "PreflightIssueKindOps"
    );
}
