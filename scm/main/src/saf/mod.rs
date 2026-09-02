//! SAF facade for consumer-facing access to configbuilder services.

mod allow_all_policy_saf;
mod application_config_saf;
mod builder_finalizer_saf_factory;
mod composite_policy_saf;
mod config;
mod config_builder_bound_saf_factory;
mod config_builder_init_saf_factory;
mod config_builder_saf;
mod config_builder_saf_factory;
mod config_error_saf;
mod config_section_saf_factory;
mod configbuilder;
mod default_substitution_config_builder_saf;
mod env_value_resolver_saf;
mod feature;
mod feature_loader_saf_factory;
mod feature_metadata_saf;
mod feature_record_builder_ops_saf_factory;
mod feature_record_builder_saf;
mod feature_record_saf;
mod feature_registry_ops_saf_factory;
mod feature_registry_saf;
mod feature_state_ops_saf_factory;
mod feature_state_saf;
mod feature_summary_ops_saf_factory;
mod feature_summary_saf;
mod loaded_feature_saf;
mod loader;
mod loader_ops_saf_factory;
mod loader_saf_factory;
mod on_error_saf;
mod optional;
mod optional_section_saf_factory;
mod override_source_saf;
mod path_validator_saf;
mod pattern_whitelist_policy_saf;
mod policy;
mod policy_catalog_saf_factory;
mod prefix_whitelist_policy_saf;
mod preflight;
mod preflight_issue_kind_ops_saf_factory;
mod preflight_issue_kind_saf;
mod preflight_issue_saf;
mod preflight_report_ops_saf_factory;
mod preflight_report_saf;
mod preflight_saf_factory;
mod section;
mod section_loader_bound_saf_factory;
mod section_loader_saf;
mod substituter;
mod substituter_saf_factory;
mod substitution;
mod substitution_error_saf;
mod substitution_policy_saf_factory;
mod topology_ops_saf_factory;
mod topology_saf;
mod validator;
mod validator_bound_saf_factory;
mod validator_error_saf;
mod validator_ops_saf_factory;
mod validator_saf_factory;
mod value_resolver_saf_factory;

pub use builder_finalizer_saf_factory::BuilderFinalizer;
pub use config_builder_init_saf_factory::ConfigBuilderInit;
pub use config_builder_saf::ConfigBuilderImpl;
pub use configbuilder::ConfigLoaderFactory;
pub use default_substitution_config_builder_saf::DefaultSubstitutionConfigBuilder;
pub use feature_record_builder_ops_saf_factory::FeatureRecordBuilderOps;
pub use feature_registry_ops_saf_factory::FeatureRegistryOps;
pub use feature_state_ops_saf_factory::FeatureStateOps;
pub use feature_summary_ops_saf_factory::FeatureSummaryOps;
pub use path_validator_saf::PathValidatorImpl;
pub use preflight_issue_kind_ops_saf_factory::PreflightIssueKindOps;
pub use preflight_report_ops_saf_factory::PreflightReportOps;
pub use section_loader_saf::SectionLoaderImpl;
pub use topology_ops_saf_factory::TopologyOps;

#[cfg(any(test, feature = "test-utils"))]
pub use allow_all_policy_saf::AllowAllPolicy;
pub use application_config_saf::ApplicationConfig;
pub use composite_policy_saf::CompositePolicy;
pub use config_builder_saf_factory::ConfigBuilder;
pub use config_error_saf::ConfigError;
pub use config_section_saf_factory::ConfigSection;
pub use env_value_resolver_saf::EnvValueResolver;
pub use feature_loader_saf_factory::FeatureLoader;
pub use feature_metadata_saf::FeatureMetadata;
pub use feature_record_builder_saf::FeatureRecordBuilder;
pub use feature_record_saf::FeatureRecord;
pub use feature_registry_saf::FeatureRegistry;
pub use feature_state_saf::FeatureState;
pub use feature_summary_saf::FeatureSummary;
pub use loaded_feature_saf::LoadedFeature;
pub use loader_saf_factory::Loader;
pub use on_error_saf::OnError;
pub use optional_section_saf_factory::OptionalSection;
pub use override_source_saf::OverrideSource;
pub use pattern_whitelist_policy_saf::PatternWhitelistPolicy;
pub use prefix_whitelist_policy_saf::PrefixWhitelistPolicy;
pub use preflight_issue_kind_saf::PreflightIssueKind;
pub use preflight_issue_saf::PreflightIssue;
pub use preflight_report_saf::PreflightReport;
pub use preflight_saf_factory::Preflight;
pub use substitution_error_saf::SubstitutionError;
pub use substitution_policy_saf_factory::SubstitutionPolicy;
pub use validator_error_saf::ValidatorError;
pub use validator_saf_factory::Validator;
pub use value_resolver_saf_factory::ValueResolver;

