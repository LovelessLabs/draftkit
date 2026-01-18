//! Preview image caching.
//!
//! Cache structure:
//! ```text
//! ~/.local/share/draftkit/
//! └── cache/
//!     └── previews/
//!         └── {component-id}/
//!             ├── light.png
//!             └── dark.png
//! ```
//!
//! Images are cached per-component, per-mode to enable incremental caching.

use std::fs;

use camino::Utf8PathBuf;

use crate::components::Mode;
use crate::data_dir::data_dir;

/// Get the previews cache directory path.
#[must_use]
pub fn previews_cache_dir() -> Option<Utf8PathBuf> {
    data_dir().map(|d| d.join("cache/previews"))
}

/// Get the path for a cached preview image.
#[must_use]
pub fn preview_cache_path(component_id: &str, mode: Mode) -> Option<Utf8PathBuf> {
    let dir = previews_cache_dir()?;
    let filename = format!("{}.png", mode.as_str());
    Some(dir.join(component_id).join(filename))
}

/// Check if a preview image is cached.
#[must_use]
#[allow(dead_code)] // Part of public API, will be used in future
pub fn is_cached(component_id: &str, mode: Mode) -> bool {
    preview_cache_path(component_id, mode)
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Get cached preview image bytes if it exists.
#[must_use]
pub fn get_cached(component_id: &str, mode: Mode) -> Option<Vec<u8>> {
    let path = preview_cache_path(component_id, mode)?;
    fs::read(path.as_std_path()).ok()
}

/// Store preview image bytes in the cache.
///
/// Creates parent directories as needed.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be created or the file cannot be written.
pub fn store_cached(component_id: &str, mode: Mode, data: &[u8]) -> std::io::Result<Utf8PathBuf> {
    let path = preview_cache_path(component_id, mode).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine previews cache directory",
        )
    })?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent.as_std_path())?;
    }

    fs::write(path.as_std_path(), data)?;
    Ok(path)
}

/// Get preview cache statistics.
#[derive(Debug, Clone, Default)]
pub struct PreviewCacheStats {
    /// Total number of cached preview files.
    pub file_count: usize,
    /// Total size in bytes.
    pub total_bytes: u64,
    /// Number of unique components (directories).
    pub component_count: usize,
}

/// Calculate preview cache statistics.
#[must_use]
pub fn get_stats() -> PreviewCacheStats {
    let Some(dir) = previews_cache_dir() else {
        return PreviewCacheStats::default();
    };

    if !dir.exists() {
        return PreviewCacheStats::default();
    }

    let mut stats = PreviewCacheStats::default();

    if let Ok(entries) = fs::read_dir(dir.as_std_path()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stats.component_count += 1;

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

/// Clear all cached preview images.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be removed.
pub fn clear_cache() -> std::io::Result<u64> {
    let Some(dir) = previews_cache_dir() else {
        return Ok(0);
    };

    if !dir.exists() {
        return Ok(0);
    }

    let stats = get_stats();
    let bytes_freed = stats.total_bytes;

    fs::remove_dir_all(dir.as_std_path())?;
    Ok(bytes_freed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_previews_cache_dir_returns_some() {
        let dir = previews_cache_dir();
        if let Some(path) = dir {
            assert!(path.as_str().contains("draftkit"));
            assert!(path.as_str().ends_with("previews"));
        }
    }

    #[test]
    fn test_preview_cache_path_format() {
        if let Some(path) = preview_cache_path("test-component", Mode::Light) {
            assert!(path.as_str().contains("test-component"));
            assert!(path.as_str().ends_with("light.png"));
        }
    }

    #[test]
    fn test_is_cached_returns_false_for_nonexistent() {
        assert!(!is_cached("nonexistent-preview-xyz", Mode::Light));
    }

    #[test]
    fn test_get_cached_returns_none_for_nonexistent() {
        assert!(get_cached("nonexistent-preview-xyz", Mode::Dark).is_none());
    }

    #[test]
    fn test_preview_cache_stats_default() {
        let stats = PreviewCacheStats::default();
        assert_eq!(stats.file_count, 0);
        assert_eq!(stats.total_bytes, 0);
        assert_eq!(stats.component_count, 0);
    }
}
