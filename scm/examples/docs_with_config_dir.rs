//! `ConfigBuilder::with_config_dir` — appends an explicit config directory.
//! Multiple calls accumulate; later directories win on key conflicts.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let low = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        low.path().join("application.toml"),
        "[app]\nvalue = \"low-priority\"\n",
    )
    .expect("write low application.toml");

    let high = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        high.path().join("application.toml"),
        "[app]\nvalue = \"high-priority\"\n",
    )
    .expect("write high application.toml");

    // Later with_config_dir calls win on key conflicts.
    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(low.path())
        .with_config_dir(high.path())
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.value, "high-priority");
    println!("with_config_dir: later dir wins, value={}", cfg.value);
}
