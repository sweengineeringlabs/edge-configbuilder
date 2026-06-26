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
