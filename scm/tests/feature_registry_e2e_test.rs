//! End-to-end tests for `FeatureRegistry` and `FeatureSummary`.
#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(unsafe_code)]

use std::sync::{Arc, Mutex};
use swe_edge_configbuilder::{
    ConfigError, ConfigLoaderFactory, FeatureMetadata, FeatureRegistry, FeatureState, OnError,
    OptionalSection, OverrideSource,
};

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
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
            return Err(ConfigError::Validation {
                section: Self::section_name().to_string(),
                reason: "cert_path is required when tls_enabled = true".to_string(),
            });
        }
        Ok(())
    }
}

/// Same validation rules as `BrokerConfig` but degrades gracefully on error.
#[derive(Debug, serde::Deserialize, PartialEq)]
struct BrokerConfigDisable {
    host: String,
    port: u16,
    #[serde(default)]
    tls_enabled: bool,
    cert_path: Option<String>,
}

impl OptionalSection for BrokerConfigDisable {
    fn section_name() -> &'static str {
        "broker_disable"
    }

    fn on_error() -> OnError {
        OnError::Disable
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        if self.tls_enabled && self.cert_path.is_none() {
            return Err(ConfigError::Validation {
                section: Self::section_name().to_string(),
                reason: "cert_path is required when tls_enabled = true".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize, Default, PartialEq)]
struct AnalyticsConfig {
    endpoint: String,
}

impl OptionalSection for AnalyticsConfig {
    fn section_name() -> &'static str {
        "analytics"
    }

    fn requires() -> &'static [&'static str] {
        &["cache"]
    }

    fn metadata() -> FeatureMetadata {
        FeatureMetadata {
            description: "Event analytics pipeline",
            owner: "data-team",
            deprecated_since: None,
        }
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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

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
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(
        output.contains("2/2"),
        "summary header must show '2/2 enabled', got: {output}"
    );
}

// ── validate_dependencies ─────────────────────────────────────────────────────

#[test]
fn test_validate_dependencies_returns_ok_when_no_features_loaded() {
    let registry = FeatureRegistry::new();
    assert!(registry.records().is_empty());
    assert!(matches!(registry.validate_dependencies(), Ok(())));
}

#[test]
fn test_validate_dependencies_returns_ok_when_required_feature_is_enabled() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[cache]\nurl = \"redis://localhost\"\n[analytics]\nendpoint = \"https://ingest.local\"",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<AnalyticsConfig>(&loader).unwrap();

    assert!(
        registry.validate_dependencies().is_ok(),
        "analytics requires cache, which is enabled — must be Ok"
    );
}

#[test]
fn test_validate_dependencies_returns_err_when_required_feature_is_disabled() {
    let dir = TempDir::new().unwrap();
    // cache is absent → Disabled; analytics requires cache
    write_toml(
        dir.path(),
        "[analytics]\nendpoint = \"https://ingest.local\"",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<AnalyticsConfig>(&loader).unwrap();

    let result = registry.validate_dependencies();
    assert!(
        result.is_err(),
        "analytics requires cache but cache is disabled — must be Err"
    );
    let err = result.unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("analytics") && msg.contains("cache"),
        "error must name both the dependent and the missing feature, got: {msg}"
    );
}

#[test]
fn test_validate_dependencies_returns_ok_when_dependent_feature_is_also_disabled() {
    let dir = TempDir::new().unwrap();
    // both absent: analytics is Disabled, so its `requires` constraint is ignored
    write_toml(dir.path(), "[other]\nkey = \"x\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<AnalyticsConfig>(&loader).unwrap();

    assert!(
        registry.validate_dependencies().is_ok(),
        "disabled features do not need their dependencies satisfied"
    );
}

// ── graceful degradation (OnError::Disable) ───────────────────────────────────

#[test]
fn test_feature_registry_load_disable_on_error_returns_disabled_state() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = TempDir::new().unwrap();
    // tls_enabled=true but no cert_path → validate_enabled fails
    write_toml(
        dir.path(),
        "[broker_disable]\nhost = \"mq\"\nport = 5672\ntls_enabled = true",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let result: Result<FeatureState<BrokerConfigDisable>, _> = registry.load(&loader);

    assert!(
        result.is_ok(),
        "OnError::Disable must not propagate validation errors, got {result:?}"
    );
    assert!(
        result.unwrap().is_disabled(),
        "state must be Disabled when validation fails with OnError::Disable"
    );
}

#[test]
fn test_feature_registry_load_disable_on_error_stores_record_with_validation_error_override() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[broker_disable]\nhost = \"mq\"\nport = 5672\ntls_enabled = true",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<BrokerConfigDisable>(&loader).unwrap();

    assert_eq!(registry.records().len(), 1);
    let record = &registry.records()[0];
    assert!(!record.enabled, "record.enabled must be false");
    assert!(
        matches!(
            record.override_source,
            Some(OverrideSource::ValidationError { .. })
        ),
        "override_source must be ValidationError, got {:?}",
        record.override_source
    );
}

