//! Preset schema definitions.
//!
//! Presets are aesthetic overlays that modify how patterns select
//! components. They define style constraints, variant preferences, and
//! blacklists without changing the underlying page structure.

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root structure of a preset TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PresetFile {
    /// The preset definition
    pub preset: Preset,
}

/// A preset that modifies pattern behavior.
///
/// Presets act as aesthetic overlays:
/// - Style overrides constrain component selection by design DNA
/// - Variant preferences specify which component variants to prefer
/// - Blacklists exclude specific components entirely
/// - Inheritance allows presets to extend others
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Preset {
    /// Preset name (e.g., "Minimalist")
    pub name: String,

    /// Preset version (semver)
    #[serde(default = "default_version")]
    pub version: String,

    /// Preset author
    #[serde(default)]
    pub author: String,

    /// Human-readable description
    #[serde(default)]
    pub description: String,

    /// Parent preset to inherit from (e.g., "@bauhaus/minimalist")
    #[serde(default)]
    pub extends: Option<String>,

    /// Searchable tags (e.g., ["minimal", "clean", "corporate"])
    #[serde(default)]
    pub tags: Vec<String>,

    /// Style constraint overrides
    #[serde(default)]
    pub style_overrides: StyleOverrides,

    /// Preferred variants by section type
    #[serde(default)]
    pub variant_preferences: HashMap<String, String>,

    /// Components to exclude
    #[serde(default)]
    pub blacklist: Blacklist,

    /// Components to always prefer (opposite of blacklist)
    #[serde(default)]
    pub whitelist: Whitelist,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: default_version(),
            author: String::new(),
            description: String::new(),
            extends: None,
            tags: Vec::new(),
            style_overrides: StyleOverrides::default(),
            variant_preferences: HashMap::new(),
            blacklist: Blacklist::default(),
            whitelist: Whitelist::default(),
        }
    }
}

/// Style constraint overrides.
///
/// Each field is optional - only specified fields override the base pattern.
/// Values are normalized 0.0-1.0 where applicable.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StyleOverrides {
    // Visual weight constraints
    /// Maximum visual weight (0.0 = minimal, 1.0 = maximal)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visual_weight_max: Option<f32>,

    /// Minimum visual weight
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visual_weight_min: Option<f32>,

    // Formality constraints
    /// Maximum formality (0.0 = playful, 1.0 = corporate)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formality_max: Option<f32>,

    /// Minimum formality
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formality_min: Option<f32>,

    // Color intensity constraints
    /// Maximum color intensity (0.0 = monochrome, 1.0 = vibrant)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_intensity_max: Option<f32>,

    /// Minimum color intensity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_intensity_min: Option<f32>,

    // Spacing density constraints
    /// Maximum spacing density (0.0 = tight, 1.0 = spacious)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing_density_max: Option<f32>,

    /// Minimum spacing density
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing_density_min: Option<f32>,

    // Typography constraints
    /// Allowed typography scales
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typography_scales: Option<Vec<String>>,
}

impl StyleOverrides {
    /// Check if a visual weight value passes the constraints.
    #[must_use]
    pub fn check_visual_weight(&self, value: f32) -> bool {
        if let Some(max) = self.visual_weight_max
            && value > max
        {
            return false;
        }
        if let Some(min) = self.visual_weight_min
            && value < min
        {
            return false;
        }
        true
    }

    /// Check if a formality value passes the constraints.
    #[must_use]
    pub fn check_formality(&self, value: f32) -> bool {
        if let Some(max) = self.formality_max
            && value > max
        {
            return false;
        }
        if let Some(min) = self.formality_min
            && value < min
        {
            return false;
        }
        true
    }

    /// Check if a color intensity value passes the constraints.
    #[must_use]
    pub fn check_color_intensity(&self, value: f32) -> bool {
        if let Some(max) = self.color_intensity_max
            && value > max
        {
            return false;
        }
        if let Some(min) = self.color_intensity_min
            && value < min
        {
            return false;
        }
        true
    }

    /// Check if a spacing density value passes the constraints.
    #[must_use]
    pub fn check_spacing_density(&self, value: f32) -> bool {
        if let Some(max) = self.spacing_density_max
            && value > max
        {
            return false;
        }
        if let Some(min) = self.spacing_density_min
            && value < min
        {
            return false;
        }
        true
    }

    /// Check if any overrides are set.
    #[must_use]
    pub const fn has_overrides(&self) -> bool {
        self.visual_weight_max.is_some()
            || self.visual_weight_min.is_some()
            || self.formality_max.is_some()
            || self.formality_min.is_some()
            || self.color_intensity_max.is_some()
            || self.color_intensity_min.is_some()
            || self.spacing_density_max.is_some()
            || self.spacing_density_min.is_some()
            || self.typography_scales.is_some()
    }
}

/// Component blacklist.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Blacklist {
    /// Component IDs to exclude (e.g., ["hero-video-background"])
    #[serde(default)]
    pub components: Vec<String>,

    /// Component tags to exclude (e.g., ["animated", "video"])
    #[serde(default)]
    pub tags: Vec<String>,

    /// Categories to exclude (e.g., ["marketing/heroes/video"])
    #[serde(default)]
    pub categories: Vec<String>,
}

