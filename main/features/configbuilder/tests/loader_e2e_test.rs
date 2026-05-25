//! Smoke tests for `create_loader` — verifies the factory produces usable instances.

use swe_edge_configbuilder::{
    create_loader, create_loader_for_dir, create_loader_xdg, ConfigError, Loader as _,
};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

/// @covers: create_loader
#[test]
fn test_create_loader_returns_not_found_for_absent_section() {
    let result: Result<Cfg, _> = create_loader()
        .unwrap()
        .load_section("nonexistent_default_xyz");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for absent section, got {result:?}"
    );
}

/// @covers: create_loader_for_dir
#[test]
fn test_create_loader_for_dir_returns_not_found_when_no_toml() {
    let dir = tempfile::tempdir().unwrap();
    let result: Result<Cfg, _> = create_loader_for_dir(dir.path()).load_section("any");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for empty dir, got {result:?}"
    );
}

/// @covers: create_loader_xdg
#[test]
fn test_create_loader_xdg_returns_not_found_for_unknown_app() {
    let result: Result<Cfg, _> = create_loader_xdg("swe-edge-default-loader-test-nonexistent-xyz")
        .unwrap()
        .load_section("any");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for unknown XDG app, got {result:?}"
    );
}
