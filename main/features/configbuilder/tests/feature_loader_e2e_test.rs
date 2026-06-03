//! Integration tests for feature section loading with env-var overrides.
//! std::env::set_var/remove_var are unsafe in Rust ≥1.80 (multi-thread UB risk).
//! Tests that exercise the SWE_EDGE_FEATURE_* env-var override path require it;
//! ENV_LOCK serializes all such tests within this binary to prevent data races.
#![allow(unsafe_code)]
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::{ConfigError, ConfigLoaderFactory, FeatureState};
use tempfile::TempDir;

// Serialize all env-var-touching tests within this test binary.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Debug, serde::Deserialize, PartialEq)]
struct BrokerConfig {
    host: String,
    port: u16,
}

fn write_toml(dir: &std::path::Path, content: &str) {
    std::fs::write(dir.join("application.toml"), content).unwrap();
}

// ── section present ───────────────────────────────────────────────────────────

#[test]
fn test_load_feature_section_present_key_returns_enabled_with_correct_values() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"localhost\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(state.is_enabled());
    let cfg = state.into_option().unwrap();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 5672);
}

#[test]
fn test_load_feature_section_present_dotted_key_returns_enabled() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[services.broker]\nhost = \"broker.svc\"\nport = 1883",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "services.broker").unwrap();
    assert!(state.is_enabled());
    assert_eq!(state.into_option().unwrap().port, 1883);
}

// ── section absent ────────────────────────────────────────────────────────────

#[test]
fn test_load_feature_section_absent_key_in_existing_file_returns_disabled() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other_service]\nhost = \"x\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(
        state.is_disabled(),
        "expected Disabled — section key absent from TOML"
    );
}

#[test]
fn test_load_feature_section_no_toml_files_returns_disabled() {
    let dir = TempDir::new().unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(
        state.is_disabled(),
        "expected Disabled — no config files exist"
    );
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_load_feature_section_malformed_toml_returns_parse_error() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "not = [broken toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let err = ConfigLoaderFactory::load_feature_section::<BrokerConfig>(&loader, "message_broker")
        .unwrap_err();
    assert!(
        matches!(err, ConfigError::Parse(_)),
        "expected Parse for malformed TOML, got {err:?}"
    );
}

#[test]
fn test_load_feature_section_missing_required_field_returns_parse_error() {
    let dir = TempDir::new().unwrap();
    // `message_broker` section present but missing required `port` field
    write_toml(dir.path(), "[message_broker]\nhost = \"localhost\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let err = ConfigLoaderFactory::load_feature_section::<BrokerConfig>(&loader, "message_broker")
        .unwrap_err();
    assert!(
        matches!(err, ConfigError::Parse(_)),
        "expected Parse for missing required field, got {err:?}"
    );
}

#[test]
fn test_load_feature_section_oversized_file_returns_io_error() {
    let dir = TempDir::new().unwrap();
    let oversized = vec![b'#'; 1_048_577];
    std::fs::write(dir.path().join("application.toml"), &oversized).unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let err = ConfigLoaderFactory::load_feature_section::<BrokerConfig>(&loader, "message_broker")
        .unwrap_err();
    assert!(
        matches!(err, ConfigError::Io(_)),
        "expected Io for oversized file, got {err:?}"
    );
}

// ── multi-dir merge ───────────────────────────────────────────────────────────

#[test]
fn test_load_feature_section_section_only_in_one_dir_returns_enabled() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[message_broker]\nhost = \"low\"\nport = 5672");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(state.is_enabled());
    assert_eq!(state.into_option().unwrap().host, "low");
}

#[test]
fn test_load_feature_section_high_priority_dir_wins_on_key_conflict() {
    let low = TempDir::new().unwrap();
    let high = TempDir::new().unwrap();
    write_toml(
        low.path(),
        "[message_broker]\nhost = \"low-host\"\nport = 1111",
    );
    write_toml(
        high.path(),
        "[message_broker]\nhost = \"high-host\"\nport = 2222",
    );
    let loader = swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_config_dir(low.path())
        .with_config_dir(high.path())
        .build_loader()
        .unwrap();
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(state.is_enabled());
    let cfg = state.into_option().unwrap();
    assert_eq!(cfg.host, "high-host", "high-priority dir must win");
    assert_eq!(cfg.port, 2222);
}

#[test]
fn test_load_feature_section_section_absent_in_both_dirs_returns_disabled() {
    let low = TempDir::new().unwrap();
    let high = TempDir::new().unwrap();
    write_toml(low.path(), "[other]\nvalue = \"x\"");
    write_toml(high.path(), "[also_other]\nvalue = \"y\"");
    let loader = swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_config_dir(low.path())
        .with_config_dir(high.path())
        .build_loader()
        .unwrap();
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(state.is_disabled());
}

// ── enabled = false TOML flag ─────────────────────────────────────────────────

