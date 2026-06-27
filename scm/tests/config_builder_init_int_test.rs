//! Integration tests for the `ConfigBuilderInit` trait — `new`, `with_read_timeout`.
#![allow(missing_docs, clippy::unwrap_used)]
use std::time::Duration;
use swe_edge_configbuilder::{
    BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderInit as _, ConfigLoaderFactory,
    Loader as _,
};

// ── new ───────────────────────────────────────────────────────────────────────

#[test]
fn test_new_creates_builder_with_empty_fields_happy() {
    use swe_edge_configbuilder::ConfigBuilderImpl;
    let b = ConfigBuilderImpl::new();
    assert_eq!(b.name(), "");
    assert_eq!(b.version(), "");
}

#[test]
fn test_new_builder_with_file_as_config_dir_fails_error() {
    // ConfigBuilderInit::new() + with_config_dir pointing to an existing file must fail.
    use swe_edge_configbuilder::ConfigBuilderImpl;
    let file = tempfile::NamedTempFile::new().unwrap();
    let result = ConfigBuilderImpl::new()
        .with_config_dir(file.path())
        .build_loader();
    match result {
        Err(e) => assert!(
            e.to_string().contains("not a directory"),
            "error must explain that the path is not a directory, got: {e}"
        ),
        Ok(_) => panic!("expected Err for file-path config dir but got Ok"),
    }
}

#[test]
fn test_new_returns_independent_instances_edge() {
    use swe_edge_configbuilder::ConfigBuilderImpl;
    let a = ConfigBuilderImpl::new();
    let b = ConfigBuilderImpl::new();
    assert_eq!(a.name(), b.name(), "two new() instances must both have empty name");
    assert_eq!(a.version(), b.version(), "two new() instances must both have empty version");
}

// ── with_read_timeout ─────────────────────────────────────────────────────────

#[test]
fn test_with_read_timeout_overrides_default_timeout_happy() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "[s]\nx = 1\n").unwrap();

    #[derive(serde::Deserialize, Default)]
    #[serde(default)]
    struct S { x: i32 }

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_read_timeout(Duration::from_millis(500))
        .build_loader()
        .unwrap();
    let s: S = loader.load_section("s").unwrap();
    assert_eq!(s.x, 1);
}

#[test]
fn test_with_read_timeout_file_path_still_fails_build_error() {
    // with_read_timeout must not suppress validation errors — a file-path config dir
    // must still produce Err from build_loader regardless of the timeout value.
    use swe_edge_configbuilder::ConfigBuilderImpl;
    let file = tempfile::NamedTempFile::new().unwrap();
    let result = ConfigBuilderImpl::new()
        .with_read_timeout(Duration::from_millis(500))
        .with_config_dir(file.path())
        .build_loader();
    assert!(result.is_err(), "build_loader must still reject a file-path config dir");
}

#[test]
fn test_with_read_timeout_produces_functional_loader_edge() {
    use swe_edge_configbuilder::ConfigBuilderImpl;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "[s]\nv = 42\n").unwrap();
    #[derive(serde::Deserialize, Default)]
    #[serde(default)]
    struct S { v: i32 }
    let loader = ConfigBuilderImpl::new()
        .with_config_dir(dir.path())
        .with_read_timeout(Duration::from_secs(30))
        .build_loader()
        .unwrap();
    let s: S = loader.load_section("s").unwrap();
    assert_eq!(s.v, 42, "loader with explicit timeout must read TOML correctly");
}
