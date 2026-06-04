use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Allows only environment variables whose names fully match a regular expression.
///
/// The pattern is anchored to the full variable name — partial matches are
/// not accepted. Returns `Err` at construction time if the regex is invalid,
/// so callers can fail fast at startup rather than at substitution time.
///
/// Use this when you need finer control than prefix matching, e.g. allowing
/// only uppercase names with a specific structural format.
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{PatternWhitelistPolicy, SubstitutionPolicy};
///
/// let policy = PatternWhitelistPolicy::new(r"^APP_[A-Z0-9_]+$".to_string())
///     .expect("pattern is valid");
///
/// assert!(policy.validate("APP_PORT").is_ok());
/// assert!(policy.validate("APP_DATABASE_URL").is_ok());
/// assert!(policy.validate("SECRET_KEY").is_err());
/// assert!(policy.validate("app_port").is_err()); // lowercase rejected
///
/// assert_eq!(policy.pattern(), r"^APP_[A-Z0-9_]+$");
/// ```
#[derive(Debug)]
pub struct PatternWhitelistPolicy {
    pattern: regex::Regex,
    pattern_str: String,
}

impl PatternWhitelistPolicy {
    /// Create a pattern whitelist from a regex string.
    ///
    /// Returns `Err(msg)` when the regex is invalid. The error message includes
    /// the invalid pattern and the regex engine's reason.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::PatternWhitelistPolicy;
    ///
    /// assert!(PatternWhitelistPolicy::new(r"^[A-Z_]+$".to_string()).is_ok());
    /// assert!(PatternWhitelistPolicy::new("[invalid".to_string()).is_err());
    /// ```
    pub fn new(pattern: String) -> Result<Self, String> {
        regex::Regex::new(&pattern)
            .map(|regex| Self {
                pattern: regex,
                pattern_str: pattern,
            })
            .map_err(|e| format!("Invalid regex pattern: {}", e))
    }

    /// Returns the original pattern string used to construct this policy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::PatternWhitelistPolicy;
    /// let policy = PatternWhitelistPolicy::new(r"^FOO_".to_string()).unwrap();
    /// assert_eq!(policy.pattern(), r"^FOO_");
    /// ```
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
