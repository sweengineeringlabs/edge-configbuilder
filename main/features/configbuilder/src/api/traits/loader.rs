use crate::api::error::config_error::ConfigError;

/// Load typed TOML sections from a layered config chain.
///
/// Implementations merge config directories in order (later wins) and return
/// `Ok(T::default())` when the requested key is absent from every source.
/// Environment variable substitution (`{{VAR_NAME}}` syntax) is optionally supported
/// when the loader was created with a [`SubstitutionPolicy`].
///
/// The concrete implementation is [`SectionLoaderImpl`], obtained from the
/// [`ConfigLoaderFactory`] family of functions.  This trait is in scope for
/// generic code that accepts any loader; concrete call sites use the inherent
/// methods on [`SectionLoaderImpl`] directly.
///
/// [`SubstitutionPolicy`]: crate::SubstitutionPolicy
/// [`SectionLoaderImpl`]: crate::SectionLoaderImpl
/// [`ConfigLoaderFactory`]: crate::ConfigLoaderFactory
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::{ConfigLoaderFactory, Loader};
///
/// #[derive(serde::Deserialize, Default)]
/// struct AuthConfig { token_ttl_secs: u64 }
///
/// // Loader has generic methods so it is not dyn-compatible.
/// // Use a concrete type or a generic bound instead of &dyn Loader.
/// fn load_auth<L: Loader>(loader: &L) -> AuthConfig {
///     loader.load_section("auth").unwrap_or_default()
/// }
///
/// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
/// let cfg = load_auth(&loader);
/// ```
pub trait Loader {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all
    /// configured directories, merging with last-wins semantics.
    ///
    /// Returns `Ok(T::default())` when the key is absent from every file.
    /// If the loader was created with a substitution policy, `{{VAR_NAME}}`
    /// placeholders are substituted with environment variable values after loading.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigLoaderFactory, Loader};
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct TlsConfig { cert_path: String }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
    /// let tls: TlsConfig = loader.load_section("tls").expect("readable");
    /// ```
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default;

    /// Validate that all configured directories are accessible.
    ///
    /// Non-existent paths are permitted (skipped at load time); a path that
    /// exists but is not a directory is a hard error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigLoaderFactory, Loader};
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("/etc/my-app");
    /// loader.validate().expect("must be a directory if it exists");
    /// ```
    fn validate(&self) -> Result<(), ConfigError>;
}
