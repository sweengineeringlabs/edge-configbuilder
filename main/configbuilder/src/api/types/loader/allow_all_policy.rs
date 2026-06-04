use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Allows all environment variables to be substituted (no restrictions).
///
/// Every `{{VAR_NAME}}` placeholder in TOML values is resolved without checking
/// the variable name against any whitelist or pattern.
///
/// **Warning:** Use only in development or fully trusted environments. In production,
/// prefer [`PrefixWhitelistPolicy`] or [`PatternWhitelistPolicy`] to prevent
/// accidental leakage of sensitive variables (e.g. `AWS_SECRET_ACCESS_KEY`).
///
/// [`PrefixWhitelistPolicy`]: crate::PrefixWhitelistPolicy
/// [`PatternWhitelistPolicy`]: crate::PatternWhitelistPolicy
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{AllowAllPolicy, SubstitutionPolicy};
///
/// let policy = AllowAllPolicy;
/// assert!(policy.validate("ANY_VAR").is_ok());
/// assert!(policy.validate("SECRET_KEY").is_ok());
/// assert!(policy.validate("DATABASE_URL").is_ok());
/// ```
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
