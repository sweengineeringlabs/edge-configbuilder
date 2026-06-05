use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// Combines multiple [`SubstitutionPolicy`] implementations with AND logic.
///
/// A variable is allowed only when **all** constituent policies accept it.
/// The first rejection short-circuits: remaining policies are still checked and
/// their errors are concatenated so operators can see every constraint in one message.
///
/// Use this when you need to stack independent constraints, e.g. a prefix
/// whitelist AND an uppercase-only pattern.
///
/// [`SubstitutionPolicy`]: crate::SubstitutionPolicy
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{CompositePolicy, PrefixWhitelistPolicy, SubstitutionPolicy};
///
/// let policy = CompositePolicy::new(vec![
///     Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
/// ]);
///
/// assert!(policy.validate("APP_HOST").is_ok());
/// assert!(policy.validate("SECRET").is_err()); // rejected by prefix policy
/// ```
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
    /// Create a composite policy from a list of policies.
    ///
    /// An empty `policies` list accepts every variable — add at least one
    /// restrictive policy to get meaningful protection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::{CompositePolicy, PrefixWhitelistPolicy, SubstitutionPolicy};
    ///
    /// let policy = CompositePolicy::new(vec![
    ///     Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
    /// ]);
    /// assert!(policy.validate("APP_HOST").is_ok());
    /// ```
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
