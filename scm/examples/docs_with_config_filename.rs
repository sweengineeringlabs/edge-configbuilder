//! `ConfigBuilder::with_config_filename` — overrides the config filename
//! searched for in each configured directory (defaults to `application.toml`).
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    // A stray application.toml must be ignored once a custom filename is set.
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nvalue = \"wrong-file\"\n",
    )
    .expect("write application.toml");
    std::fs::write(
        dir.path().join("settings.toml"),
        "[app]\nvalue = \"right-file\"\n",
    )
    .expect("write settings.toml");

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_config_filename("settings.toml")
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.value, "right-file");
    println!(
        "with_config_filename: read settings.toml, value={}",
        cfg.value
    );
}
