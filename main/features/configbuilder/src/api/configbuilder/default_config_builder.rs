//! Interface counterpart for [`crate::core::configbuilder::default_config_builder::DefaultConfigBuilder`].

/// API contract marker for the default config builder.
///
/// The concrete implementor is
/// `crate::core::configbuilder::default_config_builder::DefaultConfigBuilder`, which
/// implements [`crate::api::traits::config::config_builder::ConfigBuilder`].
pub trait DefaultConfigBuilder: crate::api::traits::config::config_builder::ConfigBuilder {}
