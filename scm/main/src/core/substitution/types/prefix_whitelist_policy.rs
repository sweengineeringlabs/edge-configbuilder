use crate::api::SubstitutionError;
use crate::{PrefixWhitelistPolicy, SubstitutionPolicy};

impl SubstitutionPolicy for PrefixWhitelistPolicy {
    fn validate(&self, var_name: &str) -> Result<(), SubstitutionError> {
        if self
            .prefixes
            .iter()
            .any(|p| var_name.starts_with(p.as_str()))
        {
            Ok(())
        } else {
            Err(SubstitutionError::VariableRejected {
                var_name: var_name.to_string(),
                reason: format!(
                    "does not match any allowed prefix: {}",
                    self.prefixes.join(", ")
                ),
                policy: self.description(),
            })
        }
    }

    fn description(&self) -> String {
        format!("PrefixWhitelist({})", self.prefixes.join(", "))
    }
}
