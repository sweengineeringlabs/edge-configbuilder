# swe-edge-config

Standalone, runtime-independent TOML section loader for swe-edge services.

Provides XDG-aware, layered config section loading for any `T: DeserializeOwned + Default`.
Library crates can depend on this crate directly without pulling in `swe-edge-runtime-main`.

## Usage

```rust
use swe_edge_config::load_section_from;

#[derive(serde::Deserialize, Default)]
struct BrokerConfig { host: String, port: u16 }

let cfg: BrokerConfig = load_section_from("application.broker", "config")?;
```
