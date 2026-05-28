//! Interface counterpart for [`crate::core::loader::default_section_loader::DefaultSectionLoader`].

/// API contract marker for the default section loader.
///
/// The concrete implementor is
/// `crate::core::loader::default_section_loader::DefaultSectionLoader`, which
/// implements [`crate::api::traits::loader::Loader`] and
/// [`crate::api::traits::feature_loader::FeatureLoader`].
pub trait DefaultSectionLoader:
    crate::api::traits::loader::Loader + crate::api::traits::feature_loader::FeatureLoader
{
}
