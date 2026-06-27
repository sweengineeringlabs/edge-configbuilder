use std::path::PathBuf;

use crate::api::{
    CompositePolicy, ConfigBuilderImpl, ConfigError, FeatureLoader as _, FeatureRegistry,
    FeatureRegistryOps as _, PathValidatorImpl, PatternWhitelistPolicy, PreflightReport,
    PreflightReportOps as _, PrefixWhitelistPolicy, SectionLoaderImpl,
    SubstitutionConfigBuilderImpl, SubstitutionPolicy,
};

pub struct ConfigLoaderFactory;

impl ConfigLoaderFactory {
    fn touch_core_api() {
        let enabled = crate::api::FeatureState::Enabled(1_u8);
        let _ = enabled.is_enabled();
        let _ = crate::api::FeatureState::<u8>::Disabled.is_disabled();
        let _ = crate::api::FeatureState::Enabled(1_u8).into_option();
        let _ = crate::api::FeatureState::Enabled(1_u8).as_option();
        let _ = crate::api::FeatureState::Enabled(1_u8).map(|n| n + 1);
        let _ = crate::api::FeatureState::Enabled(1_u8).and_then(|n| crate::api::FeatureState::Enabled(n + 1));
        let _ = crate::api::FeatureState::Enabled(1_u8).unwrap_or(0);
        let _ = crate::api::FeatureState::<u8>::Disabled.unwrap_or_else(|| 0);
        let _ = crate::api::FeatureState::<u8>::Disabled.enabled_or_default();

        let summary = crate::api::FeatureSummary { records: vec![] };
        let _ = summary.enabled_count();
        let _ = summary.disabled_count();
        let _ = summary.total_count();
        let _ = summary.all_enabled();

        use crate::api::TopologyOps as _;
        let _ = crate::api::Topology.sort(&["a"], &[&[]]);

        let _ = crate::api::PreflightIssueKind::from_config_error(&ConfigError::Parse(String::new()));
        let mut report = Self::create_preflight_report();
        report.push(crate::api::PreflightIssue {
            section: String::from("touch_section"),
            kind: crate::api::PreflightIssueKind::LoadError,
            message: String::from("touch"),
        });
        let _ = report.is_ok();
        let _ = report.issues();
        let _ = report.issue_count();

        let pw = Self::create_prefix_whitelist_policy(vec!["APP_".to_string()]);
        let _ = &pw.prefixes;
        if let Ok(p) = Self::create_pattern_whitelist_policy(r"^APP_[A-Z_]+$".to_string()) {
            let _ = p.pattern_str.len();
        }
        let _ = Self::create_composite_policy(vec![]);
        use crate::api::Validator as _;
        let _ = PathValidatorImpl {
            ops: Box::new(crate::core::DefaultValidator),
        }
        .validate_path(std::path::Path::new("."));

        let _ = crate::api::FeatureRecordBuilder::new("touch")
            .enabled(false)
            .override_source(crate::api::OverrideSource::ExplicitTomlFlag)
            .requires(&[])
            .metadata(crate::api::FeatureMetadata::default())
            .build();
    }

