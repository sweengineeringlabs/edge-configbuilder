//! Loader theme — typed/optional TOML section loading and feature tracking.
//!
//! Owns the loading ports ([`Loader`], [`FeatureLoader`], [`LoaderOps`]), the
//! section-marker traits ([`ConfigSection`], [`OptionalSection`]), the public
//! [`SectionLoaderImpl`], the feature-tracking type family (state, record,
//! registry, summary, metadata), the internal [`Topology`] resolver, and the
//! API marker for the core section loader.
//!
//! [`Loader`]: traits::loader::Loader
//! [`FeatureLoader`]: traits::feature_loader::FeatureLoader
//! [`LoaderOps`]: traits::loader_ops::LoaderOps
//! [`ConfigSection`]: traits::config_section::ConfigSection
//! [`OptionalSection`]: traits::optional_section::OptionalSection
//! [`SectionLoaderImpl`]: types::section_loader_impl::SectionLoaderImpl
//! [`Topology`]: types::topology::Topology

pub mod traits;
pub mod types;

pub use traits::section_loader_bound::SectionLoaderBound;
