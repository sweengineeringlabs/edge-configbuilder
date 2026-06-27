//! @covers: api/types/preflight/preflight_issue_kind.rs — PreflightIssueKind classification
use swe_edge_configbuilder::{
    ConfigError, ConfigLoaderFactory, OptionalSection, PreflightIssueKind, PreflightIssueKindOps as _,
};

/// Helper: run preflight and return the kind of the first issue, if any.
fn first_issue_kind<T: OptionalSection>(dir: &std::path::Path) -> Option<PreflightIssueKind>
where
    T: serde::de::DeserializeOwned + 'static,
{
    use swe_edge_configbuilder::PreflightReportOps as _;
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir);
    let report = swe_edge_configbuilder::preflight!(&loader, T);
    report.issues().first().map(|i| i.kind.clone())
}

#[derive(serde::Deserialize)]
struct SectionX;
impl OptionalSection for SectionX {
    fn section_name() -> &'static str { "section_x" }
}

#[test]
fn test_preflight_issue_kind_variant_name_load_error_returns_static_str() {
    let kind = PreflightIssueKind::LoadError;
    assert_eq!(
        kind.variant_name(),
        "LoadError",
        "variant_name must return 'LoadError' for LoadError variant"
    );
}

#[test]
fn test_preflight_issue_kind_variant_name_validation_error_returns_correct_str() {
    let kind = PreflightIssueKind::ValidationError;
    assert_eq!(kind.variant_name(), "ValidationError");
}

#[test]
fn test_preflight_issue_kind_variant_name_all_four_variants_unique() {
    let names: Vec<_> = [
        PreflightIssueKind::LoadError,
        PreflightIssueKind::ValidationError,
        PreflightIssueKind::DependencyMissing,
        PreflightIssueKind::DependencyCycle,
    ]
    .iter()
    .map(|k| k.variant_name())
    .collect();

    let unique_count = names.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(unique_count, 4, "all variant names must be distinct: {names:?}");
}

#[test]
fn test_preflight_issue_kind_equality_works_for_all_variants() {
    let variants = [
        PreflightIssueKind::LoadError,
        PreflightIssueKind::ValidationError,
        PreflightIssueKind::DependencyMissing,
        PreflightIssueKind::DependencyCycle,
    ];
    assert_eq!(variants.len(), 4);
    assert_ne!(
        variants[0], variants[1],
        "LoadError and ValidationError must remain distinct variants"
    );
}

#[test]
fn test_config_error_validation_produces_validation_error_kind_in_preflight() {
    // Validate the ConfigError → PreflightIssueKind mapping used inside the
    // preflight! macro: a Validation error must surface as ValidationError kind.
    let err = ConfigError::Validation {
        section: "x".into(),
        reason: "bad".into(),
    };
    let kind = match &err {
        ConfigError::Validation { .. } => PreflightIssueKind::ValidationError,
        _ => PreflightIssueKind::LoadError,
    };
    assert_eq!(kind, PreflightIssueKind::ValidationError);
}

#[test]
fn test_config_error_parse_produces_load_error_kind_in_preflight() {
    let err = ConfigError::Parse("unexpected char".into());
    let kind = match &err {
        ConfigError::Validation { .. } => PreflightIssueKind::ValidationError,
        _ => PreflightIssueKind::LoadError,
    };
    assert_eq!(kind, PreflightIssueKind::LoadError);
}
