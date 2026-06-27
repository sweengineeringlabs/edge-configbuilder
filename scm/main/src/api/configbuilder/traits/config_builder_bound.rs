//! Interface counterpart for [`crate::core::configbuilder::default_config_builder::DefaultConfigBuilder`].

use crate::api::{
    ApplicationConfig, ConfigBuilderImpl, SubstitutionConfigBuilderImpl,
};

/// API contract marker for the default config builder.
///
/// The concrete implementor is
/// `crate::core::configbuilder::default_config_builder::DefaultConfigBuilder`.
pub trait ConfigBuilderBound {
    /// Root application config type consumed by the builder.
    type ApplicationConfig: Into<ApplicationConfig>;

    /// Public fluent builder type produced by the factory.
    type Builder: Into<ConfigBuilderImpl>;

    /// Public fluent builder type with substitution policy attached.
    type SubstitutionBuilder: Into<SubstitutionConfigBuilderImpl>;
}
