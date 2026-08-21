//! `ConfigLoaderFactory::create_loader` — XDG-resolved loader, no app name.
//!
//! With no explicit directory and no app name, resolution falls back to the
//! XDG chain: `$SWE_EDGE_CONFIG_DIR`, `$XDG_CONFIG_DIRS`, `$XDG_CONFIG_HOME`,
//! then `./config`. We point `$SWE_EDGE_CONFIG_DIR` at a temp dir so this
//! example is deterministic regardless of the machine it runs on.
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
        "[app]\nname = \"create_loader-example\"\n",
    )
    .expect("write application.toml");

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe { std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path()) };
    let loader = ConfigLoaderFactory::create_loader().expect("create_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("SWE_EDGE_CONFIG_DIR") };

    assert_eq!(cfg.name, "create_loader-example");
    println!("create_loader: loaded name={}", cfg.name);
}
