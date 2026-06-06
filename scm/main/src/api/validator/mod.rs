//! Validator theme — config-directory path validation.
//!
//! Owns the [`Validator`] port, its type-erased [`ValidatorOps`] counterpart,
//! the public [`PathValidatorImpl`], and the API marker for the core validator.
//!
//! [`Validator`]: traits::validator::Validator
//! [`ValidatorOps`]: traits::validator_ops::ValidatorOps
//! [`PathValidatorImpl`]: types::path_validator_impl::PathValidatorImpl

pub mod traits;
pub mod types;

pub use traits::validator_bound::ValidatorBound;
