//! Configbuilder theme — fluent builder + factory for constructing loaders.
//!
//! Owns the [`ConfigBuilder`] builder-chain port, the public builder/factory
//! type ([`ConfigBuilderImpl`], [`ConfigLoaderFactory`]), the
//! [`ApplicationConfig`] root type, and the API marker for the core builder.
//! The substitution-aware builder (`DefaultSubstitutionConfigBuilder`) lives
//! in `core/` and is re-exported at the crate root via `saf/`.
//!
//! [`ConfigBuilder`]: traits::config_builder::ConfigBuilder
//! [`ConfigBuilderImpl`]: vo::config_builder_impl::ConfigBuilderImpl
//! [`ConfigLoaderFactory`]: crate::ConfigLoaderFactory
//! [`ApplicationConfig`]: vo::application_config::ApplicationConfig

pub mod traits;
pub mod vo;
