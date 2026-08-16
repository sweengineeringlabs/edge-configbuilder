//! SAF facade for consumer-facing access to configbuilder services.

mod allow_all_policy_svc;
mod application_config_svc;
mod builder_finalizer_svc_factory;
mod composite_policy_svc;
mod config;
mod config_builder_bound_svc_factory;
mod config_builder_impl_svc;
mod config_builder_init_svc_factory;
mod config_builder_svc_factory;
mod config_error_svc;
mod config_section_svc_factory;
mod configbuilder;
mod env_value_resolver_svc;
mod feature;
mod feature_loader_svc_factory;
mod feature_metadata_svc;
mod feature_record_builder_ops_svc_factory;
mod feature_record_builder_svc;
mod feature_record_svc;
mod feature_registry_ops_svc_factory;
mod feature_registry_svc;
mod feature_state_ops_svc_factory;
mod feature_state_svc;
mod feature_summary_ops_svc_factory;
mod feature_summary_svc;
mod loaded_feature_svc;
mod loader;
mod loader_ops_svc_factory;
mod loader_svc_factory;
mod on_error_svc;
mod optional;
mod optional_section_svc_factory;
mod override_source_svc;
mod path_validator_impl_svc;
mod pattern_whitelist_policy_svc;
mod policy;
mod policy_catalog_svc_factory;
mod prefix_whitelist_policy_svc;
mod preflight;
mod preflight_issue_kind_ops_svc_factory;
mod preflight_issue_kind_svc;
mod preflight_issue_svc;
mod preflight_report_ops_svc_factory;
mod preflight_report_svc;
mod preflight_svc_factory;
mod section;
mod section_loader_bound_svc_factory;
mod section_loader_impl_svc;
mod substituter;
mod substituter_svc_factory;
mod substitution;
mod substitution_config_builder_impl_svc;
mod substitution_error_svc;
mod substitution_policy_svc_factory;
mod topology_ops_svc_factory;
mod topology_svc;
mod validator;
mod validator_bound_svc_factory;
mod validator_error_svc;
mod validator_ops_svc_factory;
mod validator_svc_factory;
mod value_resolver_svc_factory;

pub use builder_finalizer_svc_factory::BuilderFinalizer;
pub use config_builder_impl_svc::ConfigBuilderImpl;
pub use config_builder_init_svc_factory::ConfigBuilderInit;
pub use configbuilder::ConfigLoaderFactory;
pub use feature_record_builder_ops_svc_factory::FeatureRecordBuilderOps;
pub use feature_registry_ops_svc_factory::FeatureRegistryOps;
pub use feature_state_ops_svc_factory::FeatureStateOps;
pub use feature_summary_ops_svc_factory::FeatureSummaryOps;
pub use path_validator_impl_svc::PathValidatorImpl;
pub use preflight_issue_kind_ops_svc_factory::PreflightIssueKindOps;
pub use preflight_report_ops_svc_factory::PreflightReportOps;
pub use section_loader_impl_svc::SectionLoaderImpl;
pub use topology_ops_svc_factory::TopologyOps;

#[cfg(any(test, feature = "test-utils"))]
pub use allow_all_policy_svc::AllowAllPolicy;
pub use application_config_svc::ApplicationConfig;
pub use composite_policy_svc::CompositePolicy;
pub use config_builder_svc_factory::ConfigBuilder;
pub use config_error_svc::ConfigError;
pub use config_section_svc_factory::ConfigSection;
pub use env_value_resolver_svc::EnvValueResolver;
pub use feature_loader_svc_factory::FeatureLoader;
pub use feature_metadata_svc::FeatureMetadata;
pub use feature_record_builder_svc::FeatureRecordBuilder;
pub use feature_record_svc::FeatureRecord;
pub use feature_registry_svc::FeatureRegistry;
pub use feature_state_svc::FeatureState;
pub use feature_summary_svc::FeatureSummary;
pub use loaded_feature_svc::LoadedFeature;
pub use loader_svc_factory::Loader;
pub use on_error_svc::OnError;
pub use optional_section_svc_factory::OptionalSection;
pub use override_source_svc::OverrideSource;
pub use pattern_whitelist_policy_svc::PatternWhitelistPolicy;
pub use prefix_whitelist_policy_svc::PrefixWhitelistPolicy;
pub use preflight_issue_kind_svc::PreflightIssueKind;
pub use preflight_issue_svc::PreflightIssue;
pub use preflight_report_svc::PreflightReport;
pub use preflight_svc_factory::Preflight;
pub use substitution_config_builder_impl_svc::SubstitutionConfigBuilderImpl;
pub use substitution_error_svc::SubstitutionError;
pub use substitution_policy_svc_factory::SubstitutionPolicy;
pub use validator_error_svc::ValidatorError;
pub use validator_svc_factory::Validator;
pub use value_resolver_svc_factory::ValueResolver;

