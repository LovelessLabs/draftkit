//! Preview image generation for page compositions.
//!
//! This module provides two tiers of preview generation:
//!
//! 1. **Composite preview** (fast): Downloads component preview images and
//!    stitches them vertically for quick iteration feedback.
//!
//! 2. **Rendered preview** (accurate): Scaffolds a full page with components,
//!    starts a dev server, and uses Playwright for pixel-accurate screenshots.
//!
//! # Example
//!
//! ```ignore
//! use draftkit_core::preview::{CompositePreview, PreviewSource};
//! use draftkit_core::Mode;
//!
//! let preview = CompositePreview::new();
//! let sources = vec![
//!     PreviewSource {
//!         component_id: "hero-1".to_string(),
//!         preview_url: "https://example.com/hero-1.png".to_string(),
//!         name: "Hero Section".to_string(),
//!     },
//!     PreviewSource {
//!         component_id: "pricing-2".to_string(),
//!         preview_url: "https://example.com/pricing-2.png".to_string(),
//!         name: "Pricing Section".to_string(),
//!     },
//! ];
//!
//! let image = preview.generate(&sources, Mode::Light).await?;
//! let data_url = image.to_data_url();
//! ```

mod cache;
mod composite;
mod error;
mod rendered;

pub use cache::{
    PreviewCacheStats, clear_cache as clear_preview_cache, get_stats as get_preview_cache_stats,
    previews_cache_dir,
};
pub use composite::{CompositePreview, PreviewImage, PreviewSource};
pub use error::PreviewError;
pub use rendered::RenderedPreview;

/// Preview generation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewMode {
    /// Fast composite preview by stitching component preview images.
    #[default]
    Composite,
    /// Accurate rendered preview using Playwright to screenshot a running page.
    Rendered,
}

impl PreviewMode {
    /// Parse a mode string.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "composite" | "fast" => Some(Self::Composite),
            "rendered" | "accurate" => Some(Self::Rendered),
            _ => None,
        }
    }

    /// Get the mode as a string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Composite => "composite",
            Self::Rendered => "rendered",
        }
    }
}

impl std::fmt::Display for PreviewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_mode_parse() {
        assert_eq!(
            PreviewMode::parse("composite"),
            Some(PreviewMode::Composite)
        );
        assert_eq!(PreviewMode::parse("fast"), Some(PreviewMode::Composite));
        assert_eq!(PreviewMode::parse("rendered"), Some(PreviewMode::Rendered));
        assert_eq!(PreviewMode::parse("accurate"), Some(PreviewMode::Rendered));
        assert_eq!(PreviewMode::parse("invalid"), None);
    }

    #[test]
    fn test_preview_mode_as_str() {
        assert_eq!(PreviewMode::Composite.as_str(), "composite");
        assert_eq!(PreviewMode::Rendered.as_str(), "rendered");
    }

    #[test]
    fn test_preview_mode_display() {
        assert_eq!(format!("{}", PreviewMode::Composite), "composite");
        assert_eq!(format!("{}", PreviewMode::Rendered), "rendered");
    }

    #[test]
    fn test_preview_mode_default() {
        assert_eq!(PreviewMode::default(), PreviewMode::Composite);
    }
}
