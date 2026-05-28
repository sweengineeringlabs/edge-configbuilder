use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Combines multiple validation policies with AND logic (all must pass).
pub struct CompositePolicy {
    policies: Vec<Box<dyn SubstitutionPolicy>>,
}

impl std::fmt::Debug for CompositePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompositePolicy")
            .field("policy_count", &self.policies.len())
            .finish()
    }
}

impl CompositePolicy {
    /// Create a new composite policy from a list of policies.
    pub fn new(policies: Vec<Box<dyn SubstitutionPolicy>>) -> Self {
        Self { policies }
    }
}

impl SubstitutionPolicy for CompositePolicy {
    fn validate(&self, var_name: &str) -> Result<(), String> {
        let mut errors = Vec::new();
        for policy in &self.policies {
            if let Err(e) = policy.validate(var_name) {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(format!("Validation failed: {}", errors.join(" AND ")))
        }
    }

    fn description(&self) -> String {
        format!(
            "Composite({})",
            self.policies
                .iter()
                .map(|p| p.description())
                .collect::<Vec<_>>()
                .join(" AND ")
        )
    }
}
