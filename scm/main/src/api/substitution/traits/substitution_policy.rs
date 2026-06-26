/// Policy for controlling which environment variables may be substituted into config values.
///
/// Implementations decide whether a variable name is allowed. The loader calls
/// `validate` on every `{{VAR_NAME}}` placeholder before substituting it;
/// rejection returns a config I/O error to the caller.
///
/// Built-in implementations: [`PrefixWhitelistPolicy`], [`PatternWhitelistPolicy`],
/// [`CompositePolicy`].  For tests only: `AllowAllPolicy` (requires the
/// `test-utils` feature).  Implement this trait when the built-ins do not match
/// your security requirements.
///
/// [`PrefixWhitelistPolicy`]: crate::PrefixWhitelistPolicy
/// [`PatternWhitelistPolicy`]: crate::PatternWhitelistPolicy
/// [`CompositePolicy`]: crate::CompositePolicy
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::SubstitutionPolicy;
///
/// /// Allows only variables whose names are all-uppercase ASCII.
/// struct UppercaseOnlyPolicy;
///
/// impl SubstitutionPolicy for UppercaseOnlyPolicy {
///     fn validate(&self, var_name: &str) -> Result<(), String> {
///         if var_name.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
///             Ok(())
///         } else {
///             Err(format!("'{}' must be UPPER_SNAKE_CASE", var_name))
///         }
///     }
///     fn description(&self) -> String {
///         "UppercaseOnly".to_string()
///     }
/// }
///
/// let policy = UppercaseOnlyPolicy;
/// assert!(policy.validate("APP_HOST").is_ok());
/// assert!(policy.validate("app_host").is_err());
/// ```
pub trait SubstitutionPolicy: Send + Sync {
    /// Decide whether `var_name` may be substituted into a config value.
    ///
    /// Return `Ok(())` to permit substitution or `Err(reason)` to reject it.
    /// The `reason` string is included verbatim in the config I/O error message
    /// returned to the caller.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, SubstitutionPolicy};
    /// assert!(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])
    ///     .validate("APP_HOST")
    ///     .is_ok());
    /// ```
    fn validate(&self, var_name: &str) -> Result<(), String>;

    /// Human-readable label included in rejection error messages.
    ///
    /// Defaults to `"custom policy"`. Override to identify your policy by name in
    /// logs and error output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, SubstitutionPolicy};
    /// assert_eq!(
    ///     PrefixWhitelistPolicy::new(vec!["APP_".to_string()]).description(),
    ///     "PrefixWhitelist(APP_)"
    /// );
    /// ```
    fn description(&self) -> String {
        "custom policy".to_string()
    }
}
