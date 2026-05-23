//! Extension hooks for downstream consumers.
//!
//! Implement [`Loader`] or [`Validator`] to plug in a custom config source or
//! path-validation strategy. Wire it up via [`crate::saf`] factory patterns or
//! inject directly as a generic type parameter.
//!
//! [`Loader`]: crate::api::traits::loader::Loader
//! [`Validator`]: crate::api::traits::validator::Validator
