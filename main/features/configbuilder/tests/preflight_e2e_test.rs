//! End-to-end tests for the `preflight!` macro.

#![allow(unsafe_code)]

use serde::Deserialize;
use swe_edge_configbuilder::{
    create_loader_for_dir, preflight, ConfigError, OnError, OptionalSection, PreflightIssueKind,
};
use tempfile::TempDir;

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn write_toml(dir: &std::path::Path, content: &str) {
    std::fs::write(dir.join("application.toml"), content).unwrap();
}

// ── test section types ────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
struct StoreSection {}
impl OptionalSection for StoreSection {
    fn section_name() -> &'static str {
        "store"
    }
}

#[derive(Deserialize, Default)]
struct IndexSection {}
impl OptionalSection for IndexSection {
    fn section_name() -> &'static str {
        "index"
    }
    fn requires() -> &'static [&'static str] {
        &["store"]
    }
}

#[derive(Deserialize)]
struct BrokenSection {
    threshold: i32,
}
impl OptionalSection for BrokenSection {
    fn section_name() -> &'static str {
        "broken"
    }
    fn validate_enabled(&self) -> Result<(), ConfigError> {
        if self.threshold < 0 {
            return Err(ConfigError::validation(
                Self::section_name(),
                "threshold must be >= 0",
            ));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct DegradedSection {
    threshold: i32,
}
impl OptionalSection for DegradedSection {
    fn section_name() -> &'static str {
        "degraded"
    }
    fn on_error() -> OnError {
        OnError::Disable
    }
    fn validate_enabled(&self) -> Result<(), ConfigError> {
        if self.threshold < 0 {
            return Err(ConfigError::validation(
                Self::section_name(),
                "threshold must be non-negative",
            ));
        }
        Ok(())
    }
}

// Cycle pair
#[derive(Deserialize, Default)]
struct CycleP;
impl OptionalSection for CycleP {
    fn section_name() -> &'static str {
        "cycle_p"
    }
    fn requires() -> &'static [&'static str] {
        &["cycle_q"]
    }
}

#[derive(Deserialize, Default)]
struct CycleQ;
impl OptionalSection for CycleQ {
    fn section_name() -> &'static str {
        "cycle_q"
    }
    fn requires() -> &'static [&'static str] {
        &["cycle_p"]
    }
}

// ── no issues ─────────────────────────────────────────────────────────────────

#[test]
fn test_preflight_all_absent_sections_returns_ok_report() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, StoreSection, IndexSection);
    assert!(
        report.is_ok(),
        "absent optional sections must not produce issues; got: {}",
        report
    );
}

#[test]
fn test_preflight_all_present_valid_returns_ok_report() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[store]\n[index]\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, StoreSection, IndexSection);
    assert!(
        report.is_ok(),
        "valid sections must produce no issues; got: {}",
        report
    );
}

// ── dependency satisfied ──────────────────────────────────────────────────────

#[test]
fn test_preflight_enabled_feature_with_satisfied_dep_returns_ok_report() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[store]\n[index]\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, IndexSection, StoreSection);
    assert!(
        report.is_ok(),
        "satisfied dependency must not be reported; got: {}",
        report
    );
    assert_eq!(report.issue_count(), 0);
}

// ── dependency missing ────────────────────────────────────────────────────────

#[test]
fn test_preflight_enabled_feature_missing_dep_reports_dependency_missing() {
    let dir = TempDir::new().unwrap();
    // index is present but store is absent → dependency violation
    write_toml(dir.path(), "[index]\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, StoreSection, IndexSection);
    assert!(!report.is_ok(), "missing dependency must be reported");
    assert!(
        report
            .issues()
            .iter()
            .any(|i| i.kind == PreflightIssueKind::DependencyMissing),
        "must contain a DependencyMissing issue; got: {}",
        report
    );
}

#[test]
fn test_preflight_missing_dep_issue_names_the_dependent_section() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[index]\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, StoreSection, IndexSection);
    let issue = report
        .issues()
        .iter()
        .find(|i| i.kind == PreflightIssueKind::DependencyMissing)
        .expect("DependencyMissing issue must be present");
    assert_eq!(
        issue.section, "index",
        "issue.section must name the dependent"
    );
    assert!(
        issue.message.contains("store"),
        "issue.message must name the missing dependency: {}",
        issue.message
    );
}

// ── validation errors ─────────────────────────────────────────────────────────

#[test]
fn test_preflight_fail_on_error_validation_reports_validation_error() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[broken]\nthreshold = -1\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, BrokenSection);
    assert!(!report.is_ok());
    assert!(
        report
            .issues()
            .iter()
            .any(|i| i.kind == PreflightIssueKind::ValidationError),
        "validation failure (OnError::Fail) must appear as ValidationError; got: {}",
        report
    );
}

#[test]
fn test_preflight_disable_on_error_validation_also_reports_validation_error() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[degraded]\nthreshold = -5\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, DegradedSection);
    assert!(!report.is_ok());
    assert!(
        report
            .issues()
            .iter()
            .any(|i| i.kind == PreflightIssueKind::ValidationError),
        "OnError::Disable degradation must also be captured as ValidationError; got: {}",
        report
    );
}

// ── cycle detection ───────────────────────────────────────────────────────────

#[test]
fn test_preflight_dependency_cycle_reports_dependency_cycle_issue() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, CycleP, CycleQ);
    assert!(!report.is_ok());
    assert!(
        report
            .issues()
            .iter()
            .any(|i| i.kind == PreflightIssueKind::DependencyCycle),
        "cycle must be reported as DependencyCycle; got: {}",
        report
    );
}

#[test]
fn test_preflight_cycle_issue_message_mentions_cycle() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, CycleP, CycleQ);
    let issue = report
        .issues()
        .iter()
        .find(|i| i.kind == PreflightIssueKind::DependencyCycle)
        .expect("DependencyCycle issue must be present");
    assert!(
        issue.message.contains("cycle"),
        "issue message must describe the cycle: {}",
        issue.message
    );
}

// ── report collects all issues (does not stop at first) ───────────────────────

#[test]
fn test_preflight_collects_all_issues_across_all_sections() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = TempDir::new().unwrap();
    // broken has invalid threshold; index is present but store is absent
    write_toml(dir.path(), "[broken]\nthreshold = -1\n[index]\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, StoreSection, BrokenSection, IndexSection);
    assert!(
        report.issue_count() >= 2,
        "must collect both the validation error and the dependency violation; got: {}",
        report
    );
}

// ── Display ───────────────────────────────────────────────────────────────────

#[test]
fn test_preflight_report_display_ok_is_readable() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, StoreSection);
    assert_eq!(report.to_string(), "preflight: OK");
}

#[test]
fn test_preflight_report_display_with_issues_is_readable() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[broken]\nthreshold = -99\n");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let report = preflight!(&loader, BrokenSection);
    let output = report.to_string();
    assert!(
        output.contains("broken"),
        "section name must appear: {output}"
    );
    assert!(
        output.contains("threshold"),
        "message must appear: {output}"
    );
}
