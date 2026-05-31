use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::substitution_policy::SubstitutionPolicy;
use crate::api::types::config::ConfigBuilderImpl;
use crate::api::types::config::ConfigLoaderFactory;
use crate::api::types::feature::feature_state::FeatureState;
use crate::api::types::path_validator_impl::PathValidatorImpl;
use crate::api::types::section_loader_impl::SectionLoaderImpl;
use crate::api::types::substitution_config_builder_impl::SubstitutionConfigBuilderImpl;
use crate::core::{DefaultConfigBuilder, DefaultSectionLoader, DefaultValidator};

// ── Extension impls for the builder types ────────────────────────────────────
//
// These impls live in saf/ so that the api/types/ files carry no dependency on
// core/ (SEA rules 46 and 116).  The structs in api/types/ store only primitive
// data; saf/ wires them to the concrete DefaultConfigBuilder at call time.

impl ConfigBuilderImpl {
    /// Consume the builder and return a ready-to-use section loader.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let core = DefaultConfigBuilder {
            name: self.name,
            version: self.version,
            config_dirs: self.config_dirs,
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(core),
        })
    }
}

impl SubstitutionConfigBuilderImpl {
    /// Consume the builder and return a ready-to-use section loader with
    /// substitution support.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let mut core = DefaultConfigBuilder {
            name: self.name,
            version: self.version,
            config_dirs: self.config_dirs,
        }
        .build_loader_internal()?;
        core.substitution_policy = Some(self.policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(core),
        })
    }
}

// ── ConfigLoaderFactory — all factory fns as associated functions on a type ───

impl ConfigLoaderFactory {
    /// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
    /// components, or if the resolved path exists but is not a directory.
    pub fn create_loader() -> Result<SectionLoaderImpl, ConfigError> {
        let loader = DefaultConfigBuilder {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a loader reading from a single explicit directory.
    pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> SectionLoaderImpl {
        SectionLoaderImpl {
            ops: Box::new(DefaultSectionLoader {
                config_dirs: vec![dir.into()],
                substitution_policy: None,
            }),
        }
    }

    /// Create a loader following the XDG Base Directory chain for `app_name`.
    ///
    /// Layer order (last wins):
    /// - `$XDG_CONFIG_DIRS/<app_name>/` (lowest priority)
    /// - `$XDG_CONFIG_HOME/<app_name>/`
    /// - `$SWE_EDGE_CONFIG_DIR/` (if set)
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    pub fn create_loader_xdg(app_name: &str) -> Result<SectionLoaderImpl, ConfigError> {
        let loader = DefaultConfigBuilder {
            name: app_name.to_owned(),
            version: String::new(),
            config_dirs: Vec::new(),
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a path validator.
    pub fn create_validator() -> PathValidatorImpl {
        PathValidatorImpl {
            ops: Box::new(DefaultValidator),
        }
    }

    /// Create a config builder pre-seeded with this package's name and version.
    ///
    /// Uses XDG Base Directory resolution for the package name so callers do not
    /// need to call the builder methods manually.
    pub fn create_config_builder() -> ConfigBuilderImpl {
        let mut b = ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Load the section at `key` as an optional feature, returning `Disabled` when absent.
    ///
    /// Presence of the section in any config file enables the feature; absence
    /// disables it without raising an error.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] for unreadable files or size-limit violations,
    /// and [`ConfigError::Parse`] for malformed TOML or deserialisation failures.
    pub fn load_feature_section<T>(
        loader: &SectionLoaderImpl,
        key: &str,
    ) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        loader.load_optional_section(key)
    }

    /// Create a loader with environment variable substitution support.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
    /// components, or if the resolved path exists but is not a directory.
    pub fn create_loader_with_substitution(
        policy: Box<dyn SubstitutionPolicy>,
    ) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = DefaultConfigBuilder {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
        }
        .build_loader_internal()?;
        loader.substitution_policy = Some(policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a loader from a single directory with substitution support.
    pub fn create_loader_for_dir_with_substitution(
        dir: impl Into<PathBuf>,
        policy: Box<dyn SubstitutionPolicy>,
    ) -> SectionLoaderImpl {
        SectionLoaderImpl {
            ops: Box::new(DefaultSectionLoader {
                config_dirs: vec![dir.into()],
                substitution_policy: Some(policy),
            }),
        }
    }

    /// Create an XDG-aware loader with substitution support.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    pub fn create_loader_xdg_with_substitution(
        app_name: &str,
        policy: Box<dyn SubstitutionPolicy>,
    ) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = DefaultConfigBuilder {
            name: app_name.to_owned(),
            version: String::new(),
            config_dirs: Vec::new(),
        }
        .build_loader_internal()?;
        loader.substitution_policy = Some(policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a config builder that supports substitution and custom paths.
    ///
    /// Returns a builder pre-seeded with the calling package's name and version.
    pub fn create_config_builder_with_substitution(
        policy: Box<dyn SubstitutionPolicy>,
    ) -> SubstitutionConfigBuilderImpl {
        SubstitutionConfigBuilderImpl {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
            policy,
        }
    }
}
