//! API contract surface — one subdir per theme (ADR-007), plus flat internal
//! re-exports so implementation layers do not depend on theme internals.

mod configbuilder;
mod loader;
mod preflight;
mod substitution;
mod validator;

pub use configbuilder::traits::config_builder::ConfigBuilder;
pub use configbuilder::traits::config_builder_bound::ConfigBuilderBound;
pub use configbuilder::types::application_config::ApplicationConfig;
pub use configbuilder::types::substitution_config_builder_impl::SubstitutionConfigBuilderImpl;
pub use configbuilder::types::ConfigBuilderImpl;
pub use configbuilder::types::ConfigLoaderFactory;
pub use loader::errors::config_error::ConfigError;
pub use loader::traits::config_section::ConfigSection;
pub use loader::traits::feature_loader::FeatureLoader;
pub use loader::traits::loader::Loader;
pub use loader::traits::loader_ops::LoaderOps;
pub use loader::traits::optional_section::OptionalSection;
pub use loader::traits::section_loader_bound::SectionLoaderBound;
pub use loader::types::feature_metadata::FeatureMetadata;
pub use loader::types::feature_record::FeatureRecord;
pub use loader::types::feature_record_builder::FeatureRecordBuilder;
pub use loader::types::feature_registry::FeatureRegistry;
pub use loader::types::feature_state::FeatureState;
pub use loader::types::feature_summary::FeatureSummary;
pub use loader::types::loaded_feature::LoadedFeature;
pub use loader::types::on_error::OnError;
pub use loader::types::override_source::OverrideSource;
pub use loader::types::raw_feature::RawFeature;
pub use loader::types::section_loader_impl::SectionLoaderImpl;
pub use loader::types::topology::Topology;
pub use preflight::traits::preflight::Preflight;
pub use preflight::types::{PreflightIssue, PreflightIssueKind, PreflightReport};
pub use substitution::error::substitution_error::SubstitutionError;
pub use substitution::substituter::Substituter as SubstituterBound;
pub use substitution::traits::policy_catalog::PolicyCatalog;
pub use substitution::traits::substitution_policy::SubstitutionPolicy;
#[cfg(any(test, feature = "test-utils"))]
pub use substitution::types::AllowAllPolicy;
pub use substitution::types::{CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy};
pub use validator::errors::validator_error::ValidatorError;
pub use validator::traits::validator::Validator;
pub use validator::traits::validator_bound::ValidatorBound;
pub use validator::traits::validator_ops::ValidatorOps;
pub use validator::types::path_validator_impl::PathValidatorImpl;
