//! Integration tests for `FeatureSummaryOps` — `enabled_count`, `disabled_count`, `total_count`, `all_enabled`.
#![allow(missing_docs, clippy::unwrap_used)]
use swe_edge_configbuilder::{
    ConfigLoaderFactory, FeatureRegistry, FeatureRegistryOps as _, FeatureSummaryOps as _,
    OptionalSection,
};

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct Alpha { value: i32 }
impl OptionalSection for Alpha {
    fn section_name() -> &'static str { "alpha" }
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct Beta { value: i32 }
impl OptionalSection for Beta {
    fn section_name() -> &'static str { "beta" }
}

fn make_registry_with_toml(content: &str) -> FeatureRegistry {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), content).unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<Alpha>(&loader);
    let _ = reg.load::<Beta>(&loader);
    reg
}

// ── enabled_count ─────────────────────────────────────────────────────────────

#[test]
fn test_enabled_count_both_sections_present_happy() {
    let reg = make_registry_with_toml("[alpha]\n[beta]\n");
    assert_eq!(reg.summary().enabled_count(), 2);
}

#[test]
fn test_enabled_count_no_sections_present_is_zero_error() {
    let reg = make_registry_with_toml("");
    assert_eq!(reg.summary().enabled_count(), 0);
}

#[test]
fn test_enabled_count_one_of_two_sections_present_edge() {
    let reg = make_registry_with_toml("[alpha]\n");
    assert_eq!(reg.summary().enabled_count(), 1);
}

// ── disabled_count ────────────────────────────────────────────────────────────

#[test]
fn test_disabled_count_both_absent_returns_two_happy() {
    let reg = make_registry_with_toml("");
    assert_eq!(reg.summary().disabled_count(), 2);
}

#[test]
fn test_disabled_count_both_present_is_zero_error() {
    let reg = make_registry_with_toml("[alpha]\n[beta]\n");
    assert_eq!(reg.summary().disabled_count(), 0);
}

#[test]
fn test_disabled_count_plus_enabled_count_equals_total_count_edge() {
    let reg = make_registry_with_toml("[alpha]\n");
    let s = reg.summary();
    assert_eq!(s.enabled_count() + s.disabled_count(), s.total_count());
}

// ── total_count ───────────────────────────────────────────────────────────────

#[test]
fn test_total_count_reflects_number_of_loaded_features_happy() {
    let reg = make_registry_with_toml("[alpha]\n[beta]\n");
    assert_eq!(reg.summary().total_count(), 2);
}

#[test]
fn test_total_count_empty_registry_is_zero_error() {
    let reg = FeatureRegistry::new();
    assert_eq!(reg.summary().total_count(), 0);
}

#[test]
fn test_total_count_single_feature_is_one_edge() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "[alpha]\n").unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut reg = FeatureRegistry::new();
    let _ = reg.load::<Alpha>(&loader);
    assert_eq!(reg.summary().total_count(), 1);
}

// ── all_enabled ───────────────────────────────────────────────────────────────

#[test]
fn test_all_enabled_both_sections_present_returns_true_happy() {
    let reg = make_registry_with_toml("[alpha]\n[beta]\n");
    assert!(reg.summary().all_enabled());
}

#[test]
fn test_all_enabled_one_absent_returns_false_error() {
    let reg = make_registry_with_toml("[alpha]\n");
    assert!(!reg.summary().all_enabled());
}

#[test]
fn test_all_enabled_empty_registry_returns_true_edge() {
    // Vacuous truth: no features → nothing disabled.
    let reg = FeatureRegistry::new();
    assert!(reg.summary().all_enabled());
}
