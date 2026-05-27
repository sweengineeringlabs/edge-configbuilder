//! Value objects and configurations for the public API.

pub mod feature_metadata;
pub mod feature_record;
pub mod feature_state;
pub mod loaded_feature;
pub mod on_error;
pub mod override_source;
pub mod preflight_report;
pub mod substitution_policies;

pub use feature_metadata::FeatureMetadata;
pub use feature_record::FeatureRecord;
pub use feature_state::FeatureState;
pub use loaded_feature::LoadedFeature;
pub use on_error::OnError;
pub use override_source::OverrideSource;
pub use preflight_report::{PreflightIssue, PreflightIssueKind, PreflightReport};
pub use substitution_policies::{
    AllowAllPolicy, CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy,
};
