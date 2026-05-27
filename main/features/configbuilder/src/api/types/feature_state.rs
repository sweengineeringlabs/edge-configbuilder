//! [`FeatureState`] — presence-driven feature gate for optional config sections.

/// Whether an optional TOML section is present and active.
///
/// `load_optional_section` returns `Enabled(T)` when the key exists in any
/// config file and `Disabled` when it is absent from all of them.  Absent
/// means the feature is intentionally off — no error is raised.
///
/// # Example
///
/// ```rust,ignore
/// match load_feature_section::<BrokerConfig>(&loader, "message_broker")? {
///     FeatureState::Enabled(cfg) => start_broker(cfg),
///     FeatureState::Disabled     => {}  // broker not configured
/// }
/// ```
pub enum FeatureState<T> {
    /// The section key is present in TOML and deserialized successfully into `T`.
    Enabled(T),
    /// The section key is absent from every config file — the feature is off.
    Disabled,
}

impl<T> FeatureState<T> {
    /// Returns `true` when the section is present and active.
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    /// Returns `true` when the section is absent.
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Consumes `self` and returns `Some(T)` when enabled, `None` when disabled.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Enabled(v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Returns `Some(&T)` when enabled, `None` when disabled.
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Enabled(ref v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Transforms the inner value with `f` when enabled; propagates `Disabled` unchanged.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => FeatureState::Enabled(f(v)),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Chains a fallible feature transition when enabled; short-circuits on `Disabled`.
    pub fn and_then<U>(self, f: impl FnOnce(T) -> FeatureState<U>) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => f(v),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Returns the inner value when enabled, or `default` when disabled.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => default,
        }
    }

    /// Returns the inner value when enabled, or the result of `f` when disabled.
    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => f(),
        }
    }
}

impl<T: Default> FeatureState<T> {
    /// Returns the inner value when enabled, or `T::default()` when disabled.
    pub fn enabled_or_default(self) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => T::default(),
        }
    }
}

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
