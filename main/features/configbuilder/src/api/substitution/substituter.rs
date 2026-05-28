//! Interface counterpart for [`crate::core::substitution::substituter::Substituter`].

/// API contract marker for the substituter.
///
/// The concrete implementor is `crate::core::substitution::substituter::Substituter`,
/// which applies a [`crate::api::traits::substitution_policy::SubstitutionPolicy`]
/// to TOML values.
pub trait Substituter {}
