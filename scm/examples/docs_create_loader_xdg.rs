//! `ConfigLoaderFactory::create_loader_xdg(app_name)` — XDG-resolved loader
//! namespaced under a specific application name.
//!
//! ## When to use this
//!
//! Use it whenever your process has a stable, known name and you want the
//! standard XDG behavior: config lives at `~/.config/<app_name>/` (or under
//! each `$XDG_CONFIG_DIRS` entry), so multiple applications on the same
//! machine each get their own namespace instead of colliding in a shared
//! directory. This is the constructor most services should reach for by
//! default. If you have no natural app name, fall back to `create_loader`
//! (`docs_create_loader`); if you already know the exact directory, skip
//! resolution with `create_loader_for_dir` (`docs_create_loader_for_dir`).
//!
//! ## Resolution order
//!
//! `app_name`-joined `$XDG_CONFIG_DIRS` entries, then `$XDG_CONFIG_HOME`, then
//! finally `$CONFIGBUILDER_CONFIG_DIR` verbatim (**not** joined with
//! `app_name` — it's always treated as a complete, explicit override path).
//!
//! ## What this example does
//!
//! 1. Creates a temp directory and writes an `application.toml` into it.
//! 2. Points `$CONFIGBUILDER_CONFIG_DIR` at that directory so the example is
//!    deterministic regardless of the machine's real XDG state — the app
//!    name (`"docs-example-app"`) still drives resolution, but this env var
//!    guarantees a directory is found even on a machine with nothing under
//!    `~/.config/docs-example-app/`.
//! 3. Calls `create_loader_xdg("docs-example-app")` and loads the section.
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
    unsafe { std::env::set_var("CONFIGBUILDER_CONFIG_DIR", dir.path()) };
    let loader =
        ConfigLoaderFactory::create_loader_xdg("docs-example-app").expect("create_loader_xdg");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("CONFIGBUILDER_CONFIG_DIR") };

    assert_eq!(cfg.name, "create_loader_xdg-example");
    println!("create_loader_xdg: loaded name={}", cfg.name);
}
