/// Application name, sourced from the Cargo package metadata at compile time.
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Application version, sourced from the Cargo package metadata at compile time.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
