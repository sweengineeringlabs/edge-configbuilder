//! API contract for the substitution policy and error type family.

#[cfg(any(test, feature = "test-utils"))]
use crate::api::AllowAllPolicy;
use crate::api::{
    CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy, SubstitutionError,
};

/// Contract binding the substitution policy and error type family.
pub trait PolicyCatalog {
    /// Error produced before conversion to the public config error.
    type SubstitutionError: Into<SubstitutionError>;

    /// Composite environment variable substitution policy.
    type CompositePolicy: Into<CompositePolicy>;

    /// Regex-backed environment variable substitution policy.
    type PatternWhitelistPolicy: Into<PatternWhitelistPolicy>;

    /// Prefix-backed environment variable substitution policy.
    type PrefixWhitelistPolicy: Into<PrefixWhitelistPolicy>;

    /// Test-only substitution policy that permits every variable.
    #[cfg(any(test, feature = "test-utils"))]
    type AllowAllPolicy: Into<AllowAllPolicy>;
}
