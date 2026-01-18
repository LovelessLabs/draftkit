//! StyleProfile extraction from Tailwind CSS classes.
//!
//! This module provides the core intelligence for extracting design DNA from
//! component code. By analyzing Tailwind utility classes, we compute
//! `StyleProfile` metrics that enable coherence checking and smart matching.
//!
//! # Extraction Algorithm
//!
//! 1. **Visual Weight**: Count decorative classes (shadows, gradients, rings, borders)
//! 2. **Formality**: Analyze color palette (grays = formal, bright colors = casual)
//! 3. **Color Intensity**: Count unique color tokens and their saturation levels
//! 4. **Spacing Density**: Average padding/margin/gap values (higher = more spacious)
//! 5. **Typography Scale**: Largest text class determines the scale
//!
//! # Example
//!
//! ```rust
//! use draftkit_core::intelligence::StyleExtractor;
//!
//! let code = r#"<div class="bg-white shadow-lg rounded-xl p-8 text-gray-900">"#;
//! let profile = StyleExtractor::extract(code);
//!
//! assert!(profile.visual_weight > 0.3); // shadow-lg, rounded-xl add weight
//! ```

use crate::components::{StyleProfile, TypographyScale};
use std::collections::HashSet;

/// Extracts `StyleProfile` from component code by analyzing Tailwind classes.
#[derive(Debug, Clone, Default)]
pub struct StyleExtractor;

impl StyleExtractor {
    /// Create a new style extractor.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Extract a `StyleProfile` from component code.
    ///
    /// Parses all className/class attributes and computes visual metrics.
    #[must_use]
    pub fn extract(code: &str) -> StyleProfile {
        let classes = Self::extract_classes(code);
        Self::compute_profile(&classes)
    }

