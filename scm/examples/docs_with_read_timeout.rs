//! `ConfigBuilderInit::with_read_timeout` — overrides the 30-second default
//! wall-clock deadline for each config file read. Guards against a stalled
//! NFS/FUSE mount hanging startup; a generous timeout has no effect on a
//! normal, fast local read, which is what this example demonstrates.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{
    BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderInit as _, ConfigLoaderFactory,
    Loader as _,
};
use std::time::Duration;

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nvalue = \"with_read_timeout-example\"\n",
    )
    .expect("write application.toml");

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_read_timeout(Duration::from_secs(5))
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.value, "with_read_timeout-example");
    println!(
        "with_read_timeout: 5s deadline, normal read still succeeds, value={}",
        cfg.value
    );
}
