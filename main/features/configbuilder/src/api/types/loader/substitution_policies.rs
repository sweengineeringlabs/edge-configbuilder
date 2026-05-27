//! Built-in substitution policy implementations.

use crate::api::loader::traits::substitution_policy::SubstitutionPolicy;

/// Allows all environment variables to be substituted (no restrictions).
///
/// **Warning:** Use only in development or trusted environments. In production,
/// prefer `PrefixWhitelistPolicy` or `PatternWhitelistPolicy`.
#[derive(Debug)]
pub struct AllowAllPolicy;

impl SubstitutionPolicy for AllowAllPolicy {
    fn validate(&self, _var_name: &str) -> Result<(), String> {
        Ok(())
    }

    fn description(&self) -> String {
        "AllowAll (no restrictions)".to_string()
    }
}

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
        if self.prefixes.iter().any(|p| var_name.starts_with(p)) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_all_policy_accepts_any_variable() {
        let policy = AllowAllPolicy;
        assert!(policy.validate("ANY_VAR").is_ok());
    }

    #[test]
    fn test_prefix_whitelist_policy_accepts_matching() {
        let policy = PrefixWhitelistPolicy::new(vec!["APP_".into()]);
        assert!(policy.validate("APP_DEBUG").is_ok());
    }

    #[test]
    fn test_prefix_whitelist_policy_rejects_non_matching() {
        let policy = PrefixWhitelistPolicy::new(vec!["APP_".into()]);
        assert!(policy.validate("DB_HOST").is_err());
    }

    #[test]
    fn test_pattern_whitelist_policy_accepts_matching() {
        let policy = PatternWhitelistPolicy::new("^APP_[A-Z_]+$".into()).unwrap();
        assert!(policy.validate("APP_DEBUG").is_ok());
    }

    #[test]
    fn test_pattern_whitelist_policy_rejects_non_matching() {
        let policy = PatternWhitelistPolicy::new("^APP_[A-Z_]+$".into()).unwrap();
        assert!(policy.validate("app_debug").is_err());
    }

    #[test]
    fn test_composite_policy_requires_all() {
        let policies: Vec<Box<dyn SubstitutionPolicy>> =
            vec![Box::new(PrefixWhitelistPolicy::new(vec!["APP_".into()]))];
        let policy = CompositePolicy::new(policies);
        assert!(policy.validate("APP_DEBUG").is_ok());
        assert!(policy.validate("DB_HOST").is_err());
    }
}
