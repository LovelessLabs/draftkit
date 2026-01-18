//! Composite preview generation.
//!
//! Downloads component preview images and stitches them together vertically
//! to create a visual representation of a page composition.

use std::io::Cursor;

use base64::Engine;
use image::codecs::png::PngEncoder;
use image::{DynamicImage, ImageEncoder, ImageReader};
use reqwest::Client;

use super::cache;
use super::error::PreviewError;
use crate::components::Mode;

/// A source for a preview image.
#[derive(Debug, Clone)]
pub struct PreviewSource {
    /// Component ID for caching.
    pub component_id: String,
    /// URL to fetch the preview image from.
    pub preview_url: String,
    /// Component name for error messages.
    pub name: String,
}

/// Result of generating a preview.
#[derive(Debug)]
pub struct PreviewImage {
    /// PNG image data.
    pub data: Vec<u8>,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
}

impl PreviewImage {
    /// Convert to a data URL suitable for MCP Content::image().
    #[must_use]
    pub fn to_data_url(&self) -> String {
        let base64 = base64::engine::general_purpose::STANDARD.encode(&self.data);
        format!("data:image/png;base64,{base64}")
    }
}

/// Generator for composite preview images.
///
/// Downloads component preview images and stitches them vertically.
pub struct CompositePreview {
    client: Client,
}

impl Default for CompositePreview {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositePreview {
    /// Create a new composite preview generator.
    #[must_use]
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("draftkit-preview/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Generate a composite preview from multiple component previews.
    ///
    /// Downloads all preview images in parallel, then stitches them vertically.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No sources are provided
    /// - Any image fails to download
    /// - Image processing fails
    pub async fn generate(
        &self,
        sources: &[PreviewSource],
        mode: Mode,
    ) -> Result<PreviewImage, PreviewError> {
        if sources.is_empty() {
            return Err(PreviewError::NoComponents);
        }

        // Download all images in parallel
        let images = self.download_all(sources, mode).await?;

        // Stitch images vertically
        self.stitch_vertical(&images)
    }

    /// Download all preview images, using cache when available.
    async fn download_all(
        &self,
        sources: &[PreviewSource],
        mode: Mode,
    ) -> Result<Vec<DynamicImage>, PreviewError> {
        use futures::future::join_all;

        let futures: Vec<_> = sources
            .iter()
            .map(|source| self.download_one(source, mode))
            .collect();

        let results = join_all(futures).await;

        // Collect all results, propagating the first error
        let mut images = Vec::with_capacity(sources.len());
        for result in results {
            images.push(result?);
        }

        Ok(images)
    }

    /// Download a single preview image, using cache when available.
    async fn download_one(
        &self,
        source: &PreviewSource,
        mode: Mode,
    ) -> Result<DynamicImage, PreviewError> {
        // Check cache first
        if let Some(cached_bytes) = cache::get_cached(&source.component_id, mode) {
            return self.decode_image(&cached_bytes);
        }

        // Download from URL
        let bytes = self.fetch_image(&source.preview_url).await?;

        // Cache the downloaded image
        if let Err(e) = cache::store_cached(&source.component_id, mode, &bytes) {
            // Log but don't fail on cache errors
            eprintln!(
                "Warning: Failed to cache preview for {}: {}",
                source.component_id, e
            );
        }

        self.decode_image(&bytes)
    }

    /// Fetch image bytes from a URL.
    async fn fetch_image(&self, url: &str) -> Result<Vec<u8>, PreviewError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(PreviewError::Http)?;

        if !response.status().is_success() {
            return Err(PreviewError::Download(format!(
                "HTTP {}: {}",
                response.status(),
                url
            )));
        }

        let bytes = response.bytes().await.map_err(PreviewError::Http)?;
        Ok(bytes.to_vec())
    }

    /// Decode image bytes into a `DynamicImage`.
    fn decode_image(&self, bytes: &[u8]) -> Result<DynamicImage, PreviewError> {
        let cursor = Cursor::new(bytes);
        let reader = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| PreviewError::Image(format!("Failed to detect format: {e}")))?;

        reader
            .decode()
            .map_err(|e| PreviewError::Image(format!("Failed to decode image: {e}")))
    }

    /// Stitch images vertically into a single composite image.
    fn stitch_vertical(&self, images: &[DynamicImage]) -> Result<PreviewImage, PreviewError> {
        if images.is_empty() {
            return Err(PreviewError::NoComponents);
        }

        // Calculate canvas dimensions
        let max_width = images.iter().map(DynamicImage::width).max().unwrap_or(0);
        let total_height: u32 = images.iter().map(DynamicImage::height).sum();

        if max_width == 0 || total_height == 0 {
            return Err(PreviewError::Image("Invalid image dimensions".to_string()));
        }

        // Create canvas with white background
        let mut canvas = image::RgbaImage::from_pixel(
            max_width,
            total_height,
            image::Rgba([255, 255, 255, 255]),
        );

        // Overlay each image, centered horizontally
        let mut y_offset = 0u32;
        for img in images {
            let rgba = img.to_rgba8();
            let x_offset = (max_width - img.width()) / 2;

            image::imageops::overlay(&mut canvas, &rgba, i64::from(x_offset), i64::from(y_offset));
            y_offset += img.height();
        }

        // Encode to PNG
        let mut png_bytes = Vec::new();
        let encoder = PngEncoder::new(&mut png_bytes);
        encoder
            .write_image(
                &canvas,
                max_width,
                total_height,
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e| PreviewError::Image(format!("Failed to encode PNG: {e}")))?;

        Ok(PreviewImage {
            data: png_bytes,
            width: max_width,
            height: total_height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_image_to_data_url() {
        let preview = PreviewImage {
            data: vec![0x89, 0x50, 0x4E, 0x47], // PNG magic bytes
            width: 100,
            height: 100,
        };

        let data_url = preview.to_data_url();
        assert!(data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_composite_preview_default() {
        let preview = CompositePreview::default();
        // Just verify it doesn't panic
        drop(preview);
    }

    #[tokio::test]
    async fn test_generate_fails_with_no_sources() {
        let preview = CompositePreview::new();
        let result = preview.generate(&[], Mode::Light).await;
        assert!(matches!(result, Err(PreviewError::NoComponents)));
    }
}
