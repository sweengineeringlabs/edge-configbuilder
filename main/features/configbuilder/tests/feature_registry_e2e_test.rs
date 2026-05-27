//! End-to-end tests for `FeatureRegistry` and `FeatureSummary`.

use swe_edge_configbuilder::{
    create_loader_for_dir, ConfigError, FeatureRegistry, FeatureState, OptionalSection,
};
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    std::fs::write(dir.join("application.toml"), content).unwrap();
}

// ── test structs ──────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, PartialEq)]
struct CacheConfig {
    url: String,
    #[serde(default)]
    ttl_secs: u32,
}

impl OptionalSection for CacheConfig {
    fn section_name() -> &'static str {
        "cache"
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct BrokerConfig {
    host: String,
    port: u16,
    #[serde(default)]
    tls_enabled: bool,
    cert_path: Option<String>,
}

impl OptionalSection for BrokerConfig {
    fn section_name() -> &'static str {
        "message_broker"
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        if self.tls_enabled && self.cert_path.is_none() {
            return Err(ConfigError::validation(
                Self::section_name(),
                "cert_path is required when tls_enabled = true",
            ));
        }
        Ok(())
    }
}

// ── empty registry ────────────────────────────────────────────────────────────

#[test]
fn test_feature_registry_new_has_no_records() {
    let registry = FeatureRegistry::new();
    assert_eq!(registry.records().len(), 0);
}

#[test]
fn test_feature_registry_summary_empty_shows_zero_of_zero() {
    let registry = FeatureRegistry::new();
    let summary = registry.summary();
    assert_eq!(summary.total_count(), 0);
    assert_eq!(summary.enabled_count(), 0);
    assert_eq!(summary.disabled_count(), 0);
    assert!(summary.all_enabled(), "vacuously all enabled when empty");
}

// ── single feature enabled ────────────────────────────────────────────────────

#[test]
fn test_feature_registry_load_enabled_section_returns_enabled_and_records_it() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let state: FeatureState<CacheConfig> = registry.load(&loader).unwrap();

    assert!(state.is_enabled());
    assert_eq!(registry.records().len(), 1);
    assert!(registry.records()[0].enabled);
    assert_eq!(registry.records()[0].section_name, "cache");
}

#[test]
fn test_feature_registry_summary_one_enabled_shows_one_of_one() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();

    let summary = registry.summary();
    assert_eq!(summary.total_count(), 1);
    assert_eq!(summary.enabled_count(), 1);
    assert_eq!(summary.disabled_count(), 0);
    assert!(summary.all_enabled());
}

// ── single feature disabled ───────────────────────────────────────────────────

#[test]
fn test_feature_registry_load_absent_section_returns_disabled_and_records_it() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other]\nkey = \"x\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let state: FeatureState<CacheConfig> = registry.load(&loader).unwrap();

    assert!(state.is_disabled());
    assert_eq!(registry.records().len(), 1);
    assert!(!registry.records()[0].enabled);
    assert_eq!(registry.records()[0].section_name, "cache");
}

#[test]
fn test_feature_registry_summary_one_disabled_shows_zero_of_one() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other]\nkey = \"x\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();

    let summary = registry.summary();
    assert_eq!(summary.total_count(), 1);
    assert_eq!(summary.enabled_count(), 0);
    assert_eq!(summary.disabled_count(), 1);
    assert!(!summary.all_enabled());
}

// ── multiple features — mixed ─────────────────────────────────────────────────

#[test]
fn test_feature_registry_load_multiple_features_records_all_in_order() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[cache]\nurl = \"redis://localhost\"\n\n[message_broker]\nhost = \"mq.local\"\nport = 5672",
    );
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let cache_state: FeatureState<CacheConfig> = registry.load(&loader).unwrap();
    let broker_state: FeatureState<BrokerConfig> = registry.load(&loader).unwrap();

    assert!(cache_state.is_enabled());
    assert!(broker_state.is_enabled());
    assert_eq!(registry.records().len(), 2);
    assert_eq!(registry.records()[0].section_name, "cache");
    assert_eq!(registry.records()[1].section_name, "message_broker");
}

#[test]
fn test_feature_registry_summary_mixed_shows_correct_counts() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();

    let summary = registry.summary();
    assert_eq!(summary.total_count(), 2);
    assert_eq!(summary.enabled_count(), 1);
    assert_eq!(summary.disabled_count(), 1);
    assert!(!summary.all_enabled());
}

// ── validation propagation ────────────────────────────────────────────────────

#[test]
fn test_feature_registry_load_propagates_validation_error() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq.local\"\nport = 5672\ntls_enabled = true",
    );
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let result = registry.load::<BrokerConfig>(&loader);
    assert!(
        matches!(result, Err(ConfigError::Validation { .. })),
        "registry.load must propagate validate_enabled errors, got {result:?}"
    );
}

#[test]
fn test_feature_registry_no_record_stored_on_validation_failure() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq.local\"\nport = 5672\ntls_enabled = true",
    );
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let _ = registry.load::<BrokerConfig>(&loader);
    assert_eq!(
        registry.records().len(),
        0,
        "record must not be stored when validate_enabled rejects the section"
    );
}

// ── Display format ────────────────────────────────────────────────────────────

#[test]
fn test_feature_summary_display_lists_section_names() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(output.contains("cache"), "summary must mention 'cache'");
    assert!(
        output.contains("message_broker"),
        "summary must mention 'message_broker'"
    );
}

#[test]
fn test_feature_summary_display_shows_on_off_status() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(output.contains("ON"), "enabled feature must appear as ON");
    assert!(
        output.contains("OFF"),
        "disabled feature must appear as OFF"
    );
}

#[test]
fn test_feature_summary_display_shows_total_counts() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[cache]\nurl = \"redis://localhost\"\n[message_broker]\nhost = \"mq\"\nport = 5672",
    );
    let loader = create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(
        output.contains("2/2"),
        "summary header must show '2/2 enabled', got: {output}"
    );
}
