//! Configbuilder theme — fluent builder + factory for constructing loaders.
//!
//! Owns the [`ConfigBuilder`] builder-chain port, the public builder/factory
//! types ([`ConfigBuilderImpl`], [`SubstitutionConfigBuilderImpl`],
//! [`ConfigLoaderFactory`]), the [`ApplicationConfig`] root type, and the API
//! marker for the core builder.
//!
//! [`ConfigBuilder`]: traits::config_builder::ConfigBuilder
//! [`ConfigBuilderImpl`]: types::config_builder_impl::ConfigBuilderImpl
//! [`SubstitutionConfigBuilderImpl`]: types::substitution_config_builder_impl::SubstitutionConfigBuilderImpl
//! [`ConfigLoaderFactory`]: types::config_loader_factory::ConfigLoaderFactory
//! [`ApplicationConfig`]: types::application_config::ApplicationConfig

pub mod traits;
pub mod types;

pub use traits::config_builder_bound::ConfigBuilderBound;
