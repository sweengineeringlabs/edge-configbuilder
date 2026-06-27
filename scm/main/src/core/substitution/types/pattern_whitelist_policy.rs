use crate::api::SubstitutionError;
use crate::{PatternWhitelistPolicy, SubstitutionPolicy};

impl SubstitutionPolicy for PatternWhitelistPolicy {
    fn validate(&self, var_name: &str) -> Result<(), SubstitutionError> {
        if self.pattern.is_match(var_name) {
            Ok(())
        } else {
            Err(SubstitutionError::VariableRejected {
                var_name: var_name.to_string(),
                reason: format!("does not match pattern: {}", self.pattern_str),
                policy: self.description(),
            })
        }
    }

    fn description(&self) -> String {
        format!("PatternWhitelist({})", self.pattern_str)
    }
}
