//! `ConfigLoaderFactory::create_loader_xdg` — XDG-resolved loader for a named app.
//!
//! Whatever the app name, `$SWE_EDGE_CONFIG_DIR` (if set) is always included as
//! an extra search directory, so we use it here to keep the example deterministic.
#![allow(unsafe_code)]
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
        "[app]\nname = \"create_loader_xdg-example\"\n",
    )
    .expect("write application.toml");

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe { std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path()) };
    let loader =
        ConfigLoaderFactory::create_loader_xdg("docs-example-app").expect("create_loader_xdg");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("SWE_EDGE_CONFIG_DIR") };

    assert_eq!(cfg.name, "create_loader_xdg-example");
    println!("create_loader_xdg: loaded name={}", cfg.name);
}
