use crate::api::SubstitutionError;

/// Resolves a substitution variable name to its value.
///
/// [`Substituter`] applies this after [`SubstitutionPolicy`] has approved a
/// variable name, decoupling *which names are allowed* from *where their
/// values come from*. The default implementor is [`EnvValueResolver`],
/// backed by `std::env::var`; downstream consumers may supply their own
/// implementation to source values from a secrets backend (Vault, AWS
/// Secrets Manager, etc.) instead of the process environment.
///
/// [`Substituter`]: crate::api::substitution::substituter::Substituter
/// [`SubstitutionPolicy`]: crate::api::substitution::traits::substitution_policy::SubstitutionPolicy
/// [`EnvValueResolver`]: crate::EnvValueResolver
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{SubstitutionError, ValueResolver};
///
/// struct StaticResolver;
///
/// impl ValueResolver for StaticResolver {
///     fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError> {
///         match var_name {
///             "GREETING" => Ok("hello".to_string()),
///             _ => Err(SubstitutionError::VariableNotFound {
///                 var_name: var_name.to_string(),
///                 location: location.to_string(),
///             }),
///         }
///     }
/// }
///
/// let resolver = StaticResolver;
/// assert_eq!(resolver.resolve("GREETING", "app.toml").unwrap(), "hello");
/// assert!(resolver.resolve("MISSING", "app.toml").is_err());
/// ```
pub trait ValueResolver: Send + Sync {
    /// Resolve `var_name` to its value.
    ///
    /// `location` identifies the config file and key the placeholder
    /// appeared in, for inclusion in the returned error on failure.
    ///
    /// # Errors
    ///
    /// Returns [`SubstitutionError`] when the variable cannot be resolved
    /// (not found, access denied, backend unavailable, etc.).
    fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError>;
}
