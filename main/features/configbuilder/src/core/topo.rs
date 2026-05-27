use std::collections::{HashMap, VecDeque};

/// Sort `names` topologically according to the `requires` dependency lists.
///
/// Returns indices into `names` in a valid load order (dependencies first).
/// Unknown dependency names (those not in `names`) are silently ignored; they
/// are validated separately by [`FeatureRegistry::validate_dependencies`].
///
/// # Errors
///
/// Returns an error string naming the cycle members when a cycle is detected.
///
/// [`FeatureRegistry::validate_dependencies`]: crate::saf::configbuilder_svc::FeatureRegistry::validate_dependencies
pub fn topo_sort(names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, String> {
    let n = names.len();
    let index: HashMap<&str, usize> = names
        .iter()
        .enumerate()
        .map(|(i, &name)| (name, i))
        .collect();

    // adj[j] = nodes that depend on j (j must load first)
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_degree: Vec<usize> = vec![0; n];

    for (i, deps) in requires.iter().enumerate() {
        for dep in *deps {
            if let Some(&j) = index.get(dep) {
                adj[j].push(i);
                in_degree[i] += 1;
            }
        }
    }

    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut order = Vec::with_capacity(n);
    while let Some(i) = queue.pop_front() {
        order.push(i);
        for &j in &adj[i] {
            in_degree[j] -= 1;
            if in_degree[j] == 0 {
                queue.push_back(j);
            }
        }
    }

    if order.len() == n {
        Ok(order)
    } else {
        let cycle: Vec<&str> = names
            .iter()
            .enumerate()
            .filter(|(i, _)| in_degree[*i] > 0)
            .map(|(_, name)| *name)
            .collect();
        Err(format!(
            "dependency cycle detected among: {}",
            cycle.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort_no_deps_returns_all_indices() {
        let names = &["a", "b", "c"];
        let requires: &[&[&str]] = &[&[], &[], &[]];
        let order = topo_sort(names, requires).unwrap();
        assert_eq!(order.len(), 3);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2]);
    }

    #[test]
    fn test_topo_sort_chain_loads_dependency_before_dependent() {
        // b (index 0) requires a (index 1) — a must appear before b
        let names = &["b", "a"];
        let requires: &[&[&str]] = &[&["a"], &[]];
        let order = topo_sort(names, requires).unwrap();
        let pos_a = order.iter().position(|&i| i == 1).unwrap();
        let pos_b = order.iter().position(|&i| i == 0).unwrap();
        assert!(pos_a < pos_b, "a must load before b");
    }

    #[test]
    fn test_topo_sort_diamond_orders_roots_before_leaves() {
        // names: d(0), b(1), c(2), a(3); d needs b,c; b needs a; c needs a
        let names = &["d", "b", "c", "a"];
        let requires: &[&[&str]] = &[&["b", "c"], &["a"], &["a"], &[]];
        let order = topo_sort(names, requires).unwrap();
        let pos = |name: &str| -> usize {
            let idx = names.iter().position(|&n| n == name).unwrap();
            order.iter().position(|&i| i == idx).unwrap()
        };
        assert!(pos("a") < pos("b"), "a before b");
        assert!(pos("a") < pos("c"), "a before c");
        assert!(pos("b") < pos("d"), "b before d");
        assert!(pos("c") < pos("d"), "c before d");
    }

    #[test]
    fn test_topo_sort_cycle_returns_error_naming_cycle_members() {
        let names = &["a", "b"];
        let requires: &[&[&str]] = &[&["b"], &["a"]];
        let err = topo_sort(names, requires).unwrap_err();
        assert!(
            err.contains("cycle"),
            "error must mention 'cycle', got: {err}"
        );
    }

    #[test]
    fn test_topo_sort_unknown_dep_does_not_error() {
        let names = &["a"];
        let requires: &[&[&str]] = &[&["ghost"]];
        let order = topo_sort(names, requires).unwrap();
        assert_eq!(order, vec![0]);
    }

    #[test]
    fn test_topo_sort_empty_input_returns_empty_order() {
        let order = topo_sort(&[], &[]).unwrap();
        assert!(order.is_empty());
    }
}
