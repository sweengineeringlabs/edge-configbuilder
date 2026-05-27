//! End-to-end tests for the internal topological sort utility.

use swe_edge_configbuilder::__internal::Topology;

#[test]
fn test_topo_sort_single_node_no_deps_returns_index_zero() {
    let order = Topology::sort(&["a"], &[&[]]).unwrap();
    assert_eq!(order, vec![0]);
}

#[test]
fn test_topo_sort_three_independent_nodes_returns_all_three_indices() {
    let order = Topology::sort(&["x", "y", "z"], &[&[], &[], &[]]).unwrap();
    assert_eq!(order.len(), 3);
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1, 2]);
}

#[test]
fn test_topo_sort_linear_chain_dependency_loads_first() {
    let order = Topology::sort(&["b", "a"], &[&["a"], &[]]).unwrap();
    let pos_a = order.iter().position(|&i| i == 1).unwrap();
    let pos_b = order.iter().position(|&i| i == 0).unwrap();
    assert!(pos_a < pos_b, "a must load before b (b depends on a)");
}

#[test]
fn test_topo_sort_diamond_graph_root_loads_before_leaf() {
    let names = &["d", "b", "c", "a"];
    let requires: &[&[&str]] = &[&["b", "c"], &["a"], &["a"], &[]];
    let order = Topology::sort(names, requires).unwrap();
    let pos = |name: &str| -> usize {
        let idx = names.iter().position(|&n| n == name).unwrap();
        order.iter().position(|&i| i == idx).unwrap()
    };
    assert!(pos("a") < pos("b"));
    assert!(pos("a") < pos("c"));
    assert!(pos("b") < pos("d"));
    assert!(pos("c") < pos("d"));
}

#[test]
fn test_topo_sort_two_node_cycle_returns_err_with_cycle_in_message() {
    let err = Topology::sort(&["a", "b"], &[&["b"], &["a"]]).unwrap_err();
    assert!(
        err.contains("cycle"),
        "error message must name the cycle, got: {err}"
    );
}

#[test]
fn test_topo_sort_self_cycle_single_node_returns_err() {
    let err = Topology::sort(&["a"], &[&["a"]]).unwrap_err();
    assert!(err.contains("cycle"), "self-cycle must be detected: {err}");
}

#[test]
fn test_topo_sort_unknown_dependency_is_silently_ignored() {
    let order = Topology::sort(&["a", "b"], &[&[], &["ghost"]]).unwrap();
    assert_eq!(order.len(), 2);
}

#[test]
fn test_topo_sort_empty_slice_returns_empty_order() {
    let order = Topology::sort(&[], &[]).unwrap();
    assert!(order.is_empty());
}
