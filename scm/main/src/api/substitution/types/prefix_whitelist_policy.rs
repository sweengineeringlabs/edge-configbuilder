use crate::api::substitution::traits::substitution_policy::SubstitutionPolicy;

/// Allows only environment variables whose names start with one of the given prefixes.
///
/// Any variable that does not match at least one prefix is rejected with an
/// explanatory error message listing all allowed prefixes. Prefix comparison is
/// case-sensitive and byte-exact.
///
/// Use this as the default production policy when your config variables share a
/// common namespace prefix (e.g. `APP_`, `SWE_EDGE_`).
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{PrefixWhitelistPolicy, SubstitutionPolicy};
///
/// let policy = PrefixWhitelistPolicy::new(vec!["APP_".to_string(), "SWE_".to_string()]);
///
/// assert!(policy.validate("APP_PORT").is_ok());
/// assert!(policy.validate("SWE_LOG_LEVEL").is_ok());
/// assert!(policy.validate("SECRET_KEY").is_err());
/// assert_eq!(policy.prefixes(), &["APP_", "SWE_"]);
/// ```
#[derive(Debug)]
pub struct PrefixWhitelistPolicy {
    prefixes: Vec<String>,
}

impl PrefixWhitelistPolicy {
    /// Create a prefix whitelist from the given list of allowed prefixes.
    ///
    /// An empty `prefixes` list rejects every variable. Pass `[""]` as the
    /// sole prefix to accept all variables when unrestricted access is needed
    /// (tests only — do not use in production).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, SubstitutionPolicy};
    /// let policy = PrefixWhitelistPolicy::new(vec!["APP_".to_string()]);
    /// assert!(policy.validate("APP_HOST").is_ok());
    /// ```
    pub fn new(prefixes: Vec<String>) -> Self {
        Self { prefixes }
    }

    /// Returns the allowed prefixes in declaration order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::PrefixWhitelistPolicy;
    /// let policy = PrefixWhitelistPolicy::new(vec!["FOO_".to_string()]);
    /// assert_eq!(policy.prefixes(), &["FOO_"]);
    /// ```
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
