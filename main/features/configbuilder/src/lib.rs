//! Standalone, runtime-independent TOML section loader for swe-edge services.
//!
//! Provides XDG-aware, layered config section loading for any
//! `T: DeserializeOwned + Default`. Library crates can depend on this crate
//! directly without pulling in `swe-edge-runtime-main`.
//!
//! # Usage
//!
//! ```rust,ignore
//! use swe_edge_configbuilder::{create_loader, Loader};
//!
//! #[derive(serde::Deserialize, Default)]
//! struct CompletionConfig { model: String, max_tokens: u32 }
//!
//! let cfg: CompletionConfig =
//!     create_loader().load_section("application.completion")?;
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use crate::api::traits::config_builder::ConfigBuilder;
pub use crate::api::traits::loader::Loader;
pub use crate::api::traits::validator::Validator;
pub use saf::*;