#[doc(hidden)]
pub use config_builder_bound_svc_factory::ConfigBuilderBound;
#[doc(hidden)]
pub use policy_catalog_svc_factory::PolicyCatalog;
#[doc(hidden)]
pub use section_loader_bound_svc_factory::SectionLoaderBound;
#[doc(hidden)]
pub use substituter_svc_factory::SubstituterBound;
#[doc(hidden)]
pub use topology_svc::Topology;
#[doc(hidden)]
pub use validator_bound_svc_factory::ValidatorBound;

#[doc(hidden)]
pub use builder_finalizer_svc_factory::BUILDER_FINALIZER_SVC_FACTORY;
#[doc(hidden)]
pub use config::CONFIG_BUILDER_SVC;
#[doc(hidden)]
pub use config_builder_bound_svc_factory::CONFIG_BUILDER_BOUND_SVC_FACTORY;
#[doc(hidden)]
pub use config_builder_init_svc_factory::CONFIG_BUILDER_INIT_SVC_FACTORY;
#[doc(hidden)]
pub use config_builder_svc_factory::CONFIG_BUILDER_SVC_FACTORY;
#[doc(hidden)]
pub use config_section_svc_factory::CONFIG_SECTION_SVC_FACTORY;
#[doc(hidden)]
pub use feature_loader_svc_factory::FEATURE_LOADER_SVC_FACTORY;
#[doc(hidden)]
pub use feature_record_builder_ops_svc_factory::FEATURE_RECORD_BUILDER_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use feature_registry_ops_svc_factory::FEATURE_REGISTRY_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use feature_state_ops_svc_factory::FEATURE_STATE_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use feature_summary_ops_svc_factory::FEATURE_SUMMARY_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use loader_ops_svc_factory::LOADER_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use loader_svc_factory::LOADER_SVC_FACTORY;
#[doc(hidden)]
pub use optional_section_svc_factory::OPTIONAL_SECTION_SVC_FACTORY;
#[doc(hidden)]
pub use policy_catalog_svc_factory::POLICY_CATALOG_SVC_FACTORY;
#[doc(hidden)]
pub use preflight_issue_kind_ops_svc_factory::PREFLIGHT_ISSUE_KIND_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use preflight_report_ops_svc_factory::PREFLIGHT_REPORT_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use preflight_svc_factory::PREFLIGHT_SVC_FACTORY;
#[doc(hidden)]
pub use section_loader_bound_svc_factory::SECTION_LOADER_BOUND_SVC_FACTORY;
#[doc(hidden)]
pub use substituter_svc_factory::SUBSTITUTER_SVC_FACTORY;
#[doc(hidden)]
pub use substitution_policy_svc_factory::SUBSTITUTION_POLICY_SVC_FACTORY;
#[doc(hidden)]
pub use topology_ops_svc_factory::TOPOLOGY_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use validator_bound_svc_factory::VALIDATOR_BOUND_SVC_FACTORY;
#[doc(hidden)]
pub use validator_ops_svc_factory::VALIDATOR_OPS_SVC_FACTORY;
#[doc(hidden)]
pub use validator_svc_factory::VALIDATOR_SVC_FACTORY;
#[doc(hidden)]
pub use value_resolver_svc_factory::VALUE_RESOLVER_SVC_FACTORY;
