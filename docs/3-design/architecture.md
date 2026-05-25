# Configbuilder Architecture

## Workspace overview

The configbuilder workspace is a single Rust crate — `swe-edge-configbuilder` — that
provides XDG-aware, layered TOML section loading for any `T: DeserializeOwned + Default`.
It is consumed by every swe-edge service crate that needs typed configuration at startup.

| Crate | Package | Purpose |
|-------|---------|---------|
| `configbuilder/main/features/configbuilder` | `swe-edge-configbuilder` | Config path resolution, layered TOML loading, section extraction |

---

## SEA module layout

```
src/
├── api/
│   ├── default_config_builder.rs   # Interface contract for DefaultConfigBuilder
│   ├── default_section_loader.rs   # MAX_CONFIG_FILE_BYTES boundary constant
│   ├── default_validator.rs        # NOT_A_DIR_MSG boundary constant
│   ├── error/
│   │   └── config_error.rs         # ConfigError — Parse | Io | NotFound
│   └── traits/
│       ├── config_builder.rs       # ConfigBuilder trait — fluent builder → Loader
│       ├── loader.rs               # Loader trait — load_section + validate
│       └── validator.rs            # Validator trait — validate_path
├── core/
│   ├── default_config_builder.rs   # DefaultConfigBuilder — path resolution logic
│   ├── default_section_loader.rs   # DefaultSectionLoader — layered TOML merge
│   └── default_validator.rs        # DefaultValidator — path existence check
├── saf/
│   └── configbuilder_svc.rs        # Public factory surface
├── spi.rs                          # ConfigSection — extension hook for consumers
└── lib.rs                          # pub use saf::*
```

---

## Data flow

```
create_config_builder()              create_loader()   create_loader_xdg(app)
         │                                 │                    │
  with_name / with_config_dir              │         XDG chain + SWE_EDGE_CONFIG_DIR
         │                                 │                    │
    build_loader()  ─────────────────────────────────────────► DefaultSectionLoader
         │                                                            │
    validate() ◄── auto-called on construction                       │
                                                              for each config_dir:
                                                                read application.toml
                                                                deep-merge section
                                                                      │
                                                              any_file_found?
                                                               no → Err(NotFound)
                                                               yes → Ok(T::default())
                                                                   or Ok(T::from_toml)
```

---

## Path resolution order

`build_loader()` resolves config directories in this priority order (first match wins):

| Priority | Source | Notes |
|----------|--------|-------|
| 1 (highest) | `with_config_dir(path)` | Caller-supplied; used verbatim; bypasses env vars |
| 2 | XDG chain (when name set) | `$XDG_CONFIG_DIRS/<name>` → `$XDG_CONFIG_HOME/<name>` → `$SWE_EDGE_CONFIG_DIR` |
| 3 (lowest) | `$SWE_EDGE_CONFIG_DIR` or `config/` | Bare fallback when no name and no explicit dirs |

Within a chain, directories are merged in order — **later entries win** on key conflicts.
Merging is recursive: nested TOML tables are deep-merged; scalars and arrays are replaced.

---

## Environment variables

Config is supplied via the environment, not baked into the binary — following
[12-factor app factor III](https://12factor.net/config): _"store config in the environment"_.

| Variable | Read by | Effect |
|----------|---------|--------|
| `SWE_EDGE_CONFIG_DIR` | `create_loader`, `create_loader_xdg`, `build_loader` | Override config directory; highest priority in XDG chain |
| `XDG_CONFIG_DIRS` | `create_loader_xdg`, `build_loader` | Colon-separated base dirs; each joined with app name |
| `XDG_CONFIG_HOME` | `dirs::config_dir()` (external) | User config home; resolved by the `dirs` crate per platform |

All env var paths are validated against `..` traversal at construction time.

---

## Key contracts

| Type | Role |
|------|------|
| `ConfigBuilder` | Fluent builder — accumulates name, version, dirs; produces a `Loader` via `build_loader()` |
| `Loader` | Loads a typed TOML section by dotted key; validates configured dirs |
| `Validator` | Checks that a path is a directory (not a file) |
| `ConfigError` | `Parse` — malformed TOML; `Io` — filesystem or traversal error; `NotFound` — no `application.toml` found |
| `ConfigSection` | `spi` extension hook — implement on a config struct to make it self-loading via `load(&loader)` |

---

## Error semantics

| Error | Condition |
|-------|-----------|
| `ConfigError::NotFound` | No `application.toml` found in any configured directory |
| `ConfigError::Io` | Path is a file not a directory; `..` traversal in env var; file exceeds 1 MiB |
| `ConfigError::Parse` | `application.toml` is not valid TOML |

`Ok(T::default())` is returned when `application.toml` exists but the requested section key is absent — this is intentional optional config, not misconfiguration.

---

## See Also

- [Architecture Overview](../../docs/3-architecture/architecture.md)
- [Runtime Architecture](../../runtime/docs/architecture.md)
