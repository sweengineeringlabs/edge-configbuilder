/// Policy for validating environment variable substitutions in configuration.
///
/// Implementations define which environment variables are allowed to be substituted
/// into configuration values. Applications must implement this trait to enable
/// substitution and enforce their own security constraints.
///
/// Built-in implementations are provided: `AllowAllPolicy`, `PrefixWhitelistPolicy`,
/// `PatternWhitelistPolicy`, and `CompositePolicy`.
pub trait SubstitutionPolicy {
    /// Validate whether an environment variable is allowed to be substituted.
    ///
    /// # Arguments
    /// * `var_name` - The name of the environment variable to validate
    ///
    /// # Returns
    /// - `Ok(())` if the variable is allowed
    /// - `Err(msg)` if the variable is not allowed, with explanation of why
    fn validate(&self, var_name: &str) -> Result<(), String>;

    /// Optional: Get a human-readable description of this policy for error messages.
    fn description(&self) -> String {
        "custom policy".to_string()
    }
}
