//! `ConfigLoaderFactory::create_loader_xdg_with_resolver(app_name, policy, resolver)`
//! — app-namespaced XDG resolution, `{{VAR_NAME}}` substitution, values from
//! a custom `ValueResolver` instead of `std::env::var`.
//!
//! ## When to use this
//!
//! The full combination: a stable app name for XDG namespacing (like
//! `create_loader_xdg`) plus substitution values sourced from something
//! other than the process environment (like `create_loader_with_resolver`).
//! This is what a production service typically wants when its config lives
//! under a per-app XDG directory *and* its secrets come from a vault or KMS
//! rather than plain env vars.
//!
//! ## What this example does
//!
//! 1. Defines `StaticResolver`, a trivial `ValueResolver` returning a
//!    hardcoded value for one known name (a stand-in for a real secrets
//!    backend).
//! 2. Writes an `application.toml` with a `{{APP_GREETING}}` placeholder.
//! 3. Points `$CONFIGBUILDER_CONFIG_DIR` at the temp dir for deterministic
//!    XDG resolution under the app name `"docs-example-app"` (see
//!    `docs_create_loader_xdg` for why this env var isn't joined with the
//!    app name).
//! 4. Loads the section and asserts the resolver — not any env var —
//!    supplied the substituted value.
#![allow(unsafe_code)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{ConfigLoaderFactory, Loader as _, SubstitutionError, ValueResolver};

struct StaticResolver;
impl ValueResolver for StaticResolver {
    fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError> {
        match var_name {
            "APP_GREETING" => Ok("hello from a named-app resolver".to_string()),
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
    let loader = ConfigLoaderFactory::create_loader_xdg_with_resolver(
        "docs-example-app",
        Box::new(policy),
        Box::new(StaticResolver),
    )
    .expect("create_loader_xdg_with_resolver");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("CONFIGBUILDER_CONFIG_DIR") };

    assert_eq!(cfg.greeting, "hello from a named-app resolver");
    println!("create_loader_xdg_with_resolver: greeting={}", cfg.greeting);
}
