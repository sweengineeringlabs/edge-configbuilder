//! Integration tests for the `BuilderFinalizer` trait — `build_loader`.
#![allow(missing_docs, clippy::unwrap_used)]
use swe_edge_configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[test]
fn test_build_loader_with_valid_config_dir_happy() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("application.toml"), "[app]\nname = \"test\"\n").unwrap();

    #[derive(serde::Deserialize, Default)]
    #[serde(default)]
    struct App { name: String }

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap();
    let app: App = loader.load_section("app").unwrap();
    assert_eq!(app.name, "test");
}

#[test]
fn test_build_loader_with_file_path_instead_of_dir_error() {
    // validate() rejects paths that exist but are not directories.
    let file = tempfile::NamedTempFile::new().unwrap();
    let result = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(file.path())
        .build_loader();
    assert!(result.is_err(), "a file path must be rejected as config dir");
}

#[test]
fn test_build_loader_nonexistent_dir_load_returns_not_found_edge() {
    use swe_edge_configbuilder::ConfigError;
    // A nonexistent dir passes validate() (only existing-but-not-dir paths fail).
    // Loading a section from it must return NotFound.
    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir("/nonexistent_xyz_swelabs_configbuilder_test_dir")
        .build_loader()
        .expect("build_loader must succeed for nonexistent dir path");
    let result: Result<std::collections::HashMap<String, String>, _> =
        loader.load_section("anything");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "loading from nonexistent dir must return NotFound, got: {result:?}"
    );
}
