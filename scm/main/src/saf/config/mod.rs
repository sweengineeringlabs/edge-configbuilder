//! SAF config facade modules for consumer-facing builder entry points.

mod config_builder_bound_saf;
mod config_builder_saf;
mod config_section_saf;

#[doc(hidden)]
pub use config_builder_saf::CONFIG_BUILDER_SVC;