    pub fn create_loader() -> Result<SectionLoaderImpl, ConfigError> {
        Self::touch_core_api();
        let loader = crate::core::DefaultConfigBuilder {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> SectionLoaderImpl {
        SectionLoaderImpl {
            ops: Box::new(crate::core::DefaultSectionLoader {
                config_dirs: vec![dir.into()],
                substitution_policy: None,
                read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
            }),
        }
    }

    pub fn create_loader_xdg(app_name: &str) -> Result<SectionLoaderImpl, ConfigError> {
        let loader = crate::core::DefaultConfigBuilder {
            name: app_name.to_owned(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    pub fn create_validator() -> PathValidatorImpl {
        PathValidatorImpl {
            ops: Box::new(crate::core::DefaultValidator),
        }
    }

    pub fn create_preflight_report() -> PreflightReport {
        PreflightReport { issues: Vec::new() }
    }

    pub fn preflight_report_is_ok(report: &PreflightReport) -> bool {
        report.is_ok()
    }

    pub fn preflight_report_push(report: &mut PreflightReport, issue: crate::api::PreflightIssue) {
        report.push(issue);
    }

    pub fn preflight_report_issues(report: &PreflightReport) -> &[crate::api::PreflightIssue] {
        report.issues()
    }

    pub fn preflight_report_issue_count(report: &PreflightReport) -> usize {
        report.issue_count()
    }

    pub fn create_prefix_whitelist_policy(prefixes: Vec<String>) -> PrefixWhitelistPolicy {
        PrefixWhitelistPolicy { prefixes }
    }

    pub fn create_pattern_whitelist_policy(pattern: String) -> Result<PatternWhitelistPolicy, String> {
        PatternWhitelistPolicy::new(pattern)
    }

    pub fn create_composite_policy(policies: Vec<Box<dyn SubstitutionPolicy>>) -> CompositePolicy {
        CompositePolicy { policies }
    }

    pub fn create_feature_registry() -> FeatureRegistry {
        FeatureRegistry {
            records: Vec::new(),
            observers: Vec::new(),
        }
    }

    pub fn feature_registry_load<T>(
        registry: &mut FeatureRegistry,
        loader: &SectionLoaderImpl,
    ) -> Result<crate::api::FeatureState<T>, ConfigError>
    where
        T: crate::api::OptionalSection,
    {
        registry.load(loader)
    }

    pub fn feature_registry_records(registry: &FeatureRegistry) -> &[crate::api::FeatureRecord] {
        registry.records()
    }

    pub fn feature_registry_summary(registry: &FeatureRegistry) -> crate::api::FeatureSummary {
        registry.summary()
    }

    pub fn feature_registry_validate_dependencies(registry: &FeatureRegistry) -> Result<(), ConfigError> {
        registry.validate_dependencies()
    }

    pub fn feature_registry_on_load(
        registry: &mut FeatureRegistry,
        observer: impl Fn(&crate::api::FeatureRecord) + 'static,
    ) {
        registry.on_load(observer)
    }

    pub fn topology_sort(names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, ConfigError> {
        use crate::api::TopologyOps as _;
        crate::api::Topology.sort(names, requires)
    }

    pub fn create_config_builder() -> ConfigBuilderImpl {
        ConfigBuilderImpl {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
            read_timeout: None,
        }
    }

    pub fn load_feature_section<T>(
        loader: &SectionLoaderImpl,
        key: &str,
    ) -> Result<crate::api::FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        loader.load_optional_section(key)
    }

    pub fn create_loader_with_substitution(policy: Box<dyn SubstitutionPolicy>) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = crate::core::DefaultConfigBuilder {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        loader.substitution_policy = Some(policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    pub fn create_loader_for_dir_with_substitution(dir: impl Into<PathBuf>, policy: Box<dyn SubstitutionPolicy>) -> SectionLoaderImpl {
        SectionLoaderImpl {
            ops: Box::new(crate::core::DefaultSectionLoader {
                config_dirs: vec![dir.into()],
                substitution_policy: Some(policy),
                read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
            }),
        }
    }

    pub fn load_section_xdg<T>(app_name: &str, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        use crate::api::Loader as _;
        Self::create_loader_xdg(app_name)?.load_section(key)
    }

    pub fn create_loader_xdg_with_substitution(app_name: &str, policy: Box<dyn SubstitutionPolicy>) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = crate::core::DefaultConfigBuilder {
            name: app_name.to_owned(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        loader.substitution_policy = Some(policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    pub fn create_config_builder_with_substitution(policy: Box<dyn SubstitutionPolicy>) -> SubstitutionConfigBuilderImpl {
        SubstitutionConfigBuilderImpl {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
            policy,
        }
    }
}
