#![allow(missing_docs)]
use swe_edge_configbuilder::TOPOLOGY_OPS_SVC_FACTORY;

#[test]
fn test_topology_ops_svc_factory_constant_matches_trait_name() {
    assert_eq!(TOPOLOGY_OPS_SVC_FACTORY, "TopologyOps");
}
