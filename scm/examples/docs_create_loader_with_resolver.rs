//! `ConfigLoaderFactory::create_loader_with_resolver(policy, resolver)` —
//! `create_loader_with_substitution`'s XDG resolution and substitution, but
//! values come from a custom `ValueResolver` instead of `std::env::var`.
//!
//! ## When to use this
//!
//! `create_loader_with_substitution` always sources substituted values from
//! the process environment (`EnvValueResolver`, the default). Use this
//! constructor instead when values should come from somewhere else — a
//! secrets manager, a KMS-backed store, an in-memory map populated at
//! startup. The `policy` still gates *which* variable names are allowed;
//! the `resolver` only changes *where* an allowed name's value comes from.
//! This separation of concerns means you can swap the value source without
//! touching your substitution security policy at all.
//!
//! ## What this example does
//!
//! 1. Defines `StaticResolver`, a trivial `ValueResolver` that returns a
//!    hardcoded value for one known name and errors for anything else — a
//!    stand-in for a real secrets-backend integration.
//! 2. Writes an `application.toml` with a `{{APP_GREETING}}` placeholder.
//! 3. Points `$CONFIGBUILDER_CONFIG_DIR` at the temp dir for deterministic
//!    XDG resolution (same reasoning as `docs_create_loader`).
//! 4. Loads the section and asserts the value came from `StaticResolver`,
//!    not from any environment variable (none was set).
#![allow(unsafe_code)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{ConfigLoaderFactory, Loader as _, SubstitutionError, ValueResolver};

struct StaticResolver;
impl ValueResolver for StaticResolver {
    fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError> {
        match var_name {
            "APP_GREETING" => Ok("hello from a static resolver".to_string()),
            _ => Err(SubstitutionError::VariableNotFound {
                var_name: var_name.to_string(),
                location: location.to_string(),
            }),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    greeting: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\ngreeting = \"{{APP_GREETING}}\"\n",
    )
    .expect("write application.toml");

    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["APP_".to_string()]);

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe { std::env::set_var("CONFIGBUILDER_CONFIG_DIR", dir.path()) };
    let loader = ConfigLoaderFactory::create_loader_with_resolver(
        Box::new(policy),
        Box::new(StaticResolver),
    )
    .expect("create_loader_with_resolver");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("CONFIGBUILDER_CONFIG_DIR") };

    assert_eq!(cfg.greeting, "hello from a static resolver");
    println!("create_loader_with_resolver: greeting={}", cfg.greeting);
}
