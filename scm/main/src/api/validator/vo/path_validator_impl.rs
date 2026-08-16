use crate::api::validator::traits::validator_ops::ValidatorOps;

/// Public concrete path validator returned by `create_validator`.
pub struct PathValidatorImpl {
    pub(crate) ops: Box<dyn ValidatorOps>,
}
