//! [`Topology`] — topological dependency resolver for feature load ordering.

use crate::api::{ConfigError, Topology, TopologyOps};

impl TopologyOps for Topology {
    fn sort(&self, names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, ConfigError> {
        let n = names.len();
        let index: std::collections::HashMap<&str, usize> = names
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

        let mut queue: std::collections::VecDeque<usize> = in_degree
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
            Err(ConfigError::Validation {
                section: String::from("dependency_graph"),
                reason: format!(
                    "dependency cycle detected among: {}",
                    cycle.join(", ")
                ),
            })
        }
    }
}
