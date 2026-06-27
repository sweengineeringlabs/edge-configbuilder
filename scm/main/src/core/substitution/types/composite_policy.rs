use crate::api::SubstitutionError;
use crate::{CompositePolicy, SubstitutionPolicy};

impl std::fmt::Debug for CompositePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompositePolicy")
            .field("policy_count", &self.policies.len())
            .finish()
    }
}

impl SubstitutionPolicy for CompositePolicy {
    fn validate(&self, var_name: &str) -> Result<(), SubstitutionError> {
        let mut reasons = Vec::new();
        for policy in &self.policies {
            if let Err(SubstitutionError::VariableRejected { reason, .. }) =
                policy.validate(var_name)
            {
                reasons.push(reason);
            }
        }
        if reasons.is_empty() {
            Ok(())
        } else {
            Err(SubstitutionError::VariableRejected {
                var_name: var_name.to_string(),
                reason: reasons.join(" AND "),
                policy: self.description(),
            })
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
