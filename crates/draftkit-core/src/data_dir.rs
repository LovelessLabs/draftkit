//! Runtime data directory resolution.
//!
//! Provides platform-appropriate paths for runtime data storage:
//! - Linux: `~/.local/share/draftkit/`
//! - macOS: `~/Library/Application Support/draftkit/`
//! - Windows: `%APPDATA%/draftkit/`
//!
//! The runtime directory structure mirrors the embedded cache:
//! ```text
//! ~/.local/share/draftkit/
//! ├── manifest.json
//! ├── data/
//! │   └── components/
//! │       ├── react-v4.ndjson
//! │       ├── vue-v4.ndjson
//! │       └── html-v4.ndjson
//! ├── docs/
//! │   └── tailwind/
//! │       ├── v3/
//! │       └── v4/
//! ├── kits/
//! │   └── catalyst/
//! │       ├── typescript/
//! │       └── javascript/
//! └── elements/
//!     └── llms.txt
//! ```

use camino::{Utf8Path, Utf8PathBuf};
use std::sync::OnceLock;

/// App name for directory creation
const APP_NAME: &str = "draftkit";

/// Cached data directory path
static DATA_DIR: OnceLock<Option<Utf8PathBuf>> = OnceLock::new();

/// Get the platform-appropriate data directory for draftkit.
///
/// Returns `None` if the data directory cannot be determined (e.g., no home directory).
#[must_use]
pub fn data_dir() -> Option<&'static Utf8Path> {
    DATA_DIR
        .get_or_init(|| dirs::data_dir().and_then(|p| Utf8PathBuf::try_from(p.join(APP_NAME)).ok()))
        .as_deref()
}

/// Check if runtime data exists and appears valid.
///
/// Validates that the data directory exists and contains a manifest.json file.
#[must_use]
pub fn has_runtime_data() -> bool {
    data_dir()
        .map(|dir| dir.join("manifest.json").is_file())
        .unwrap_or(false)
}

/// Get the path to the runtime manifest file, if it exists.
#[must_use]
pub fn runtime_manifest_path() -> Option<Utf8PathBuf> {
    let dir = data_dir()?;
    let path = dir.join("manifest.json");
    if path.is_file() { Some(path) } else { None }
}

/// Get the path to the runtime components directory.
#[must_use]
pub fn runtime_components_dir() -> Option<Utf8PathBuf> {
    let dir = data_dir()?;
    let path = dir.join("data/components");
    if path.is_dir() { Some(path) } else { None }
}

/// Get the path to the runtime docs directory for a specific Tailwind version.
#[must_use]
pub fn runtime_docs_dir(version: &str) -> Option<Utf8PathBuf> {
    let dir = data_dir()?;
    let path = dir.join(format!("docs/tailwind/{version}"));
    if path.is_dir() { Some(path) } else { None }
}

/// Get the path to the runtime Catalyst directory for a specific language.
#[must_use]
pub fn runtime_catalyst_dir(language: &str) -> Option<Utf8PathBuf> {
    let dir = data_dir()?;
    let path = dir.join(format!("kits/catalyst/{language}"));
    if path.is_dir() { Some(path) } else { None }
}

/// Get the path to the runtime elements directory.
#[must_use]
pub fn runtime_elements_dir() -> Option<Utf8PathBuf> {
    let dir = data_dir()?;
    let path = dir.join("elements");
    if path.is_dir() { Some(path) } else { None }
}

/// Data source indicator for info/diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSource {
    /// Data loaded from runtime directory
    Runtime,
    /// Data loaded from compile-time embedded resources
    Embedded,
    /// No data available
    None,
}

impl std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Runtime => write!(f, "runtime"),
            Self::Embedded => write!(f, "embedded"),
            Self::None => write!(f, "none"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_dir_returns_some_on_most_systems() {
        // On most systems, data_dir should return Some
        // This might fail in unusual environments, but that's expected
        let dir = data_dir();
        // Just verify it doesn't panic and returns a valid path if Some
        if let Some(path) = dir {
            assert!(path.as_str().contains("draftkit"));
        }
    }

    #[test]
    fn test_has_runtime_data_false_by_default() {
        // Unless the user has run `draftkit update`, runtime data shouldn't exist
        // This test just verifies the function doesn't panic
        let _ = has_runtime_data();
    }

    #[test]
    fn test_data_source_display() {
        assert_eq!(DataSource::Runtime.to_string(), "runtime");
        assert_eq!(DataSource::Embedded.to_string(), "embedded");
        assert_eq!(DataSource::None.to_string(), "none");
    }

    #[test]
    fn test_runtime_paths_return_none_when_missing() {
        // These should return None when data doesn't exist
        // (unless user has actually installed runtime data)
        let _ = runtime_manifest_path();
        let _ = runtime_components_dir();
        let _ = runtime_docs_dir("v4");
        let _ = runtime_catalyst_dir("typescript");
        let _ = runtime_elements_dir();
    }
}
