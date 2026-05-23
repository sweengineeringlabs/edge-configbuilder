# Config Architecture

## Workspace overview

The config workspace is a single Rust crate — `swe-edge-config` — that provides
typed, layered TOML configuration loading. It is the foundation for all policy
configuration in the swe-edge stack.

| Crate | Package | Purpose |
|-------|---------|---------|
| `config/swe-edge-config` | `swe-edge-config` | Typed TOML section loader with XDG path resolution |

---

## SEA module layout

```
src/
├── api/
│   ├── config_error.rs      # ConfigError — Io, Parse, SectionNotFound
│   └── traits.rs            # Validator trait — structural validation contract
├── core/
│   └── section_loader.rs    # DefaultSectionLoader — reads and deserialises TOML
├── saf/
│   ├── mod.rs               # Public factory surface
│   └── section_loader.rs    # load_section(), load_section_xdg() factories
└── lib.rs                   # pub use saf::*
```

---

## Config resolution

`load_section<T>(path, key)` reads a TOML file at `path` and deserialises the
section at `key` into type `T` via `serde`. A missing section returns `T::default()`.

`load_section_xdg<T>(app, key)` resolves the config directory using the XDG
specification: `$XDG_CONFIG_HOME/<app>/application.toml`, or
`$HOME/.config/<app>/application.toml` as a fallback. The `SWE_EDGE_CONFIG_DIR`
environment variable overrides both.

```
SWE_EDGE_CONFIG_DIR (highest precedence)
        ↓
$XDG_CONFIG_HOME/<app>/application.toml
        ↓
$HOME/.config/<app>/application.toml
```

---

## Key contracts

| Type | Role |
|------|------|
| `load_section::<T>()` | Load a typed section from an explicit file path |
| `load_section_xdg::<T>()` | Load a typed section using XDG path resolution |
| `ConfigError` | `Io`, `Parse`, `SectionNotFound` error variants |

---

## See Also

- [Observability Config Architecture](../../../observ-config/swe-edge-observ-config/docs/architecture.md)
- [Runtime Architecture](../../../runtime/docs/architecture.md)
- [Architecture Overview](../../../docs/3-architecture/architecture.md)
