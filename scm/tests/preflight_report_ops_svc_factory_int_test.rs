#![allow(missing_docs)]
use swe_edge_configbuilder::PREFLIGHT_REPORT_OPS_SVC_FACTORY;

#[test]
fn test_preflight_report_ops_svc_factory_constant_matches_trait_name() {
    assert_eq!(PREFLIGHT_REPORT_OPS_SVC_FACTORY, "PreflightReportOps");
}
