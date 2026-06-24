//! API counterpart for the core environment-variable substituter.

/// API contract marker for the substituter.
///
/// The concrete implementor is `crate::core::substitution::substituter::Substituter`,
/// which applies a [`crate::api::substitution::traits::substitution_policy::SubstitutionPolicy`]
/// to TOML values.
pub trait Substituter {}
