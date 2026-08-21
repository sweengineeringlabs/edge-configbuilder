//! `ConfigLoaderFactory::create_loader()` — the simplest loader constructor:
//! no explicit directory, no app name.
//!
//! ## When to use this
//!
//! Use it when your process has no meaningful "app name" for XDG namespacing
//! (e.g. a short-lived CLI or a library that doesn't own its own config
//! namespace) and you're happy to rely purely on env vars / `./config`. If
//! you *do* have a stable app name, prefer `create_loader_xdg` instead (see
//! the `docs_create_loader_xdg` example) — it namespaces the XDG directories
//! under that name so multiple apps on the same machine don't collide. If
//! you already know the exact directory, skip resolution entirely with
//! `create_loader_for_dir` (see `docs_create_loader_for_dir`).
//!
//! ## Resolution order
//!
//! With no directory and no name, `create_loader()` searches, in order:
//! `$CONFIGBUILDER_CONFIG_DIR`, then each entry in `$XDG_CONFIG_DIRS`, then
//! `$XDG_CONFIG_HOME`, then finally `./config` as a last resort.
//!
//! ## What this example does
//!
//! 1. Creates a temp directory and writes an `application.toml` into it.
//! 2. Points `$CONFIGBUILDER_CONFIG_DIR` at that directory — this makes the
//!    example deterministic; without it, resolution would depend on the
//!    machine's real XDG environment and `./config`, which we can't control
//!    here.
//! 3. Calls `create_loader()` and loads the `[app]` section.
//! 4. Asserts the loaded value matches what was written, proving the whole
//!    chain — env var lookup, file read, TOML parse, deserialize — worked.
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
    unsafe { std::env::set_var("CONFIGBUILDER_CONFIG_DIR", dir.path()) };
    let loader = ConfigLoaderFactory::create_loader().expect("create_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("CONFIGBUILDER_CONFIG_DIR") };

    assert_eq!(cfg.name, "create_loader-example");
    println!("create_loader: loaded name={}", cfg.name);
}
