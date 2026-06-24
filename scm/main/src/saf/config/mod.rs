//! SAF config facade modules for consumer-facing builder entry points.

mod config_builder_bound_svc;
mod config_builder_svc;
mod config_section_svc;

#[doc(hidden)]
pub use config_builder_svc::CONFIG_BUILDER_SVC;
