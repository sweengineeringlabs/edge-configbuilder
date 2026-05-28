//! Interface counterpart for [`crate::core::validator::default_validator::DefaultValidator`].

/// API contract marker for the default path validator.
///
/// The concrete implementor is
/// `crate::core::validator::default_validator::DefaultValidator`, which
/// implements [`crate::api::traits::validator::Validator`].
pub trait DefaultValidator: crate::api::traits::validator::Validator {}
