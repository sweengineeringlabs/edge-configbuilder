//! Interface counterpart for [`crate::core::validator::default_validator::DefaultValidator`].

use crate::api::PathValidatorImpl;

/// API contract marker for the default path validator.
///
/// The concrete implementor is
/// `crate::core::validator::default_validator::DefaultValidator`.
pub trait ValidatorBound {
    /// Public validator facade type.
    type PathValidator: Into<PathValidatorImpl>;
}
