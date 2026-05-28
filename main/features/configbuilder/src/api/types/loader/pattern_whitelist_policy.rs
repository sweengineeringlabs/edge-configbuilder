use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Allows only environment variables matching a regular expression pattern.
#[derive(Debug)]
pub struct PatternWhitelistPolicy {
    pattern: regex::Regex,
    pattern_str: String,
}

impl PatternWhitelistPolicy {
    /// Create a new pattern whitelist policy.
    pub fn new(pattern: String) -> Result<Self, String> {
        regex::Regex::new(&pattern)
            .map(|regex| Self {
                pattern: regex,
                pattern_str: pattern,
            })
            .map_err(|e| format!("Invalid regex pattern: {}", e))
    }

    /// Get the pattern string.
    pub fn pattern(&self) -> &str {
        &self.pattern_str
    }
}

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
