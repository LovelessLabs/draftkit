//! Error types for preview generation.

use thiserror::Error;

/// Error type for preview operations.
#[derive(Debug, Error)]
pub enum PreviewError {
    /// Failed to download a preview image.
    #[error("Failed to download preview: {0}")]
    Download(String),

    /// Failed to process or decode an image.
    #[error("Image processing error: {0}")]
    Image(String),

    /// No components provided for preview.
    #[error("No components provided for preview")]
    NoComponents,

    /// Component has no preview URL.
    #[error("Component '{0}' has no preview image")]
    NoPreviewUrl(String),

    /// I/O error (cache, temp files).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}
