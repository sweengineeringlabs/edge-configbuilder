use crate::api::SubstitutionError;
use crate::{PatternWhitelistPolicy, SubstitutionPolicy};

impl PatternWhitelistPolicy {
    /// Create a regex-backed whitelist policy.
    pub fn new(pattern: String) -> Result<Self, String> {
        regex::Regex::new(&pattern)
            .map(|regex| Self {
                pattern: regex,
                pattern_str: pattern,
            })
            .map_err(|e| format!("Invalid regex pattern: {}", e))
    }

    /// Return the original regex pattern string.
    pub fn pattern(&self) -> &str {
        &self.pattern_str
    }
}

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
