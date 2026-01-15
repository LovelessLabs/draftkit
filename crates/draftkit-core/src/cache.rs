//! Local component cache for on-demand fetched content.
//!
//! Cache structure:
//! ```text
//! ~/.local/share/draftkit/
//! └── cache/
//!     └── components/
//!         └── {component-id}/
//!             ├── react-v4-light.jsx
//!             ├── react-v4-dark.jsx
//!             ├── vue-v4-light.vue
//!             └── ...
//! ```
//!
//! Files are stored per-component, per-variant to enable incremental caching
//! as users request different components.

use std::fs;

use camino::Utf8PathBuf;

use crate::components::{Framework, Mode};
use crate::data_dir::data_dir;

/// Get the cache directory path.
#[must_use]
pub fn cache_dir() -> Option<Utf8PathBuf> {
    data_dir().map(|d| d.join("cache"))
}

/// Get the path for a cached component file.
///
/// Returns `None` if the data directory cannot be determined.
#[must_use]
pub fn component_cache_path(
    component_id: &str,
    framework: Framework,
    mode: Mode,
) -> Option<Utf8PathBuf> {
    let dir = cache_dir()?;
    let filename = format!(
        "{}-v4-{}.{}",
        framework.as_str(),
        mode.as_str(),
        framework.file_extension()
    );
    Some(dir.join("components").join(component_id).join(filename))
}

/// Check if a component variant is cached.
#[must_use]
pub fn is_cached(component_id: &str, framework: Framework, mode: Mode) -> bool {
    component_cache_path(component_id, framework, mode)
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Get cached component code if it exists.
#[must_use]
pub fn get_cached(component_id: &str, framework: Framework, mode: Mode) -> Option<String> {
    let path = component_cache_path(component_id, framework, mode)?;
    fs::read_to_string(path.as_std_path()).ok()
}

/// Store component code in the cache.
///
/// Creates parent directories as needed.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be created or the file cannot be written.
pub fn store_cached(
    component_id: &str,
    framework: Framework,
    mode: Mode,
    code: &str,
) -> std::io::Result<Utf8PathBuf> {
    let path = component_cache_path(component_id, framework, mode).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine cache directory",
        )
    })?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent.as_std_path())?;
    }

    fs::write(path.as_std_path(), code)?;
    Ok(path)
}

/// Get cache statistics.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of cached component files
    pub file_count: usize,
    /// Total size in bytes
    pub total_bytes: u64,
    /// Number of unique components (directories)
    pub component_count: usize,
}

/// Calculate cache statistics.
#[must_use]
pub fn get_stats() -> CacheStats {
    let Some(dir) = cache_dir() else {
        return CacheStats::default();
    };

    let components_dir = dir.join("components");
    if !components_dir.exists() {
        return CacheStats::default();
    }

    let mut stats = CacheStats::default();

    // Count component directories
    if let Ok(entries) = fs::read_dir(components_dir.as_std_path()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stats.component_count += 1;

                // Count files in component directory
                if let Ok(files) = fs::read_dir(&path) {
                    for file in files.flatten() {
                        let file_path = file.path();
                        if file_path.is_file() {
                            stats.file_count += 1;
                            if let Ok(meta) = file.metadata() {
                                stats.total_bytes += meta.len();
                            }
                        }
                    }
                }
            }
        }
    }

    stats
}

/// Clear all cached components.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be removed.
pub fn clear_cache() -> std::io::Result<u64> {
    let Some(dir) = cache_dir() else {
        return Ok(0);
    };

    let components_dir = dir.join("components");
    if !components_dir.exists() {
        return Ok(0);
    }

    let stats = get_stats();
    let bytes_freed = stats.total_bytes;

    fs::remove_dir_all(components_dir.as_std_path())?;
    Ok(bytes_freed)
}

/// Get the cache directory path as a string (for display).
#[must_use]
pub fn cache_path_string() -> Option<String> {
    cache_dir().map(|p| p.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_dir_returns_some() {
        // On most systems, this should return Some
        let dir = cache_dir();
        if let Some(path) = dir {
            assert!(path.as_str().contains("draftkit"));
            assert!(path.as_str().ends_with("cache"));
        }
    }

    #[test]
    fn test_component_cache_path_format() {
        if let Some(path) = component_cache_path("test-component", Framework::React, Mode::Light) {
            assert!(path.as_str().contains("test-component"));
            assert!(path.as_str().contains("react-v4-light"));
            assert!(path.as_str().ends_with(".jsx"));
        }
    }

    #[test]
    fn test_is_cached_returns_false_for_nonexistent() {
        assert!(!is_cached(
            "nonexistent-component-xyz",
            Framework::React,
            Mode::Light
        ));
    }

    #[test]
    fn test_get_cached_returns_none_for_nonexistent() {
        assert!(get_cached("nonexistent-component-xyz", Framework::Vue, Mode::Dark).is_none());
    }

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats::default();
        assert_eq!(stats.file_count, 0);
        assert_eq!(stats.total_bytes, 0);
        assert_eq!(stats.component_count, 0);
    }

    #[test]
    fn test_get_stats_does_not_panic() {
        // Just verify it doesn't panic, even if cache doesn't exist
        let _ = get_stats();
    }

    #[test]
    fn test_cache_path_string() {
        if let Some(path) = cache_path_string() {
            assert!(path.contains("draftkit"));
        }
    }
}
