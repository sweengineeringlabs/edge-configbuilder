use crate::api::FeatureState;

/// Operations on [`FeatureState<T>`] — presence-driven feature gate.
///
/// Implemented by [`FeatureState<T>`] in the `core/` layer.
///
/// [`FeatureState<T>`]: crate::FeatureState
pub trait FeatureStateOps: Sized {
    /// The inner value type when the state is enabled.
    type Value;

    /// Return `true` when the state holds an enabled value.
    fn is_enabled(&self) -> bool;

    /// Return `true` when the state is disabled.
    fn is_disabled(&self) -> bool;

    /// Convert into an `Option<Self::Value>`, discarding disabled states.
    fn into_option(self) -> Option<Self::Value>;

    /// Borrow the inner value when enabled.
    fn as_option(&self) -> Option<&Self::Value>;

    /// Map the inner value when enabled.
    fn map<U>(self, f: impl FnOnce(Self::Value) -> U) -> FeatureState<U>;

    /// Chain another state-producing operation when enabled.
    fn and_then<U>(self, f: impl FnOnce(Self::Value) -> FeatureState<U>) -> FeatureState<U>;

    /// Return the inner value or the provided default when disabled.
    fn unwrap_or(self, default: Self::Value) -> Self::Value;

    /// Return the inner value or compute one lazily when disabled.
    fn unwrap_or_else(self, f: impl FnOnce() -> Self::Value) -> Self::Value;

    /// Return the enabled value or `Self::Value::default()` when disabled.
    fn enabled_or_default(self) -> Self::Value
    where
        Self::Value: Default;
}
