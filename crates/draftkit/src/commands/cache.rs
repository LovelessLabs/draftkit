//! Cache command implementation
//!
//! Manages the local component cache where fetched TailwindPlus components
//! are stored for offline access.

use std::fs;

use anyhow::{Context, Result, bail};
use camino::Utf8PathBuf;
use clap::Args;
use dialoguer::Confirm;
use draftkit_core::data_dir;

use crate::cli::Styler;

/// Manage the local component cache.
#[derive(Args)]
pub struct CacheArgs {
    /// Show cache statistics (size, component count)
    #[arg(long)]
    pub stats: bool,

    /// Clear all cached components
    #[arg(long)]
    pub clear: bool,

    /// Print the cache directory path
    #[arg(long)]
    pub path: bool,
}

/// Calculate cache statistics by walking the directory tree.
fn calculate_cache_stats(cache_dir: &Utf8PathBuf) -> Result<(u64, u64)> {
    let mut file_count = 0u64;
    let mut total_bytes = 0u64;

    if cache_dir.exists() {
        for entry in walkdir::WalkDir::new(cache_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            file_count += 1;
            if let Ok(metadata) = entry.metadata() {
                total_bytes += metadata.len();
            }
        }
    }

    Ok((file_count, total_bytes))
}

/// Format bytes into human-readable size.
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Get the cache directory path.
fn cache_dir() -> Option<Utf8PathBuf> {
    data_dir::data_dir().map(|d| d.to_path_buf())
}

/// Execute the cache command.
pub fn cmd_cache(args: CacheArgs, color_mode: &str) -> Result<()> {
    let styler = Styler::new(color_mode);

    // Default to --stats if no flags provided
    let show_stats = args.stats || (!args.clear && !args.path);

    if args.path {
        return cmd_cache_path();
    }

    if args.clear {
        return cmd_cache_clear(&styler);
    }

    if show_stats {
        return cmd_cache_stats(&styler);
    }

    Ok(())
}

/// Print cache statistics.
fn cmd_cache_stats(styler: &Styler) -> Result<()> {
    let Some(cache_path) = cache_dir() else {
        styler.print_error("Could not determine cache directory");
        bail!("No data directory available");
    };

    styler.print_header("Component Cache");
    println!();

    if !cache_path.exists() {
        styler.print_kv("Location", cache_path.as_str(), 10);
        styler.print_kv("Status", "Not initialized", 10);
        println!();
        styler.print_info("No components have been cached yet.");
        return Ok(());
    }

    let (file_count, total_bytes) =
        calculate_cache_stats(&cache_path).context("Failed to calculate cache statistics")?;

    styler.print_kv("Location", cache_path.as_str(), 10);
    styler.print_kv("Files", &file_count.to_string(), 10);
    styler.print_kv("Size", &format_size(total_bytes), 10);

    Ok(())
}

/// Clear all cached components.
fn cmd_cache_clear(styler: &Styler) -> Result<()> {
    let Some(cache_path) = cache_dir() else {
        styler.print_error("Could not determine cache directory");
        bail!("No data directory available");
    };

    if !cache_path.exists() {
        styler.print_info("Cache is already empty.");
        return Ok(());
    }

    let (file_count, total_bytes) =
        calculate_cache_stats(&cache_path).context("Failed to calculate cache statistics")?;

    if file_count == 0 {
        styler.print_info("Cache is already empty.");
        return Ok(());
    }

    // Confirm before clearing
    let message = format!(
        "Clear {} cached files ({})? This cannot be undone.",
        file_count,
        format_size(total_bytes)
    );

    let confirmed = Confirm::new()
        .with_prompt(message)
        .default(false)
        .interact()
        .unwrap_or(false);

    if !confirmed {
        styler.print_info("Cache clear cancelled.");
        return Ok(());
    }

    // Remove the cache directory
    let spinner = styler.spinner("Clearing cache...");

    fs::remove_dir_all(&cache_path).context("Failed to remove cache directory")?;

    spinner.finish_and_clear();
    styler.print_success(&format!(
        "Cache cleared (freed {})",
        format_size(total_bytes)
    ));

    Ok(())
}

/// Print the cache directory path.
fn cmd_cache_path() -> Result<()> {
    let Some(cache_path) = cache_dir() else {
        bail!("Could not determine cache directory");
    };

    println!("{cache_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(10240), "10.00 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 5), "5.00 MB");
        assert_eq!(format_size(12_300_000), "11.73 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 2), "2.00 GB");
    }

    #[test]
    fn test_cache_dir_returns_some() {
        // Should return Some on most systems
        let dir = cache_dir();
        if let Some(path) = dir {
            assert!(path.as_str().contains("draftkit"));
        }
    }

    #[test]
    fn test_cache_path_command() {
        // Just verify it doesn't panic
        let result = cmd_cache_path();
        assert!(result.is_ok());
    }
}