#[doc(hidden)]
pub use config_builder_bound_saf_factory::ConfigBuilderBound;
#[doc(hidden)]
pub use policy_catalog_saf_factory::PolicyCatalog;
#[doc(hidden)]
pub use section_loader_bound_saf_factory::SectionLoaderBound;
#[doc(hidden)]
pub use substituter_saf_factory::SubstituterBound;
#[doc(hidden)]
pub use topology_saf::Topology;
#[doc(hidden)]
pub use validator_bound_saf_factory::ValidatorBound;

#[doc(hidden)]
pub use builder_finalizer_saf_factory::BUILDER_FINALIZER_SVC_FACTORY;
#[doc(hidden)]
pub use config::CONFIG_BUILDER_SVC;
#[doc(hidden)]
pub use config_builder_bound_saf_factory::CONFIG_BUILDER_BOUND_SVC_FACTORY;
#[doc(hidden)]
pub use config_builder_init_saf_factory::CONFIG_BUILDER_INIT_SVC_FACTORY;
#[doc(hidden)]
pub use config_builder_saf_factory::CONFIG_BUILDER_SVC_FACTORY;
#[doc(hidden)]
pub use config_section_saf_factory::CONFIG_SECTION_SVC_FACTORY;
#[doc(hidden)]
pub use feature_loader_saf_factory::FEATURE_LOADER_SVC_FACTORY;
#[doc(hidden)]
pub use feature_record_builder_ops_saf_factory::FEATURE_RECORD_BUILDER_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use feature_registry_ops_saf_factory::FEATURE_REGISTRY_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use feature_state_ops_saf_factory::FEATURE_STATE_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use feature_summary_ops_saf_factory::FEATURE_SUMMARY_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use loader_ops_saf_factory::LOADER_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use loader_saf_factory::LOADER_SVC_FACTORY;
#[doc(hidden)]
pub use optional_section_saf_factory::OPTIONAL_SECTION_SVC_FACTORY;
#[doc(hidden)]
pub use policy_catalog_saf_factory::POLICY_CATALOG_SVC_FACTORY;
#[doc(hidden)]
pub use preflight_issue_kind_ops_saf_factory::PREFLIGHT_ISSUE_KIND_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use preflight_report_ops_saf_factory::PREFLIGHT_REPORT_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use preflight_saf_factory::PREFLIGHT_SVC_FACTORY;
#[doc(hidden)]
pub use section_loader_bound_saf_factory::SECTION_LOADER_BOUND_SVC_FACTORY;
#[doc(hidden)]
pub use substituter_saf_factory::SUBSTITUTER_SVC_FACTORY;
#[doc(hidden)]
pub use substitution_policy_saf_factory::SUBSTITUTION_POLICY_SVC_FACTORY;
#[doc(hidden)]
pub use topology_ops_saf_factory::TOPOLOGY_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use validator_bound_saf_factory::VALIDATOR_BOUND_SVC_FACTORY;
#[doc(hidden)]
pub use validator_ops_saf_factory::VALIDATOR_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use validator_saf_factory::VALIDATOR_SVC_FACTORY;
#[doc(hidden)]
pub use value_resolver_saf_factory::VALUE_RESOLVER_SVC_FACTORY;
