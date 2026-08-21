//! `ConfigBuilder::with_version(version)` — sets the application version
//! string.
//!
//! ## When to use this
//!
//! Purely metadata: unlike `with_name`, it has **no effect on directory
//! resolution** — no `<version>` component appears in any resolved path.
//! `create_config_builder()` defaults it to `CARGO_PKG_VERSION`. Set it
//! explicitly if you want the builder's `.version()` accessor to reflect
//! something other than your own crate's compiled-in version — e.g. a
//! config-format version distinct from your binary's release version, if
//! your application chooses to track those separately and inspects
//! `.version()` for its own purposes.
//!
//! ## What this example does
//!
//! Calls `with_version("2.0.0")` and reads it straight back via
//! `ConfigBuilder::version()`, since that's the entirety of this method's
//! observable behavior — there's no loader-building step to demonstrate
//! because version never influences where or how config is loaded.

use configbuilder::{ConfigBuilder as _, ConfigLoaderFactory};

fn main() {
    let builder = ConfigLoaderFactory::create_config_builder().with_version("2.0.0");
    assert_eq!(builder.version(), "2.0.0");
    println!("with_version: version reflected = {}", builder.version());
}
