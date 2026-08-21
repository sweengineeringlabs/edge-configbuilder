# configbuilder

## WHAT

Standalone, runtime-independent TOML section loader for swe-edge services.

Key capabilities:

- **`ConfigSection` / `OptionalSection`** — typed TOML section loading for any `T: DeserializeOwned + Default`
- **XDG-aware layered resolution** — searches `$XDG_CONFIG_DIRS`, `$XDG_CONFIG_HOME`, `$SWE_EDGE_CONFIG_DIR`, and explicit paths; later sources win at the key level
- **Dotted key paths** — load nested sections with `"outer.inner"` syntax
- **`{{VAR_NAME}}` substitution** — pluggable name policies (`PrefixWhitelistPolicy`, `PatternWhitelistPolicy`, `CompositePolicy`) gate *which* variables are allowed; a pluggable `ValueResolver` (default: `EnvValueResolver`, backed by `std::env::var`) controls *where* their values come from — e.g. a secrets backend instead of the process environment
- **Preflight validation** — validates config shape and reports structured `PreflightIssue` list before runtime
- **`ConfigLoaderFactory`** — the crate's SAF; all consumer-facing behavior is reached through it, so `api/` and `core/` stay internal to the crate's architecture

## WHY

| Problem | Solution |
|---------|----------|
| Config loading tied to runtime startup | Standalone crate — no dependency on `swe-edge-runtime`; library crates can load config without pulling in the full runtime |
| Different services implement their own TOML parsing | Single typed `load_section("key")` API; deserialization and merging handled once |
| Credentials injected as hardcoded strings | Env var substitution with explicit security policies; `AllowAllPolicy` is test-only, `PrefixWhitelistPolicy` is the production default |
| Config errors discovered at handler execution time | Preflight report surfaces missing keys and malformed sections at startup, before any request is served |
| Diamond dep conflicts from multiple config-loading crates | One crate, one tag — all edge consumers pin the same version; kgraph detects conflicts pre-commit |
