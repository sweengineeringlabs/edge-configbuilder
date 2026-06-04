//! [`FeatureState`] — presence-driven feature gate for optional config sections.

/// Whether an optional TOML section is present and active.
///
/// `load_optional_section` returns `Enabled(T)` when the key exists in any
/// config file and `Disabled` when it is absent from all of them.  Absent
/// means the feature is intentionally off — no error is raised.
///
/// Use [`FeatureRegistry::load`] for full startup coordination (graceful
/// degradation, dependency validation, observer hooks). Use
/// [`ConfigLoaderFactory::load_feature_section`] for one-off ad-hoc loading.
///
/// [`FeatureRegistry::load`]: crate::FeatureRegistry::load
/// [`ConfigLoaderFactory::load_feature_section`]: crate::ConfigLoaderFactory::load_feature_section
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::FeatureState;
///
/// let enabled: FeatureState<u32> = FeatureState::Enabled(42);
/// assert!(enabled.is_enabled());
/// assert_eq!(enabled.into_option(), Some(42));
///
/// let disabled: FeatureState<u32> = FeatureState::Disabled;
/// assert!(disabled.is_disabled());
/// assert_eq!(disabled.into_option(), None);
/// ```
pub enum FeatureState<T> {
    /// The section key is present in TOML and deserialized successfully into `T`.
    Enabled(T),
    /// The section key is absent from every config file — the feature is off.
    Disabled,
}

impl<T> FeatureState<T> {
    /// Returns `true` when the section is present and active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// assert!(FeatureState::Enabled(1_u32).is_enabled());
    /// assert!(!FeatureState::<u32>::Disabled.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    /// Returns `true` when the section is absent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// assert!(FeatureState::<u32>::Disabled.is_disabled());
    /// assert!(!FeatureState::Enabled(1_u32).is_disabled());
    /// ```
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Consumes `self` and returns `Some(T)` when enabled, `None` when disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// assert_eq!(FeatureState::Enabled(99_u32).into_option(), Some(99));
    /// assert_eq!(FeatureState::<u32>::Disabled.into_option(), None);
    /// ```
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Enabled(v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Returns `Some(&T)` when enabled, `None` when disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// let state = FeatureState::Enabled(String::from("hello"));
    /// assert_eq!(state.as_option(), Some(&String::from("hello")));
    /// assert_eq!(FeatureState::<String>::Disabled.as_option(), None);
    /// ```
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Enabled(ref v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Transforms the inner value with `f` when enabled; propagates `Disabled` unchanged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// let doubled = FeatureState::Enabled(21_u32).map(|n| n * 2);
    /// assert_eq!(doubled, FeatureState::Enabled(42));
    ///
    /// let still_disabled = FeatureState::<u32>::Disabled.map(|n| n * 2);
    /// assert_eq!(still_disabled, FeatureState::Disabled);
    /// ```
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => FeatureState::Enabled(f(v)),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Chains a fallible feature transition when enabled; short-circuits on `Disabled`.
    ///
    /// Use this when the next step may itself produce `Disabled` (e.g. a secondary
    /// validation that gates the feature further).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    ///
    /// // Gate a sub-feature on a field inside the outer config.
    /// let outer: FeatureState<bool> = FeatureState::Enabled(true);
    /// let inner = outer.and_then(|tls_on| {
    ///     if tls_on { FeatureState::Enabled("tls") } else { FeatureState::Disabled }
    /// });
    /// assert_eq!(inner, FeatureState::Enabled("tls"));
    /// ```
    pub fn and_then<U>(self, f: impl FnOnce(T) -> FeatureState<U>) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => f(v),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Returns the inner value when enabled, or `default` when disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// assert_eq!(FeatureState::Enabled(7_u32).unwrap_or(0), 7);
    /// assert_eq!(FeatureState::<u32>::Disabled.unwrap_or(0), 0);
    /// ```
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => default,
        }
    }

    /// Returns the inner value when enabled, or the result of `f` when disabled.
    ///
    /// Prefer this over [`unwrap_or`] when the fallback is expensive to compute.
    ///
    /// [`unwrap_or`]: FeatureState::unwrap_or
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// let result = FeatureState::<u32>::Disabled.unwrap_or_else(|| 42);
    /// assert_eq!(result, 42);
    /// ```
    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => f(),
        }
    }
}

impl<T: Default> FeatureState<T> {
    /// Returns the inner value when enabled, or `T::default()` when disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureState;
    /// assert_eq!(FeatureState::Enabled(5_u32).enabled_or_default(), 5);
    /// assert_eq!(FeatureState::<u32>::Disabled.enabled_or_default(), 0);
    /// ```
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
