//! End-to-end tests for `ConfigBuilder::with_config_filename`.
#![cfg(feature = "test-utils")]
#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(unsafe_code)]

use configbuilder::{
    AllowAllPolicy, BuilderFinalizer as _, ConfigBuilder as _, ConfigError, ConfigLoaderFactory,
    Loader as _,
};

#[derive(Debug, serde::Deserialize, Default)]
#[serde(default)]
struct App {
    name: String,
}

#[test]
fn test_with_config_filename_reads_custom_filename_instead_of_application_toml() {
    let dir = tempfile::tempdir().unwrap();
    // A stray application.toml must be ignored once a custom filename is set.
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nname = \"wrong\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("custom.toml"), "[app]\nname = \"right\"\n").unwrap();

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_config_filename("custom.toml")
        .build_loader()
        .unwrap();
    let app: App = loader.load_section("app").unwrap();
    assert_eq!(app.name, "right");
}

#[test]
fn test_without_with_config_filename_default_behavior_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nname = \"default\"\n",
    )
    .unwrap();

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap();
    let app: App = loader.load_section("app").unwrap();
    assert_eq!(app.name, "default");
}

#[test]
fn test_with_config_filename_missing_custom_file_returns_not_found() {
    let dir = tempfile::tempdir().unwrap();
    // application.toml exists, but the configured custom filename does not.
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nname = \"unused\"\n",
    )
    .unwrap();

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_config_filename("custom.toml")
        .build_loader()
        .unwrap();
    let result: Result<App, _> = loader.load_section("app");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "missing custom filename must return NotFound, got: {result:?}"
    );
}

#[test]
fn test_with_config_filename_on_substitution_builder_chain() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("custom.toml"),
        "[app]\nname = \"{{APP_NAME_CONFIG_FILENAME_TEST}}\"\n",
    )
    .unwrap();

    // SAFETY: single-threaded test binary; no other thread reads or writes this var
    unsafe { std::env::set_var("APP_NAME_CONFIG_FILENAME_TEST", "substituted") };
    let loader =
        ConfigLoaderFactory::create_config_builder_with_substitution(Box::new(AllowAllPolicy))
            .with_config_dir(dir.path())
            .with_config_filename("custom.toml")
            .build_loader()
            .unwrap();
    let app: App = loader.load_section("app").unwrap();
    // SAFETY: cleanup — same invariant as above
    unsafe { std::env::remove_var("APP_NAME_CONFIG_FILENAME_TEST") };

    assert_eq!(app.name, "substituted");
}
