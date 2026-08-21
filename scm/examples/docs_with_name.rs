//! `ConfigBuilder::with_name` — sets the application name used to derive
//! XDG-resolved config directories (e.g. `~/.config/<name>/`).
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

    // `$SWE_EDGE_CONFIG_DIR` is always included as an extra search directory
    // regardless of the app name, which keeps this example deterministic.
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nvalue = \"with_name-example\"\n",
    )
    .expect("write application.toml");

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe { std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path()) };
    let loader = builder.build_loader().expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("SWE_EDGE_CONFIG_DIR") };

    assert_eq!(cfg.value, "with_name-example");
    println!(
        "with_name: name reflected + loader works, value={}",
        cfg.value
    );
}
