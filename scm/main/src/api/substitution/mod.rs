//! Substitution theme — environment-variable expansion policy for config values.
//!
//! Owns the [`SubstitutionPolicy`] port, its built-in whitelist implementations,
//! the substitution error type, and the API marker for the core substituter.
//!
//! [`SubstitutionPolicy`]: traits::substitution_policy::SubstitutionPolicy

pub mod error;
pub mod substituter;
pub mod traits;
pub mod types;
