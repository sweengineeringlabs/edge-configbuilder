//! Integration tests for `TopologyOps` — `sort`.
#![allow(missing_docs, clippy::unwrap_used)]
use swe_edge_configbuilder::{Topology, TopologyOps as _};

// ── sort ──────────────────────────────────────────────────────────────────────

#[test]
fn test_sort_acyclic_graph_returns_valid_topological_order_happy() {
    // b depends on a → a must come first
    let order = Topology.sort(&["a", "b"], &[&[], &["a"]]).unwrap();
    let pos_a = order.iter().position(|&i| i == 0).unwrap();
    let pos_b = order.iter().position(|&i| i == 1).unwrap();
    assert!(pos_a < pos_b, "a must precede b in topological order");
}

#[test]
fn test_sort_cyclic_graph_returns_err_with_cycle_message_error() {
    let err = Topology.sort(&["a", "b"], &[&["b"], &["a"]]).unwrap_err();
    assert!(
        err.to_string().contains("cycle"),
        "error message must describe the cycle: {err}"
    );
}

#[test]
fn test_sort_empty_input_returns_empty_order_edge() {
    let order = Topology.sort(&[], &[]).unwrap();
    assert!(order.is_empty());
}