impl Blacklist {
    /// Check if a component ID is blacklisted.
    #[must_use]
    pub fn is_component_blocked(&self, component_id: &str) -> bool {
        self.components.iter().any(|c| c == component_id)
    }

    /// Check if any of the given tags are blacklisted.
    #[must_use]
    pub fn has_blocked_tag(&self, tags: &[String]) -> bool {
        tags.iter().any(|t| self.tags.contains(t))
    }

    /// Check if a category path is blacklisted.
    #[must_use]
    pub fn is_category_blocked(&self, category: &str) -> bool {
        self.categories.iter().any(|c| category.starts_with(c))
    }

    /// Check if the blacklist has any entries.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.components.is_empty() && self.tags.is_empty() && self.categories.is_empty()
    }
}

/// Component whitelist (preference list).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Whitelist {
    /// Component IDs to prefer
    #[serde(default)]
    pub components: Vec<String>,

    /// Component tags to prefer
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Whitelist {
    /// Check if a component ID is whitelisted.
    #[must_use]
    pub fn is_component_preferred(&self, component_id: &str) -> bool {
        self.components.iter().any(|c| c == component_id)
    }

    /// Check if any of the given tags are whitelisted.
    #[must_use]
    pub fn has_preferred_tag(&self, tags: &[String]) -> bool {
        tags.iter().any(|t| self.tags.contains(t))
    }

    /// Check if the whitelist has any entries.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.components.is_empty() && self.tags.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_preset() {
        let toml = r#"
[preset]
name = "Test Preset"
"#;
        let parsed: PresetFile = toml::from_str(toml).unwrap();
        assert_eq!(parsed.preset.name, "Test Preset");
        assert_eq!(parsed.preset.version, "1.0.0");
    }

    #[test]
    fn parse_preset_with_inheritance() {
        let toml = r#"
[preset]
name = "Corporate Dark"
extends = "minimalist"

[preset.style_overrides]
formality_min = 0.8
"#;
        let parsed: PresetFile = toml::from_str(toml).unwrap();
        assert_eq!(parsed.preset.name, "Corporate Dark");
        assert_eq!(parsed.preset.extends, Some("minimalist".to_string()));
        assert_eq!(parsed.preset.style_overrides.formality_min, Some(0.8));
    }

    #[test]
    fn parse_full_preset() {
        let toml = r#"
[preset]
name = "Minimalist"
version = "1.0.0"
author = "draftkit"
description = "Clean, minimal design with lots of whitespace"
tags = ["minimal", "clean", "corporate"]

[preset.style_overrides]
visual_weight_max = 0.3
spacing_density_min = 0.7
color_intensity_max = 0.2

[preset.variant_preferences]
hero = "hero-centered-cta"
features = "feature-three-column-cards"

[preset.blacklist]
components = ["hero-video-background", "cta-branded-background"]
tags = ["animated", "video"]

[preset.whitelist]
components = ["hero-simple", "footer-minimal"]
"#;
        let parsed: PresetFile = toml::from_str(toml).unwrap();
        let preset = &parsed.preset;

        assert_eq!(preset.name, "Minimalist");
        assert_eq!(preset.style_overrides.visual_weight_max, Some(0.3));
        assert_eq!(preset.style_overrides.spacing_density_min, Some(0.7));
        assert_eq!(
            preset.variant_preferences.get("hero"),
            Some(&"hero-centered-cta".to_string())
        );
        assert!(
            preset
                .blacklist
                .components
                .contains(&"hero-video-background".to_string())
        );
        assert!(
            preset
                .whitelist
                .components
                .contains(&"hero-simple".to_string())
        );
    }

    #[test]
    fn style_overrides_check_visual_weight() {
        let overrides = StyleOverrides {
            visual_weight_max: Some(0.5),
            visual_weight_min: Some(0.1),
            ..Default::default()
        };

        assert!(overrides.check_visual_weight(0.3));
        assert!(!overrides.check_visual_weight(0.6));
        assert!(!overrides.check_visual_weight(0.05));
    }

    #[test]
    fn blacklist_component_check() {
        let blacklist = Blacklist {
            components: vec!["hero-video".to_string(), "cta-animated".to_string()],
            tags: vec!["video".to_string()],
            categories: vec!["marketing/heroes/video".to_string()],
        };

        assert!(blacklist.is_component_blocked("hero-video"));
        assert!(!blacklist.is_component_blocked("hero-simple"));
        assert!(blacklist.has_blocked_tag(&["video".to_string(), "clean".to_string()]));
        assert!(blacklist.is_category_blocked("marketing/heroes/video/background"));
    }

    #[test]
    fn whitelist_component_check() {
        let whitelist = Whitelist {
            components: vec!["hero-simple".to_string()],
            tags: vec!["minimal".to_string()],
        };

        assert!(whitelist.is_component_preferred("hero-simple"));
        assert!(!whitelist.is_component_preferred("hero-video"));
        assert!(whitelist.has_preferred_tag(&["minimal".to_string()]));
    }
}
