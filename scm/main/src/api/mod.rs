//! API contract surface — one subdir per theme (ADR-007), plus flat internal
//! re-exports so implementation layers do not depend on theme internals.

mod configbuilder;
mod loader;
mod preflight;
mod substitution;
mod validator;

pub use configbuilder::traits::builder_finalizer::BuilderFinalizer;
pub use configbuilder::traits::config_builder::ConfigBuilder;
pub use configbuilder::traits::config_builder_bound::ConfigBuilderBound;
pub use configbuilder::traits::config_builder_init::ConfigBuilderInit;
pub use configbuilder::vo::application_config::ApplicationConfig;
pub use configbuilder::vo::substitution_config_builder_impl::SubstitutionConfigBuilderImpl;
pub use configbuilder::vo::ConfigBuilderImpl;
pub use loader::dto::loaded_feature::LoadedFeature;
pub use loader::dto::raw_feature::RawFeature;
pub use loader::errors::config_error::ConfigError;
pub use loader::traits::config_section::ConfigSection;
pub use loader::traits::feature_loader::FeatureLoader;
pub use loader::traits::feature_record_builder_ops::FeatureRecordBuilderOps;
pub use loader::traits::feature_registry_ops::FeatureRegistryOps;
pub use loader::traits::feature_state_ops::FeatureStateOps;
pub use loader::traits::feature_summary_ops::FeatureSummaryOps;
pub use loader::traits::loader::Loader;
pub use loader::traits::loader_ops::LoaderOps;
pub use loader::traits::optional_section::OptionalSection;
pub use loader::traits::section_loader_bound::SectionLoaderBound;
pub use loader::traits::topology_ops::TopologyOps;
pub use loader::vo::feature_metadata::FeatureMetadata;
pub use loader::vo::feature_record::FeatureRecord;
pub use loader::vo::feature_record_builder::FeatureRecordBuilder;
pub use loader::vo::feature_registry::FeatureRegistry;
pub use loader::vo::feature_state::FeatureState;
pub use loader::vo::feature_summary::FeatureSummary;
pub use loader::vo::on_error::OnError;
pub use loader::vo::override_source::OverrideSource;
pub use loader::vo::section_loader_impl::SectionLoaderImpl;
pub use loader::vo::topology::Topology;
pub use preflight::traits::preflight::Preflight;
pub use preflight::traits::preflight_issue_kind_ops::PreflightIssueKindOps;
pub use preflight::traits::preflight_report_ops::PreflightReportOps;
pub use preflight::vo::{PreflightIssue, PreflightIssueKind, PreflightReport};
pub use substitution::error::substitution_error::SubstitutionError;
pub use substitution::substituter::Substituter as SubstituterBound;
pub use substitution::traits::policy_catalog::PolicyCatalog;
pub use substitution::traits::substitution_policy::SubstitutionPolicy;
pub use substitution::traits::value_resolver::ValueResolver;
#[cfg(any(test, feature = "test-utils"))]
pub use substitution::vo::AllowAllPolicy;
pub use substitution::vo::{
    CompositePolicy, EnvValueResolver, PatternWhitelistPolicy, PrefixWhitelistPolicy,
};
pub use validator::errors::validator_error::ValidatorError;
pub use validator::traits::validator::Validator;
pub use validator::traits::validator_bound::ValidatorBound;
pub use validator::traits::validator_ops::ValidatorOps;
pub use validator::vo::path_validator_impl::PathValidatorImpl;
