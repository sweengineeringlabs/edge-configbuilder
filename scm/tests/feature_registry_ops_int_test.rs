//! Integration tests for `FeatureRegistryOps` — `new`, `on_load`, `records`, `summary`, `validate_dependencies`.
#![allow(missing_docs, clippy::unwrap_used)]
use swe_edge_configbuilder::{
    ConfigLoaderFactory, FeatureRegistry, FeatureRegistryOps as _, FeatureSummaryOps as _,
    OptionalSection,
};

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct CacheSection { ttl: u32 }
impl OptionalSection for CacheSection {
    fn section_name() -> &'static str { "cache" }
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct BrokerSection { host: String }
impl OptionalSection for BrokerSection {
    fn section_name() -> &'static str { "broker" }
    fn requires() -> &'static [&'static str] { &["cache"] }
}

// ── new ───────────────────────────────────────────────────────────────────────

#[test]
fn test_new_creates_empty_registry_happy() {
    let reg = FeatureRegistry::new();
    assert!(reg.records().is_empty());
}

#[test]
fn test_new_validate_dependencies_on_empty_registry_returns_ok_error() {
    let reg = FeatureRegistry::new();
    reg.validate_dependencies()
        .expect("empty registry must have no unsatisfied dependencies");
    assert_eq!(reg.records().len(), 0, "freshly created registry must have no records");
}

#[test]
fn test_new_summary_on_empty_registry_has_zero_totals_edge() {
    let reg = FeatureRegistry::new();
    let s = reg.summary();
    assert_eq!(s.total_count(), 0);
}

// ── on_load ───────────────────────────────────────────────────────────────────

#[test]
fn test_on_load_observer_is_called_when_feature_is_loaded_happy() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "[cache]\nttl = 30\n").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let called_clone = called.clone();
    let mut reg = FeatureRegistry::new();
    reg.on_load(move |_| called_clone.store(true, std::sync::atomic::Ordering::SeqCst));
    let _ = reg.load::<CacheSection>(&loader);
    assert!(called.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn test_on_load_observer_receives_record_with_correct_section_name_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let observed = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let observed_clone = observed.clone();
    let mut reg = FeatureRegistry::new();
    reg.on_load(move |r| *observed_clone.lock().unwrap() = r.section_name.clone());
    let _ = reg.load::<CacheSection>(&loader);
    assert_eq!(*observed.lock().unwrap(), "cache");
}

#[test]
fn test_on_load_no_observers_registered_still_succeeds_edge() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);
    assert_eq!(reg.records().len(), 1);
}

// ── records ───────────────────────────────────────────────────────────────────

#[test]
fn test_records_returns_all_loaded_features_happy() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("application.toml"),
        "[cache]\nttl = 1\n[broker]\nhost = \"localhost\"\n",
    ).unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);
    let _ = reg.load::<BrokerSection>(&loader);
    assert_eq!(reg.records().len(), 2);
}

#[test]
fn test_records_section_names_match_loaded_types_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);
    assert_eq!(reg.records()[0].section_name, "cache");
}

#[test]
fn test_records_before_any_load_is_empty_edge() {
    let reg = FeatureRegistry::new();
    assert_eq!(reg.records().len(), 0);
}

// ── summary ───────────────────────────────────────────────────────────────────

#[test]
fn test_summary_enabled_count_matches_loaded_sections_happy() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "[cache]\nttl = 5\n").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);
    let s = reg.summary();
    assert_eq!(s.enabled_count(), 1);
}

#[test]
fn test_summary_disabled_when_section_absent_from_toml_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);
    let s = reg.summary();
    assert_eq!(s.disabled_count(), 1);
}

#[test]
fn test_summary_on_empty_registry_total_count_is_zero_edge() {
    let reg = FeatureRegistry::new();
    assert_eq!(reg.summary().total_count(), 0);
}

// ── validate_dependencies ─────────────────────────────────────────────────────

#[test]
fn test_validate_dependencies_satisfied_deps_return_ok_happy() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("application.toml"),
        "[cache]\nttl = 1\n[broker]\n",
    ).unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);
    let _ = reg.load::<BrokerSection>(&loader);
    assert!(reg.validate_dependencies().is_ok());
}

#[test]
fn test_validate_dependencies_missing_dep_returns_err_error() {
    let dir = tempfile::tempdir().unwrap();
    // broker depends on cache; cache is absent (disabled) but broker is present (enabled)
    std::fs::write(
        dir.path().join("application.toml"),
        "[broker]\nhost = \"localhost\"\n",
    ).unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<CacheSection>(&loader);  // disabled — absent from TOML
    let _ = reg.load::<BrokerSection>(&loader); // enabled — requires cache
    assert!(reg.validate_dependencies().is_err());
}

#[test]
fn test_validate_dependencies_empty_registry_is_trivially_satisfied_edge() {
    let reg = FeatureRegistry::new();
    reg.validate_dependencies()
        .expect("empty registry must trivially pass dependency validation");
    assert_eq!(reg.summary().total_count(), 0);
}
