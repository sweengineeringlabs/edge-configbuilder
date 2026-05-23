//! Smoke tests for `create_loader` — verifies the factory produces usable instances.

use swe_edge_configbuilder::{
    create_loader, create_loader_for_dir, create_loader_xdg, Loader as _,
};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

/// @covers: create_loader
#[test]
fn test_create_loader_returns_usable_loader() {
    let result: Result<Cfg, _> = create_loader().load_section("nonexistent_default_xyz");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Cfg::default());
}

/// @covers: create_loader_for_dir
#[test]
fn test_create_loader_for_dir_returns_usable_loader() {
    let dir = tempfile::tempdir().unwrap();
    let result: Result<Cfg, _> = create_loader_for_dir(dir.path()).load_section("any");
    assert!(result.is_ok());
}

/// @covers: create_loader_xdg
#[test]
fn test_create_loader_xdg_returns_usable_loader() {
    let result: Result<Cfg, _> =
        create_loader_xdg("swe-edge-default-loader-test-nonexistent-xyz").load_section("any");
    assert!(result.is_ok());
}
