//! Load a typed configuration section from application.toml via ConfigBuilder.

use swe_edge_configbuilder::{create_config_builder, ConfigBuilder as _, Loader as _};

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
struct BrokerConfig {
    host: String,
    port: u16,
}

fn main() {
    let loader = create_config_builder().with_name("my-app").build_loader();

    match loader.load_section::<BrokerConfig>("application.broker") {
        Ok(cfg) => println!("host={} port={}", cfg.host, cfg.port),
        Err(e) => eprintln!("failed to load config: {e}"),
    }
}
