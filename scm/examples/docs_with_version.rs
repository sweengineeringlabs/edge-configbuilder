//! `ConfigBuilder::with_version` — sets the application version string.
//! Purely metadata: it does not affect config directory resolution.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{ConfigBuilder as _, ConfigLoaderFactory};

fn main() {
    let builder = ConfigLoaderFactory::create_config_builder().with_version("2.0.0");
    assert_eq!(builder.version(), "2.0.0");
    println!("with_version: version reflected = {}", builder.version());
}