#[test]
fn test_load_feature_section_explicit_enabled_false_returns_disabled() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nenabled = false\nhost = \"localhost\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state: FeatureState<BrokerConfig> =
        ConfigLoaderFactory::load_feature_section(&loader, "message_broker").unwrap();
    assert!(
        state.is_disabled(),
        "enabled=false in TOML must disable the feature"
    );
}

#[test]
fn test_load_feature_explicit_enabled_false_record_carries_toml_flag_source() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nenabled = false\nhost = \"localhost\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let loaded = loader
        .load_feature::<BrokerConfig>("message_broker")
        .unwrap();
    assert!(!loaded.record.enabled);
    assert!(
        matches!(
            loaded.record.override_source,
            Some(swe_edge_configbuilder::OverrideSource::ExplicitTomlFlag)
        ),
        "override source must be ExplicitTomlFlag"
    );
}

// ── env-var override ──────────────────────────────────────────────────────────

#[test]
fn test_load_feature_env_var_false_disables_present_section() {
    let _g = ENV_LOCK.lock().unwrap();
    let var = "SWE_EDGE_FEATURE_FL_E2E_OFF";
    // SAFETY: serialized by ENV_LOCK; test-only process mutation
    unsafe { std::env::set_var(var, "false") };
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[fl_e2e_off]\nhost = \"localhost\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result: Result<FeatureState<BrokerConfig>, _> =
        ConfigLoaderFactory::load_feature_section(&loader, "fl_e2e_off");
    // SAFETY: cleanup before any assert that could panic
    unsafe { std::env::remove_var(var) };
    let state = result.unwrap();
    assert!(
        state.is_disabled(),
        "env var=false must disable a present section"
    );
}

#[test]
fn test_load_feature_env_var_true_enables_present_section_and_records_env_source() {
    let _g = ENV_LOCK.lock().unwrap();
    let var = "SWE_EDGE_FEATURE_FL_E2E_ON";
    // SAFETY: serialized by ENV_LOCK; test-only process mutation
    unsafe { std::env::set_var(var, "true") };
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[fl_e2e_on]\nhost = \"localhost\"\nport = 5672");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result = loader.load_feature::<BrokerConfig>("fl_e2e_on");
    // SAFETY: cleanup
    unsafe { std::env::remove_var(var) };
    let loaded = result.unwrap();
    assert!(loaded.state.is_enabled());
    assert!(loaded.record.enabled);
    assert!(
        matches!(
            loaded.record.override_source,
            Some(swe_edge_configbuilder::OverrideSource::EnvVar { .. })
        ),
        "record must carry EnvVar override source"
    );
}

#[test]
fn test_load_feature_env_var_true_overrides_enabled_false_in_toml() {
    let _g = ENV_LOCK.lock().unwrap();
    let var = "SWE_EDGE_FEATURE_FL_E2E_FORCE";
    // SAFETY: serialized by ENV_LOCK; test-only process mutation
    unsafe { std::env::set_var(var, "1") };
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[fl_e2e_force]\nenabled = false\nhost = \"localhost\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result: Result<FeatureState<BrokerConfig>, _> =
        ConfigLoaderFactory::load_feature_section(&loader, "fl_e2e_force");
    // SAFETY: cleanup
    unsafe { std::env::remove_var(var) };
    assert!(
        result.unwrap().is_enabled(),
        "env var=true must override enabled=false in TOML"
    );
}

#[test]
fn test_load_feature_env_var_true_with_absent_section_returns_not_found() {
    let _g = ENV_LOCK.lock().unwrap();
    let var = "SWE_EDGE_FEATURE_FL_E2E_ABSENT";
    // SAFETY: serialized by ENV_LOCK; test-only process mutation
    unsafe { std::env::set_var(var, "yes") };
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other]\nhost = \"x\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result =
        ConfigLoaderFactory::load_feature_section::<BrokerConfig>(&loader, "fl_e2e_absent");
    // SAFETY: cleanup
    unsafe { std::env::remove_var(var) };
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "env var=true + absent section must be NotFound error, got {result:?}"
    );
}

#[test]
fn test_load_feature_invalid_env_var_value_returns_io_error() {
    let _g = ENV_LOCK.lock().unwrap();
    let var = "SWE_EDGE_FEATURE_FL_E2E_INVALID";
    // SAFETY: serialized by ENV_LOCK; test-only process mutation
    unsafe { std::env::set_var(var, "maybe") };
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[fl_e2e_invalid]\nhost = \"localhost\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result =
        ConfigLoaderFactory::load_feature_section::<BrokerConfig>(&loader, "fl_e2e_invalid");
    // SAFETY: cleanup
    unsafe { std::env::remove_var(var) };
    assert!(
        matches!(result, Err(ConfigError::Io(_))),
        "invalid env var value must return Io error, got {result:?}"
    );
}
