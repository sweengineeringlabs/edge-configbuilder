/// Trait for environment variable substitution within TOML values.
///
/// Implementations perform `{{VAR_NAME}}` placeholder substitution,
/// applying an optional validation policy to restrict which variables can be substituted.
pub trait Substitution {
    /// Substitute all `{{VAR_NAME}}` placeholders in the given value.
    ///
    /// # Arguments
    /// * `value` - The value to substitute
    ///
    /// # Returns
    /// - `Ok(substituted_value)` if substitution succeeds
    /// - `Err` if an environment variable is missing or rejected by policy
    fn substitute(
        &self,
        value: &str,
    ) -> Result<String, crate::api::error::config_error::ConfigError>;
}
