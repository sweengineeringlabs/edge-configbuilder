use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Allows only environment variables matching specified prefixes.
#[derive(Debug)]
pub struct PrefixWhitelistPolicy {
    prefixes: Vec<String>,
}

impl PrefixWhitelistPolicy {
    /// Create a new prefix whitelist policy.
    pub fn new(prefixes: Vec<String>) -> Self {
        Self { prefixes }
    }

    /// Get the allowed prefixes.
    pub fn prefixes(&self) -> &[String] {
        &self.prefixes
    }
}

impl SubstitutionPolicy for PrefixWhitelistPolicy {
    fn validate(&self, var_name: &str) -> Result<(), String> {
        if self
            .prefixes
            .iter()
            .any(|p| var_name.starts_with(p.as_str()))
        {
            Ok(())
        } else {
            Err(format!(
                "Variable '{}' does not match any allowed prefix: {}",
                var_name,
                self.prefixes.join(", ")
            ))
        }
    }

    fn description(&self) -> String {
        format!("PrefixWhitelist({})", self.prefixes.join(", "))
    }
}
