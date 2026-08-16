//! Configbuilder theme — fluent builder + factory for constructing loaders.
//!
//! Owns the [`ConfigBuilder`] builder-chain port, the public builder/factory
//! types ([`ConfigBuilderImpl`], [`SubstitutionConfigBuilderImpl`],
//! [`ConfigLoaderFactory`]), the [`ApplicationConfig`] root type, and the API
//! marker for the core builder.
//!
//! [`ConfigBuilder`]: traits::config_builder::ConfigBuilder
//! [`ConfigBuilderImpl`]: vo::config_builder_impl::ConfigBuilderImpl
//! [`SubstitutionConfigBuilderImpl`]: vo::substitution_config_builder_impl::SubstitutionConfigBuilderImpl
//! [`ConfigLoaderFactory`]: crate::ConfigLoaderFactory
//! [`ApplicationConfig`]: vo::application_config::ApplicationConfig

pub mod traits;
pub mod vo;
