//! Load a typed configuration section from application.toml.

use swe_edge_configbuilder::{create_loader_for_dir, Loader as _};

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct BrokerConfig {
    host: String,
    port: u16,
}

fn main() {
    let cfg: BrokerConfig = create_loader_for_dir("config")
        .load_section("application.broker")
        .unwrap_or_default();
    println!("host={} port={}", cfg.host, cfg.port);
}
