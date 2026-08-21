//! `ConfigLoaderFactory::create_config_builder` — the fluent builder-chain
//! entry point. `name`/`version` default to `CARGO_PKG_NAME`/`CARGO_PKG_VERSION`
//! until overridden; see `with_name.rs` and `with_version.rs` for that.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    name: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nname = \"create_config_builder-example\"\n",
    )
    .expect("write application.toml");

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.name, "create_config_builder-example");
    println!("create_config_builder: loaded name={}", cfg.name);
}