    /// Extract all Tailwind classes from component code.
    ///
    /// Handles both `class="..."` and `className="..."` patterns,
    /// including template literals and conditional classes.
    fn extract_classes(code: &str) -> Vec<String> {
        let mut classes = Vec::new();

        // Pattern 1: class="..." or className="..."
        for cap in regex_lite::Regex::new(r#"(?:class|className)\s*=\s*["']([^"']+)["']"#)
            .unwrap()
            .captures_iter(code)
        {
            if let Some(m) = cap.get(1) {
                classes.extend(m.as_str().split_whitespace().map(String::from));
            }
        }

        // Pattern 2: clsx/cn helper: clsx("class1", "class2") or cn("class1", "class2")
        for cap in regex_lite::Regex::new(r#"(?:clsx|cn)\s*\(\s*["']([^"']+)["']"#)
            .unwrap()
            .captures_iter(code)
        {
            if let Some(m) = cap.get(1) {
                classes.extend(m.as_str().split_whitespace().map(String::from));
            }
        }

        // Pattern 3: Template literals with embedded classes
        for cap in regex_lite::Regex::new(r#"`([^`]*)`"#)
            .unwrap()
            .captures_iter(code)
        {
            if let Some(m) = cap.get(1) {
                // Extract likely Tailwind classes (alphanumeric with dashes/colons)
                for word in m.as_str().split_whitespace() {
                    if Self::looks_like_tailwind_class(word) {
                        classes.push(word.to_string());
                    }
                }
            }
        }

        classes
    }

    /// Check if a string looks like a Tailwind class.
    fn looks_like_tailwind_class(s: &str) -> bool {
        // Remove responsive/state prefixes for checking
        let base = s.split(':').next_back().unwrap_or(s);

        // Common Tailwind patterns
        base.starts_with("bg-")
            || base.starts_with("text-")
            || base.starts_with("p-")
            || base.starts_with("px-")
            || base.starts_with("py-")
            || base.starts_with("pt-")
            || base.starts_with("pb-")
            || base.starts_with("pl-")
            || base.starts_with("pr-")
            || base.starts_with("m-")
            || base.starts_with("mx-")
            || base.starts_with("my-")
            || base.starts_with("mt-")
            || base.starts_with("mb-")
            || base.starts_with("ml-")
            || base.starts_with("mr-")
            || base.starts_with("gap-")
            || base.starts_with("space-")
            || base.starts_with("w-")
            || base.starts_with("h-")
            || base.starts_with("min-")
            || base.starts_with("max-")
            || base.starts_with("flex")
            || base.starts_with("grid")
            || base.starts_with("rounded")
            || base.starts_with("shadow")
            || base.starts_with("border")
            || base.starts_with("ring")
            || base.starts_with("font-")
            || base.starts_with("leading-")
            || base.starts_with("tracking-")
            || base.starts_with("opacity-")
            || base.starts_with("z-")
            || base.starts_with("inset-")
            || base.starts_with("top-")
            || base.starts_with("right-")
            || base.starts_with("bottom-")
            || base.starts_with("left-")
            || base.starts_with("gradient")
            || base.starts_with("from-")
            || base.starts_with("to-")
            || base.starts_with("via-")
            || base == "hidden"
            || base == "block"
            || base == "inline"
            || base == "relative"
            || base == "absolute"
            || base == "fixed"
            || base == "sticky"
    }

    /// Compute `StyleProfile` from extracted classes.
    fn compute_profile(classes: &[String]) -> StyleProfile {
        StyleProfile {
            visual_weight: Self::compute_visual_weight(classes),
            formality: Self::compute_formality(classes),
            color_intensity: Self::compute_color_intensity(classes),
            spacing_density: Self::compute_spacing_density(classes),
            typography_scale: Self::compute_typography_scale(classes),
        }
    }

    /// Compute visual weight from decorative classes.
    ///
    /// High visual weight = shadows, gradients, rings, heavy borders.
    fn compute_visual_weight(classes: &[String]) -> f32 {
        let mut score: f32 = 0.0;

        for class in classes {
            let base = class.split(':').next_back().unwrap_or(class);

            // Shadows add weight
            if base.starts_with("shadow") {
                score += match base {
                    "shadow-sm" => 0.5,
                    "shadow" => 1.0,
                    "shadow-md" => 1.5,
                    "shadow-lg" => 2.0,
                    "shadow-xl" => 2.5,
                    "shadow-2xl" => 3.0,
                    _ => 0.5,
                };
            }

            // Gradients add significant weight
            if base.starts_with("bg-gradient") || base.starts_with("from-") {
                score += 2.0;
            }

            // Rings add weight
            if base.starts_with("ring") && base != "ring-0" {
                score += match base {
                    "ring-1" => 0.5,
                    "ring-2" => 1.0,
                    "ring" => 1.0,
                    "ring-4" => 1.5,
                    "ring-8" => 2.0,
                    _ => 0.5,
                };
            }

            // Borders add weight
            if base.starts_with("border") && !base.contains("transparent") {
                if base == "border" || base.starts_with("border-x") || base.starts_with("border-y") {
                    score += 0.5;
                } else if base.starts_with("border-2") || base.starts_with("border-4") {
                    score += 1.0;
                }
            }

            // Rounded corners add subtle weight
            if base.starts_with("rounded") {
                score += match base {
                    "rounded-sm" => 0.1,
                    "rounded" => 0.2,
                    "rounded-md" => 0.3,
                    "rounded-lg" => 0.4,
                    "rounded-xl" => 0.5,
                    "rounded-2xl" => 0.6,
                    "rounded-3xl" => 0.7,
                    "rounded-full" => 0.8,
                    _ => 0.2,
                };
            }

            // Backdrop blur adds weight
            if base.starts_with("backdrop-blur") {
                score += 1.0;
            }
        }

        // Normalize to 0-1 (cap at 15 points)
        (score / 15.0).min(1.0)
    }

    /// Compute formality from color palette analysis.
    ///
    /// Grays, slates = formal. Bright colors = casual.
    fn compute_formality(classes: &[String]) -> f32 {
        let mut formal_count = 0;
        let mut casual_count = 0;

        for class in classes {
            let base = class.split(':').next_back().unwrap_or(class);

            // Extract color from class like "bg-gray-900" or "text-indigo-600"
            let parts: Vec<&str> = base.split('-').collect();
            if parts.len() >= 2 {
                let color = parts[1];

                match color {
                    // Formal colors
                    "gray" | "slate" | "zinc" | "neutral" | "stone" | "black" | "white" => {
                        formal_count += 1;
                    }
                    // Casual/vibrant colors
                    "red" | "orange" | "amber" | "yellow" | "lime" | "green" | "emerald"
                    | "teal" | "cyan" | "sky" | "blue" | "indigo" | "violet" | "purple"
                    | "fuchsia" | "pink" | "rose" => {
                        casual_count += 1;
                    }
                    _ => {}
                }
            }
        }

        let total = formal_count + casual_count;
        if total == 0 {
            0.5 // Neutral default
        } else {
            // Higher ratio of formal colors = higher formality
            formal_count as f32 / total as f32
        }
    }

    /// Compute color intensity from unique color token count.
    fn compute_color_intensity(classes: &[String]) -> f32 {
        let mut unique_colors: HashSet<String> = HashSet::new();
        let mut high_saturation_count = 0;

        for class in classes {
            let base = class.split(':').next_back().unwrap_or(class);

            // Extract color from class like "bg-indigo-600"
            let parts: Vec<&str> = base.split('-').collect();
            if parts.len() >= 3 {
                // Has color and shade (e.g., "bg", "indigo", "600")
                let color = parts[1];
                let shade = parts[2];

                // Skip neutral colors for intensity calculation
                if !matches!(
                    color,
                    "gray" | "slate" | "zinc" | "neutral" | "stone" | "black" | "white"
                ) {
                    unique_colors.insert(color.to_string());

                    // High saturation shades (400-600 typically most vibrant)
                    if let Ok(shade_num) = shade.parse::<u32>() {
                        if (400..=600).contains(&shade_num) {
                            high_saturation_count += 1;
                        }
                    }
                }
            }
        }

        // Combine unique color count and saturation
        let color_score = (unique_colors.len() as f32 / 5.0).min(1.0);
        let saturation_score = (high_saturation_count as f32 / 10.0).min(1.0);

        // Weighted average
        (color_score * 0.6 + saturation_score * 0.4).min(1.0)
    }

    /// Compute spacing density from padding/margin/gap values.
    ///
    /// Higher average spacing = more spacious (closer to 1.0).
    fn compute_spacing_density(classes: &[String]) -> f32 {
        let mut spacing_values: Vec<f32> = Vec::new();

        for class in classes {
            let base = class.split(':').next_back().unwrap_or(class);

            // Extract spacing value from classes like "p-4", "mx-8", "gap-6"
            if let Some(value) = Self::extract_spacing_value(base) {
                spacing_values.push(value);
            }
        }

        if spacing_values.is_empty() {
            0.5 // Neutral default
        } else {
            let avg = spacing_values.iter().sum::<f32>() / spacing_values.len() as f32;
            // Normalize: 0 = very tight, 24+ = very spacious
            (avg / 16.0).min(1.0)
        }
    }

    /// Extract numeric spacing value from a Tailwind spacing class.
    fn extract_spacing_value(class: &str) -> Option<f32> {
        let prefixes = [
            "p-", "px-", "py-", "pt-", "pb-", "pl-", "pr-", "m-", "mx-", "my-", "mt-", "mb-", "ml-",
            "mr-", "gap-", "gap-x-", "gap-y-", "space-x-", "space-y-",
        ];

        for prefix in prefixes {
            if let Some(rest) = class.strip_prefix(prefix) {
                // Handle numeric values
                if let Ok(num) = rest.parse::<f32>() {
                    return Some(num);
                }
                // Handle fractional values like "1/2"
                if rest.contains('/') {
                    let parts: Vec<&str> = rest.split('/').collect();
                    if parts.len() == 2 {
                        if let (Ok(n), Ok(d)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                            return Some(n / d * 4.0); // Approximate rem value
                        }
                    }
                }
                // Handle special values
                return match rest {
                    "px" => Some(0.25),
                    "0.5" => Some(0.125),
                    "1.5" => Some(0.375),
                    "2.5" => Some(0.625),
                    "3.5" => Some(0.875),
                    _ => None,
                };
            }
        }

        None
    }

    /// Compute typography scale from text size classes.
    fn compute_typography_scale(classes: &[String]) -> TypographyScale {
        let mut largest_size = 0;

        for class in classes {
            let base = class.split(':').next_back().unwrap_or(class);

            if base.starts_with("text-") {
                let size = match base {
                    "text-xs" => 1,
                    "text-sm" => 2,
                    "text-base" => 3,
                    "text-lg" => 4,
                    "text-xl" => 5,
                    "text-2xl" => 6,
                    "text-3xl" => 7,
                    "text-4xl" => 8,
                    "text-5xl" => 9,
                    "text-6xl" => 10,
                    "text-7xl" => 11,
                    "text-8xl" => 12,
                    "text-9xl" => 13,
                    _ => 0,
                };
                largest_size = largest_size.max(size);
            }
        }

        match largest_size {
            0..=3 => TypographyScale::Small,
            4..=6 => TypographyScale::Medium,
            _ => TypographyScale::Large,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic_classes() {
        let code = r#"<div class="bg-white shadow-lg p-4">"#;
        let profile = StyleExtractor::extract(code);

        assert!(profile.visual_weight > 0.0);
    }

    #[test]
    fn extract_jsx_classname() {
        let code = r#"<div className="bg-gray-900 text-white rounded-xl">"#;
        let profile = StyleExtractor::extract(code);

        assert!(profile.visual_weight > 0.0);
        assert!(profile.formality > 0.5); // gray is formal
    }

    #[test]
    fn visual_weight_shadows() {
        let light = r#"<div class="shadow-sm">"#;
        let heavy = r#"<div class="shadow-2xl">"#;

        let light_profile = StyleExtractor::extract(light);
        let heavy_profile = StyleExtractor::extract(heavy);

        assert!(heavy_profile.visual_weight > light_profile.visual_weight);
    }

    #[test]
    fn visual_weight_gradients() {
        let plain = r#"<div class="bg-white">"#;
        let gradient = r#"<div class="bg-gradient-to-r from-indigo-500 to-purple-500">"#;

        let plain_profile = StyleExtractor::extract(plain);
        let gradient_profile = StyleExtractor::extract(gradient);

        assert!(gradient_profile.visual_weight > plain_profile.visual_weight);
    }

    #[test]
    fn formality_gray_colors() {
        let formal = r#"<div class="bg-gray-900 text-gray-100 border-gray-200">"#;
        let casual = r#"<div class="bg-indigo-600 text-pink-500 border-purple-400">"#;

        let formal_profile = StyleExtractor::extract(formal);
        let casual_profile = StyleExtractor::extract(casual);

        assert!(formal_profile.formality > casual_profile.formality);
    }

    #[test]
    fn color_intensity_calculation() {
        let muted = r#"<div class="bg-gray-100 text-gray-900">"#;
        let vibrant = r#"<div class="bg-indigo-500 text-pink-500 border-purple-500">"#;

        let muted_profile = StyleExtractor::extract(muted);
        let vibrant_profile = StyleExtractor::extract(vibrant);

        assert!(vibrant_profile.color_intensity > muted_profile.color_intensity);
    }

    #[test]
    fn spacing_density_tight_vs_spacious() {
        let tight = r#"<div class="p-1 m-1 gap-1">"#;
        let spacious = r#"<div class="p-12 m-8 gap-8">"#;

        let tight_profile = StyleExtractor::extract(tight);
        let spacious_profile = StyleExtractor::extract(spacious);

        assert!(spacious_profile.spacing_density > tight_profile.spacing_density);
    }

    #[test]
    fn typography_scale_detection() {
        let small = r#"<div class="text-sm text-xs">"#;
        let large = r#"<div class="text-4xl text-lg">"#;

        let small_profile = StyleExtractor::extract(small);
        let large_profile = StyleExtractor::extract(large);

        assert_eq!(small_profile.typography_scale, TypographyScale::Small);
        assert_eq!(large_profile.typography_scale, TypographyScale::Large);
    }

    #[test]
    fn empty_code_returns_defaults() {
        let profile = StyleExtractor::extract("");

        assert_eq!(profile.visual_weight, 0.0);
        assert_eq!(profile.formality, 0.5);
        assert_eq!(profile.color_intensity, 0.0);
        assert_eq!(profile.spacing_density, 0.5);
        assert_eq!(profile.typography_scale, TypographyScale::Small);
    }

    #[test]
    fn responsive_prefixes_handled() {
        // Should extract "shadow-lg" from "md:shadow-lg"
        let code = r#"<div class="md:shadow-lg lg:p-8">"#;
        let profile = StyleExtractor::extract(code);

        assert!(profile.visual_weight > 0.0);
    }

    #[test]
    fn state_prefixes_handled() {
        // Should extract "shadow-lg" from "hover:shadow-lg"
        let code = r#"<div class="hover:shadow-lg focus:ring-2">"#;
        let profile = StyleExtractor::extract(code);

        assert!(profile.visual_weight > 0.0);
    }

    #[test]
    fn real_hero_component() {
        // Realistic hero section code
        let code = r#"
            <section className="bg-white">
                <div className="mx-auto max-w-7xl px-6 py-24 sm:py-32 lg:px-8">
                    <h1 className="text-5xl font-semibold tracking-tight text-gray-900">
                        Build faster
                    </h1>
                    <p className="mt-6 text-xl text-gray-600">
                        Ship your product in days, not weeks.
                    </p>
                    <button className="mt-8 rounded-lg bg-indigo-600 px-4 py-2 text-white shadow-md">
                        Get started
                    </button>
                </div>
            </section>
        "#;

        let profile = StyleExtractor::extract(code);

        // Hero should have:
        // - Some visual weight (shadow, rounded)
        // - Mixed formality (gray text + indigo button)
        // - Large typography
        assert!(profile.visual_weight > 0.0);
        assert_eq!(profile.typography_scale, TypographyScale::Large);
    }
}
