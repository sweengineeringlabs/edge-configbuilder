/// Default [`ValueResolver`] — resolves variables from the process
/// environment via `std::env::var`, preserving this crate's original
/// substitution behavior.
///
/// [`ValueResolver`]: crate::api::substitution::traits::value_resolver::ValueResolver
#[derive(Debug, Default, Clone, Copy)]
pub struct EnvValueResolver;
