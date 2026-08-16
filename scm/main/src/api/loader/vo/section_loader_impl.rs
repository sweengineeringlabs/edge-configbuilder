//! Public concrete section loader returned by the `saf/` factory functions.

use crate::api::loader::traits::loader_ops::LoaderOps;

/// A ready-to-use section loader produced by the `create_loader*` factory functions.
pub struct SectionLoaderImpl {
    pub(crate) ops: Box<dyn LoaderOps>,
}
