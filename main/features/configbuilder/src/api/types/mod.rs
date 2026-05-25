//! Value objects and configurations for the public API.

pub mod substitution_policies;

pub use substitution_policies::{
    AllowAllPolicy, CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy,
};
