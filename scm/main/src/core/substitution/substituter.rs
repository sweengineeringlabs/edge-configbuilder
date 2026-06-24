use crate::api::{
    PolicyCatalog, SubstituterBound as SubstituterContract, SubstitutionError, SubstitutionPolicy,
};
use regex::Regex;
use std::env;

/// Performs environment variable substitution on a string value.
///
/// Supports `{{VAR_NAME}}` syntax. Escaping is supported via `\{\{` and `\}\}`.
pub(crate) struct Substituter<'a> {
    policy: &'a dyn SubstitutionPolicy,
    location: String,
}

impl<'a> Substituter<'a> {
    /// Create a new substituter with a reference to a policy.
    pub(crate) fn new(policy: &'a dyn SubstitutionPolicy, location: String) -> Self {
        Self { policy, location }
    }

    /// Substitute all `{{VAR_NAME}}` placeholders in the given value.
    ///
    /// # Arguments
    /// * `value` - The value to substitute
    ///
    /// # Returns
    /// - `Ok(substituted_value)` if substitution succeeds
    /// - `Err(SubstitutionError)` if:
    ///   - A referenced variable doesn't exist
    ///   - A variable is rejected by the policy
    ///   - The syntax is invalid (e.g., nested placeholders)
    pub(crate) fn substitute(&self, value: &str) -> Result<String, SubstitutionError> {
        // Check for invalid nested syntax
        if value.contains("{{{{") || value.contains("}}}}") {
            return Err(SubstitutionError::InvalidSubstitutionSyntax {
                value: value.to_string(),
                reason: "nested placeholders (e.g., {{VAR_{{OTHER}}}}) are not supported"
                    .to_string(),
            });
        }

        // Replace escaped braces first (temporarily)
        let escaped = value
            .replace("\\{\\{", "\x00ESCAPED_OPEN\x00")
            .replace("\\}\\}", "\x00ESCAPED_CLOSE\x00");

        // Find and replace all {{VAR_NAME}} patterns
        #[allow(clippy::expect_used)]
        let re = Regex::new(r"\{\{([A-Za-z_][A-Za-z0-9_]*)\}\}")
            .expect("valid regex pattern for {{VAR_NAME}} substitution");
        let mut first_error: Option<SubstitutionError> = None;
        let result = re
            .replace_all(&escaped, |caps: &regex::Captures| {
                let var_name = &caps[1];
                match self.substitute_var(var_name) {
                    Ok(value) => value,
                    Err(e) => {
                        // Store the first error encountered
                        if first_error.is_none() {
                            first_error = Some(e);
                        }
                        // Return a sentinel; we'll check errors after
                        String::new()
                    }
                }
            })
            .to_string();

        // Return the first error if any occurred
        if let Some(err) = first_error {
            return Err(err);
        }

        // Restore escaped braces
        Ok(result
            .replace("\x00ESCAPED_OPEN\x00", "{{")
            .replace("\x00ESCAPED_CLOSE\x00", "}}"))
    }

    /// Substitute a single environment variable.
    fn substitute_var(&self, var_name: &str) -> Result<String, SubstitutionError> {
        // Validate the variable name using the policy
        self.policy
            .validate(var_name)
            .map_err(|reason| SubstitutionError::VariableRejected {
                var_name: var_name.to_string(),
                reason,
                policy: self.policy.description(),
            })?;

        // Get the variable value
        env::var(var_name).map_err(|_| SubstitutionError::VariableNotFound {
            var_name: var_name.to_string(),
            location: self.location.clone(),
        })
    }
}

impl<'a> SubstituterContract for Substituter<'a> {}

impl<'a> PolicyCatalog for Substituter<'a> {
    type SubstitutionError = crate::api::SubstitutionError;
    type CompositePolicy = crate::api::CompositePolicy;
    type PatternWhitelistPolicy = crate::api::PatternWhitelistPolicy;
    type PrefixWhitelistPolicy = crate::api::PrefixWhitelistPolicy;

    #[cfg(any(test, feature = "test-utils"))]
    type AllowAllPolicy = crate::api::AllowAllPolicy;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::AllowAllPolicy;

    fn must<T, E>(result: Result<T, E>) -> T {
        result.unwrap_or_else(|_| std::process::abort())
    }

    fn substituter(policy: &dyn SubstitutionPolicy) -> Substituter<'_> {
        Substituter::new(policy, "test_location".to_string())
    }

    #[test]
    fn test_new() {
        let policy = AllowAllPolicy;
        let sub = Substituter::new(&policy, "test_file.toml".to_string());
        assert!(!sub.location.is_empty());
    }

    #[test]
    fn test_substitute() {
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = must(sub.substitute("value"));
        assert_eq!(result, "value");
    }

    #[test]
    fn test_no_substitution_needed() {
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = must(sub.substitute("hello world"));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_single_substitution() {
        std::env::set_var("TEST_VAR", "value123");
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = must(sub.substitute("prefix-{{TEST_VAR}}-suffix"));
        assert_eq!(result, "prefix-value123-suffix");
    }

    #[test]
    fn test_multiple_substitutions() {
        std::env::set_var("VAR1", "hello");
        std::env::set_var("VAR2", "world");
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = must(sub.substitute("{{VAR1}} {{VAR2}}"));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_escaped_braces() {
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = must(sub.substitute(r"use \{\{VAR\}\} for substitution"));
        assert_eq!(result, "use {{VAR}} for substitution");
    }

    #[test]
    fn test_missing_variable() {
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = sub.substitute("value: {{NONEXISTENT_VAR}}");
        assert!(matches!(
            result,
            Err(SubstitutionError::VariableNotFound { .. })
        ));
    }

    #[test]
    fn test_nested_placeholder_rejected() {
        let policy = AllowAllPolicy;
        let sub = substituter(&policy);
        let result = sub.substitute("{{VAR_{{INNER}}}}");
        assert!(matches!(
            result,
            Err(SubstitutionError::InvalidSubstitutionSyntax { .. })
        ));
    }

    #[test]
    fn test_policy_rejection() {
        use crate::api::PrefixWhitelistPolicy;
        let policy = PrefixWhitelistPolicy::new(vec!["ALLOWED_".to_string()]);
        let sub = substituter(&policy);
        let result = sub.substitute("{{FORBIDDEN_VAR}}");
        assert!(matches!(
            result,
            Err(SubstitutionError::VariableRejected { .. })
        ));
    }
}
