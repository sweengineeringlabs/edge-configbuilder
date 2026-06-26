//! [`FeatureRegistry`] — startup feature collector and dependency validator.

use crate::FeatureRegistry;

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}
