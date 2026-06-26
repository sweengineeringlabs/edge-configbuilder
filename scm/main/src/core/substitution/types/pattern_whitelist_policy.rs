use crate::{PatternWhitelistPolicy, SubstitutionPolicy};

impl SubstitutionPolicy for PatternWhitelistPolicy {
    fn validate(&self, var_name: &str) -> Result<(), String> {
        if self.pattern.is_match(var_name) {
            Ok(())
        } else {
            Err(format!(
                "Variable '{}' does not match pattern: {}",
                var_name, self.pattern_str
            ))
        }
    }

    fn description(&self) -> String {
        format!("PatternWhitelist({})", self.pattern_str)
    }
}