#[test]
fn test_feature_registry_load_disable_on_error_summary_shows_off() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[broker_disable]\nhost = \"mq\"\nport = 5672\ntls_enabled = true",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<BrokerConfigDisable>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(
        output.contains("OFF"),
        "gracefully degraded feature must appear as OFF in summary, got: {output}"
    );
}

// ── env var override for on_error ─────────────────────────────────────────────

#[test]
fn test_feature_registry_env_var_fail_overrides_on_error_disable() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // SAFETY: single-threaded access guaranteed by ENV_LOCK; no other thread reads this var concurrently.
    unsafe { std::env::set_var("SWE_EDGE_FEATURE_BROKER_DISABLE_ON_ERROR", "fail") };

    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[broker_disable]\nhost = \"mq\"\nport = 5672\ntls_enabled = true",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let result = registry.load::<BrokerConfigDisable>(&loader);

    // SAFETY: restoring env state; same lock guarantees exclusivity.
    unsafe { std::env::remove_var("SWE_EDGE_FEATURE_BROKER_DISABLE_ON_ERROR") };

    assert!(
        result.is_err(),
        "env var on_error=fail must override trait on_error=Disable and propagate the error"
    );
}

// ── on_load observers ─────────────────────────────────────────────────────────

#[test]
fn test_on_load_observer_is_called_after_each_successful_load() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let names_clone = Arc::clone(&names);
    registry.on_load(move |r| names_clone.lock().unwrap().push(r.section_name.clone()));

    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();

    let captured = names.lock().unwrap();
    assert_eq!(
        *captured,
        vec!["cache", "message_broker"],
        "observer must be called once per successful load in load order"
    );
}

#[test]
fn test_on_load_observer_receives_correct_enabled_flag() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let states: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
    let states_clone = Arc::clone(&states);
    registry.on_load(move |r| states_clone.lock().unwrap().push(r.enabled));

    registry.load::<CacheConfig>(&loader).unwrap(); // present → enabled
    registry.load::<BrokerConfig>(&loader).unwrap(); // absent  → disabled

    assert_eq!(*states.lock().unwrap(), vec![true, false]);
}

#[test]
fn test_on_load_multiple_observers_all_called_in_registration_order() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nurl = \"redis://localhost\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let log: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    let log1 = Arc::clone(&log);
    let log2 = Arc::clone(&log);
    registry.on_load(move |_| log1.lock().unwrap().push(1));
    registry.on_load(move |_| log2.lock().unwrap().push(2));

    registry.load::<CacheConfig>(&loader).unwrap();

    assert_eq!(
        *log.lock().unwrap(),
        vec![1, 2],
        "both observers must fire in registration order"
    );
}

#[test]
fn test_on_load_observer_not_called_when_load_returns_err() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq\"\nport = 5672\ntls_enabled = true",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let call_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let count = Arc::clone(&call_count);
    registry.on_load(move |_| *count.lock().unwrap() += 1);

    let _ = registry.load::<BrokerConfig>(&loader); // Err(Validation{..})

    assert_eq!(
        *call_count.lock().unwrap(),
        0,
        "observer must not be called when load returns Err (no record committed)"
    );
}

// ── metadata in Display ───────────────────────────────────────────────────────

#[test]
fn test_feature_summary_display_shows_metadata_description() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[cache]\nurl = \"redis://localhost\"\n[analytics]\nendpoint = \"https://ingest.local\"",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<AnalyticsConfig>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(
        output.contains("Event analytics pipeline"),
        "summary must include feature description from metadata, got: {output}"
    );
}

#[test]
fn test_feature_summary_display_shows_metadata_owner() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[analytics]\nendpoint = \"https://ingest.local\"",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    registry.load::<AnalyticsConfig>(&loader).unwrap();

    let output = registry.summary().to_string();
    assert!(
        output.contains("data-team"),
        "summary must include owner from metadata, got: {output}"
    );
}
