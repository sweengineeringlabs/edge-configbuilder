# swe-edge-config

Standalone, runtime-independent TOML section loader for swe-edge services.

Provides XDG-aware, layered config section loading for any `T: DeserializeOwned + Default`.
Library crates can depend on this crate directly without pulling in `swe-edge-runtime-main`.

## Features

- **Layered config resolution**: merges config from multiple directories with later sources winning
- **XDG Base Directory support**: automatic path resolution via `$XDG_CONFIG_HOME`, `$XDG_CONFIG_DIRS`, `$SWE_EDGE_CONFIG_DIR`
- **Dotted key paths**: load nested sections with `"outer.inner"` syntax
- **Optional environment variable substitution**: inject env vars into TOML with `{{VAR_NAME}}` syntax and pluggable security policies

## Basic Usage

```rust
use swe_edge_config::create_loader;

#[derive(serde::Deserialize, Default)]
struct BrokerConfig { host: String, port: u16 }

let loader = create_loader()?;
let cfg: BrokerConfig = loader.load_section("broker")?;
```

## Environment Variable Substitution

Substitute environment variables in TOML config using `{{VAR_NAME}}` syntax. Substitution is **opt-in** and requires an explicit security policy.

### AllowAllPolicy (unsafe — testing only)

```rust
use swe_edge_config::{create_loader_with_substitution, AllowAllPolicy};

#[derive(serde::Deserialize, Default)]
struct DbConfig { 
    host: String,
    port: u16,
    password: String,
}

let policy = AllowAllPolicy;
let loader = create_loader_with_substitution(Box::new(policy))?;

// TOML: [db]
//       host = "{{DB_HOST}}"
//       password = "{{DB_PASSWORD}}"
let cfg: DbConfig = loader.load_section("db")?;
// Substitutes from environment: DB_HOST, DB_PASSWORD
```

### PrefixWhitelistPolicy (recommended for production)

Restrict substitution to environment variables with allowed prefixes:

```rust
use swe_edge_config::{create_loader_with_substitution, PrefixWhitelistPolicy};

let policy = PrefixWhitelistPolicy::new(vec![
    "APP_".into(),
    "DB_".into(),
    "SERVICE_".into(),
]);
let loader = create_loader_with_substitution(Box::new(policy))?;

// TOML: [db]
//       host = "{{DB_HOST}}"
//       port = "5432"
// 
// OK: DB_HOST matches DB_ prefix
// Error: would fail if you used {{PRIVATE_KEY}} (no matching prefix)
let cfg: DbConfig = loader.load_section("db")?;
```

### PatternWhitelistPolicy (regex-based validation)

Use regex patterns for fine-grained control:

```rust
use swe_edge_config::{create_loader_with_substitution, PatternWhitelistPolicy};

let policy = PatternWhitelistPolicy::new(
    "^(APP|SERVICE)_[A-Z_]+$".into()
)?;
let loader = create_loader_with_substitution(Box::new(policy))?;

// TOML: [service]
//       url = "{{SERVICE_API_URL}}"
// OK: matches pattern
let cfg = loader.load_section("service")?;
```

### CompositePolicy (layered validation)

Combine multiple policies — all must pass:

```rust
use swe_edge_config::{
    create_loader_with_substitution,
    PrefixWhitelistPolicy,
    PatternWhitelistPolicy,
    CompositePolicy,
    SubstitutionPolicy,
};

let policies: Vec<Box<dyn SubstitutionPolicy>> = vec![
    Box::new(PrefixWhitelistPolicy::new(vec!["APP_".into()])),
    Box::new(PatternWhitelistPolicy::new("^APP_[A-Z_]+$".into())?),
];
let policy = CompositePolicy::new(policies);
let loader = create_loader_with_substitution(Box::new(policy))?;
```

## Escaping Literal Braces

To use literal `{{` or `}}` in your TOML without substitution, escape them:

```toml
# TOML: [docs]
#       note = "Use \{\{VAR_NAME\}\} for substitution"
# 
# Result: "Use {{VAR_NAME}} for substitution"
```

## Substitution Error Handling

Substitution can fail if:
- Environment variable doesn't exist
- Variable name rejected by security policy
- Invalid placeholder syntax (nested placeholders)

```rust
use swe_edge_config::ConfigError;

match loader.load_section::<MyConfig>("section") {
    Ok(cfg) => { /* use cfg */ }
    Err(ConfigError::Io(msg)) => {
        eprintln!("Substitution failed: {}", msg);
        // Error includes file path and config key for debugging
    }
    Err(e) => eprintln!("Config error: {}", e),
}
```

## Configuration Layering

Config directories are searched in order; later sources override earlier ones **at the key level** (deep merge for TOML tables):

```
$XDG_CONFIG_DIRS/app/application.toml  (lowest priority)
    ↓ merged into ↓
$XDG_CONFIG_HOME/app/application.toml
    ↓ merged into ↓
$SWE_EDGE_CONFIG_DIR/application.toml (if set)
    ↓ merged into ↓
builder.with_config_dir("/explicit/path")  (highest priority)
```

Tables are recursively merged; scalars and arrays are replaced.

## Builder Pattern

For more control over paths and substitution together:

```rust
use swe_edge_config::{create_config_builder_with_substitution, PrefixWhitelistPolicy};

let policy = PrefixWhitelistPolicy::new(vec!["APP_".into()]);
let loader = create_config_builder_with_substitution(Box::new(policy))
    .with_config_dir("/etc/myapp")
    .build_loader()?;
```
