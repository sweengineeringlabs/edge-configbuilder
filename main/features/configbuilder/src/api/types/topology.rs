use std::collections::{HashMap, VecDeque};

/// Topological dependency resolver for feature load ordering.
pub struct Topology;

impl Topology {
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
    /// [`FeatureRegistry::validate_dependencies`]: crate::api::types::feature_registry::FeatureRegistry::validate_dependencies
    pub fn sort(names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, String> {
        let n = names.len();
        let index: HashMap<&str, usize> = names
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();

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
}
