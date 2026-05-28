use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Allows all environment variables to be substituted (no restrictions).
///
/// **Warning:** Use only in development or trusted environments. In production,
/// prefer [`PrefixWhitelistPolicy`] or [`PatternWhitelistPolicy`].
///
/// [`PrefixWhitelistPolicy`]: crate::api::types::loader::prefix_whitelist_policy::PrefixWhitelistPolicy
/// [`PatternWhitelistPolicy`]: crate::api::types::loader::pattern_whitelist_policy::PatternWhitelistPolicy
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
