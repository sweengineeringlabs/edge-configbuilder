// @covers: api/types/feature_summary.rs — FeatureSummary counts and Display
use swe_edge_configbuilder::{
    create_loader_for_dir, FeatureRegistry, FeatureSummary, OptionalSection,
};

use std::io::Write as _;
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[derive(Debug, serde::Deserialize)]
struct CacheConfig {
    ttl: u32,
}

impl OptionalSection for CacheConfig {
    fn section_name() -> &'static str {
        "cache"
    }
}

#[derive(Debug, serde::Deserialize)]
struct BrokerConfig {
    url: String,
}

impl OptionalSection for BrokerConfig {
    fn section_name() -> &'static str {
        "broker"
    }
}

#[test]
fn test_feature_summary_enabled_count_counts_only_enabled_features() {
    // enabled_count() must return exactly the number of features that resolved
    // to Enabled — not total, not disabled.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nttl = 60");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();
    let summary: FeatureSummary = registry.summary();
    assert_eq!(
        summary.enabled_count(),
        1,
        "only cache is enabled; broker is absent"
    );
}

#[test]
fn test_feature_summary_disabled_count_counts_only_disabled_features() {
    // disabled_count() must count exactly the features that were absent/disabled.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nttl = 30");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();
    let summary: FeatureSummary = registry.summary();
    assert_eq!(
        summary.disabled_count(),
        1,
        "only broker is disabled; cache is present"
    );
}

#[test]
fn test_feature_summary_display_contains_feature_counts() {
    // Display output must include the enabled/total ratio so operators can
    // read the startup log without parsing structured data.
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[cache]\nttl = 10\n[broker]\nurl = \"amqp://localhost\"",
    );
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();
    let summary: FeatureSummary = registry.summary();
    let display = summary.to_string();
    assert!(
        display.contains("2/2"),
        "Display must show enabled/total; got: {display}"
    );
}

#[test]
fn test_feature_summary_all_enabled_returns_true_when_all_enabled() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[cache]\nttl = 5\n[broker]\nurl = \"amqp://localhost\"",
    );
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();
    let summary: FeatureSummary = registry.summary();
    assert!(
        summary.all_enabled(),
        "all features are present; all_enabled must be true"
    );
}

#[test]
fn test_feature_summary_all_enabled_returns_false_when_some_disabled() {
    // If even one feature is absent, all_enabled() must return false so callers
    // can use it as a strict "production-ready" gate.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[cache]\nttl = 5");
    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let mut registry = FeatureRegistry::new();
    registry.load::<CacheConfig>(&loader).unwrap();
    registry.load::<BrokerConfig>(&loader).unwrap();
    let summary: FeatureSummary = registry.summary();
    assert!(
        !summary.all_enabled(),
        "broker is absent; all_enabled must be false"
    );
}
