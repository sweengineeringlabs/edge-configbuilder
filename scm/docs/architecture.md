# Architecture — edge-configbuilder

## Sequence

> `ConfigLoaderFactory` resolves XDG paths at startup, merges the layer stack (defaults → application → env overrides), and returns a typed section to the caller.

```mermaid
sequenceDiagram
    participant App
    participant ConfigLoaderFactory
    participant SectionLoader
    participant XDGResolver
    participant FileSystem

    App->>ConfigLoaderFactory: create_loader()
    ConfigLoaderFactory->>XDGResolver: resolve(app_name)
    XDGResolver-->>ConfigLoaderFactory: [~/.config/app/, /etc/app/]
    ConfigLoaderFactory-->>App: SectionLoaderImpl

    App->>SectionLoader: load_section("application.tls")
    SectionLoader->>FileSystem: read config/default.toml
    SectionLoader->>FileSystem: read config/application.toml
    SectionLoader->>SectionLoader: merge layers (later overrides earlier)
    SectionLoader->>SectionLoader: apply env var substitutions
    SectionLoader-->>App: Result<TlsConfig, ConfigError>

    opt feature flags
        App->>SectionLoader: load_optional<T>(registry)
        SectionLoader-->>App: FeatureRecord{enabled, value}
    end
```

## Data Flow

> A section key and `app_name` drive XDG path resolution; layered TOML files merge into a typed config struct.

```mermaid
flowchart LR
    A["app_name: &str\nsection_key: &str"] --> B["XDGResolver\nresolve paths"]
    B --> C["config/default.toml\n(bundled defaults)"]
    B --> D["~/.config/<app>/\napplication.toml"]
    B --> E["ENV vars\nSWE_SECTION_KEY=value"]

    C --> F["TOML merge\n(later layers win)"]
    D --> F
    E --> F["SubstitutionPolicy\nreplace ${VAR}"]

    F --> G["deserialize<T:\nDeserializeOwned\n+ Default>"]
    G -->|Ok| H["T  (typed config struct)"]
    G -->|Err| I["ConfigError\n::Parse / ::Missing\n::Validation"]

    subgraph FeatureRegistry["Optional features (load_in_order!)"]
        J["OptionalSection::section_name()"] --> K["topological sort\n(dependency order)"]
        K --> L["FeatureRegistry\n::load<T>"]
        L --> M["FeatureRecord\n{enabled, value}"]
    end
```
