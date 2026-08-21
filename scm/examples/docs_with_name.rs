//! `ConfigBuilder::with_name(name)` — sets the application name used to
//! derive XDG-resolved config directories (e.g. `~/.config/<name>/`).
//!
//! ## When to use this
//!
//! Set this whenever you're using the builder chain (as opposed to
//! `create_loader_xdg`, which takes the name as a plain argument) and want
//! XDG namespacing rather than an explicit `with_config_dir`. It's the
//! builder-chain equivalent of the `app_name` parameter on `create_loader_xdg`
//! (`docs_create_loader_xdg`) — same resolution behavior, different call
//! shape. Without it, the builder falls back to `create_loader`'s
//! unnamespaced XDG chain (see `docs_create_loader`).
//!
//! ## What this example does
//!
//! 1. Calls `with_name("docs-example-app")` and reads the name straight back
//!    via `ConfigBuilder::name()` — proving the setter's effect is visible
//!    before the builder is consumed by `build_loader()`.
//! 2. Points `$CONFIGBUILDER_CONFIG_DIR` at a temp dir. This env var is
//!    always included as an extra search directory regardless of the app
//!    name (see `docs_create_loader_xdg`'s resolution-order note), which
//!    keeps this example deterministic without needing a real
//!    `~/.config/docs-example-app/` on the machine running it.
//! 3. Builds the loader and loads the section, proving `with_name` didn't
//!    just set a field — the whole chain still works end to end.
#![allow(unsafe_code)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let builder = ConfigLoaderFactory::create_config_builder().with_name("docs-example-app");
    assert_eq!(builder.name(), "docs-example-app");

    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nvalue = \"with_name-example\"\n",
    )
    .expect("write application.toml");

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe { std::env::set_var("CONFIGBUILDER_CONFIG_DIR", dir.path()) };
    let loader = builder.build_loader().expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("CONFIGBUILDER_CONFIG_DIR") };

    assert_eq!(cfg.value, "with_name-example");
    println!(
        "with_name: name reflected + loader works, value={}",
        cfg.value
    );
}
