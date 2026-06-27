use std::path::PathBuf;
use std::time::Duration;

use crate::api::{
    CompositePolicy, ConfigBuilderImpl, ConfigError, FeatureMetadata,
    FeatureRecord, FeatureRecordBuilder, FeatureRegistry, FeatureState, FeatureSummary,
    LoadedFeature, OnError, OptionalSection, OverrideSource, PathValidatorImpl,
    PatternWhitelistPolicy, PrefixWhitelistPolicy, PreflightIssue, PreflightIssueKind,
    PreflightReport, SectionLoaderImpl, SubstitutionConfigBuilderImpl, SubstitutionPolicy,
    Topology,
};

// All impl blocks have been moved to their respective core/ type files.
// This file is now empty and can be removed.
