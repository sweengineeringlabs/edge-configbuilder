//! `ConfigLoaderFactory::create_loader_for_dir` — loader scoped to one explicit directory.
//!
//! No XDG resolution, no env vars — reads `application.toml` only from `dir`.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    name: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nname = \"create_loader_for_dir-example\"\n",
    )
    .expect("write application.toml");

    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.name, "create_loader_for_dir-example");
    println!("create_loader_for_dir: loaded name={}", cfg.name);
}
