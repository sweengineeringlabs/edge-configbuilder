//! SAF facade for consumer-facing access to configbuilder services.

mod builder_finalizer_svc_factory;
mod config;
mod config_builder_bound_svc_factory;
mod config_builder_init_svc_factory;
mod config_builder_svc_factory;
mod config_section_svc_factory;
mod configbuilder;
mod feature;
mod feature_loader_svc_factory;
mod feature_record_builder_ops_svc_factory;
mod feature_registry_ops_svc_factory;
mod feature_state_ops_svc_factory;
mod feature_summary_ops_svc_factory;
mod loader;
mod loader_ops_svc_factory;
mod loader_svc_factory;
mod optional;
mod optional_section_svc_factory;
mod policy;
mod policy_catalog_svc_factory;
mod preflight;
mod preflight_issue_kind_ops_svc_factory;
mod preflight_report_ops_svc_factory;
mod preflight_svc_factory;
mod section;
mod section_loader_bound_svc_factory;
mod substituter;
mod substituter_svc_factory;
mod substitution;
mod substitution_policy_svc_factory;
mod topology_ops_svc_factory;
mod validator;
mod validator_bound_svc_factory;
mod validator_ops_svc_factory;
mod validator_svc_factory;

pub use crate::api::BuilderFinalizer;
pub use crate::api::ConfigBuilderImpl;
pub use crate::api::ConfigBuilderInit;
pub use crate::api::FeatureRecordBuilderOps;
pub use crate::api::FeatureRegistryOps;
pub use crate::api::FeatureStateOps;
pub use crate::api::FeatureSummaryOps;
pub use crate::api::PathValidatorImpl;
pub use crate::api::PreflightIssueKindOps;
pub use crate::api::PreflightReportOps;
pub use crate::api::SectionLoaderImpl;
pub use crate::api::TopologyOps;
pub use configbuilder::ConfigLoaderFactory;

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
