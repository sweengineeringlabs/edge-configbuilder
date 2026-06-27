//! [`FeatureState`] — presence-driven feature gate for optional config sections.

use crate::FeatureState;

impl<T: std::fmt::Debug> std::fmt::Debug for FeatureState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enabled(v) => f.debug_tuple("Enabled").field(v).finish(),
            Self::Disabled => write!(f, "Disabled"),
        }
    }
}

impl<T: Clone> Clone for FeatureState<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Enabled(v) => Self::Enabled(v.clone()),
            Self::Disabled => Self::Disabled,
        }
    }
}

impl<T: PartialEq> PartialEq for FeatureState<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Enabled(a), Self::Enabled(b)) => a == b,
            (Self::Disabled, Self::Disabled) => true,
            _ => false,
        }
    }
}

impl<T> From<Option<T>> for FeatureState<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => Self::Enabled(v),
            None => Self::Disabled,
        }
    }
}

impl<T> From<FeatureState<T>> for Option<T> {
    fn from(state: FeatureState<T>) -> Self {
        state.into_option()
    }
}

impl<T> FeatureState<T> {
    /// Return `true` when the state holds an enabled value.
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    /// Return `true` when the state is disabled.
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Convert into an `Option<T>`, discarding disabled states.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Enabled(v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Borrow the inner value when enabled.
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Enabled(v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Map the inner value when enabled.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => FeatureState::Enabled(f(v)),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Chain another state-producing operation when enabled.
    pub fn and_then<U>(self, f: impl FnOnce(T) -> FeatureState<U>) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => f(v),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Return the inner value or the provided default when disabled.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => default,
        }
    }

    /// Return the inner value or compute one lazily when disabled.
    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => f(),
        }
    }

    /// Return the enabled value or `T::default()` when disabled.
    pub fn enabled_or_default(self) -> T
    where
        T: Default,
    {
        self.unwrap_or_else(T::default)
    }
}

impl<T> crate::api::FeatureStateOps for FeatureState<T> {
    type Value = T;

    fn is_enabled(&self) -> bool {
        FeatureState::is_enabled(self)
    }

    fn is_disabled(&self) -> bool {
        FeatureState::is_disabled(self)
    }

    fn into_option(self) -> Option<Self::Value> {
        FeatureState::into_option(self)
    }

    fn as_option(&self) -> Option<&Self::Value> {
        FeatureState::as_option(self)
    }

    fn map<U>(self, f: impl FnOnce(Self::Value) -> U) -> FeatureState<U> {
        FeatureState::map(self, f)
    }

    fn and_then<U>(self, f: impl FnOnce(Self::Value) -> FeatureState<U>) -> FeatureState<U> {
        FeatureState::and_then(self, f)
    }

    fn unwrap_or(self, default: Self::Value) -> Self::Value {
        FeatureState::unwrap_or(self, default)
    }

    fn unwrap_or_else(self, f: impl FnOnce() -> Self::Value) -> Self::Value {
        FeatureState::unwrap_or_else(self, f)
    }

    fn enabled_or_default(self) -> Self::Value
    where
        Self::Value: Default,
    {
        FeatureState::enabled_or_default(self)
    }
}
