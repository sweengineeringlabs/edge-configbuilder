//! `ConfigLoaderFactory::create_loader_for_dir(dir)` — loader scoped to one
//! explicit directory, no resolution at all.
//!
//! ## When to use this
//!
//! Use it whenever the config directory is already known — a CLI `--config-dir`
//! flag, a container-mounted volume, a path baked into a deployment manifest.
//! It's the simplest, fastest, and most predictable constructor: no env vars
//! are consulted, no XDG chain is walked, nothing is inferred. This is also
//! the right choice for tests and examples (as used throughout this crate's
//! own test suite) because behavior never depends on the machine's ambient
//! environment.
//!
//! Compare with `create_loader` (`docs_create_loader`) and `create_loader_xdg`
//! (`docs_create_loader_xdg`), both of which fall back to env-var/XDG
//! resolution when no directory is supplied.
//!
//! ## What this example does
//!
//! 1. Creates a temp directory and writes an `application.toml` into it.
//! 2. Calls `create_loader_for_dir(dir)` directly — no env vars touched.
//! 3. Loads the `[app]` section and asserts the value round-trips correctly.
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
