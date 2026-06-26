use crate::api::substitution::traits::substitution_policy::SubstitutionPolicy;

/// Combines multiple [`SubstitutionPolicy`] implementations with AND logic.
pub struct CompositePolicy {
    pub(crate) policies: Vec<Box<dyn SubstitutionPolicy>>,
}
