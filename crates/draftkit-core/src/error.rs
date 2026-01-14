//! Error types for draftkit-core

use camino::Utf8PathBuf;
use thiserror::Error;

/// Errors that can occur when working with configuration.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Failed to load configuration from a file.
    #[error("failed to load config from {path}: {source}")]
    Load {
        path: Utf8PathBuf,
        #[source]
        source: config::ConfigError,
    },

    /// Failed to deserialize configuration.
    #[error("invalid configuration: {0}")]
    Deserialize(#[from] config::ConfigError),

    /// Configuration file not found after searching all locations.
    #[error("no configuration file found")]
    NotFound,
}

/// Result type alias using [`ConfigError`].
pub type ConfigResult<T> = Result<T, ConfigError>;
