//! Preset loading with directory precedence and stacking.
//!
//! Presets are loaded from multiple directories with later sources
//! overriding earlier ones:
//!
//! 1. Built-in presets (embedded in binary)
//! 2. User presets (~/.config/draftkit/presets/)
//! 3. Project presets (./.draftkit/presets/)
//!
//! Multiple presets can be active simultaneously and stack on top of each other.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::preset::schema::{Preset, PresetFile, StyleOverrides};

/// Error type for preset operations.
#[derive(Debug, thiserror::Error)]
pub enum PresetError {
    #[error("Preset not found: {0}")]
    NotFound(String),

    #[error("Failed to read preset file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse preset TOML: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Preset validation failed: {0}")]
    ValidationError(String),

    #[error("Circular inheritance detected: {0}")]
    CircularInheritance(String),
}

/// Preset source for tracking where a preset came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetSource {
    /// Embedded in binary
    BuiltIn,
    /// User's config directory
    User,
    /// Project-local .draftkit directory
    Project,
}

/// A loaded preset with source tracking.
#[derive(Debug, Clone)]
pub struct LoadedPreset {
    /// The preset definition
    pub preset: Preset,
    /// Where this preset was loaded from
    pub source: PresetSource,
    /// File path (if loaded from disk)
    pub path: Option<PathBuf>,
}

/// Preset loader with directory precedence.
#[derive(Debug)]
pub struct PresetLoader {
    /// All available presets by name
    presets: HashMap<String, LoadedPreset>,
    /// Currently active preset names (in stack order, first = bottom)
    active_stack: Vec<String>,
}

impl PresetLoader {
    /// Create a new preset loader and load from all sources.
    ///
    /// # Errors
    /// Returns error if preset directories cannot be read.
    pub fn new() -> Result<Self, PresetError> {
        let mut loader = Self {
            presets: HashMap::new(),
            active_stack: Vec::new(),
        };

        // Load in precedence order (later overrides earlier)
        loader.load_builtin_presets();
        loader.load_user_presets()?;
        loader.load_project_presets()?;

        Ok(loader)
    }

    /// Create a loader with only built-in presets.
    #[must_use]
    pub fn builtin_only() -> Self {
        let mut loader = Self {
            presets: HashMap::new(),
            active_stack: Vec::new(),
        };
        loader.load_builtin_presets();
        loader
    }

    /// Create an empty loader (no presets).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            presets: HashMap::new(),
            active_stack: Vec::new(),
        }
    }

    /// Get a preset by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&LoadedPreset> {
        self.presets.get(name)
    }

    /// List all available preset names.
    #[must_use]
    pub fn list_names(&self) -> Vec<&str> {
        self.presets.keys().map(String::as_str).collect()
    }

    /// List all presets with metadata.
    #[must_use]
    pub fn list_all(&self) -> Vec<&LoadedPreset> {
        self.presets.values().collect()
    }

    /// Activate a preset by name (adds to stack).
    ///
    /// # Errors
    /// Returns error if preset not found.
    pub fn activate(&mut self, name: &str) -> Result<(), PresetError> {
        if !self.presets.contains_key(name) {
            return Err(PresetError::NotFound(name.to_string()));
        }

        // Remove if already in stack (will be re-added at top)
        self.active_stack.retain(|n| n != name);
        self.active_stack.push(name.to_string());

        Ok(())
    }

    /// Deactivate a preset by name (removes from stack).
    pub fn deactivate(&mut self, name: &str) {
        self.active_stack.retain(|n| n != name);
    }

    /// Clear all active presets.
    pub fn clear_active(&mut self) {
        self.active_stack.clear();
    }

    /// Set the entire stack at once (replaces current stack).
    ///
    /// # Errors
    /// Returns error if any preset in the list is not found.
    pub fn set_stack(&mut self, names: Vec<String>) -> Result<(), PresetError> {
        // Validate all names first
        for name in &names {
            if !self.presets.contains_key(name) {
                return Err(PresetError::NotFound(name.clone()));
            }
        }
        self.active_stack = names;
        Ok(())
    }

    /// Get the currently active preset names (in stack order, first = bottom).
    #[must_use]
    pub fn active_stack(&self) -> &[String] {
        &self.active_stack
    }

    /// Check if any presets are active.
    #[must_use]
    pub const fn has_active_presets(&self) -> bool {
        !self.active_stack.is_empty()
    }

    /// Get the merged style overrides from all active presets.
    ///
    /// Later presets in the stack override earlier ones.
    /// Inheritance is resolved for each preset in the stack.
    #[must_use]
    pub fn merged_style_overrides(&self) -> StyleOverrides {
        let mut merged = StyleOverrides::default();

        for name in &self.active_stack {
            if let Some(loaded) = self.presets.get(name) {
                // Resolve inheritance chain for this preset
                let resolved = self.resolve_style_overrides(&loaded.preset);
                merge_style_overrides(&mut merged, &resolved);
            }
        }

        merged
    }

    /// Resolve style overrides for a preset, following inheritance chain.
    fn resolve_style_overrides(&self, preset: &Preset) -> StyleOverrides {
        let mut resolved = if let Some(ref parent_name) = preset.extends
            && let Some(parent) = self.presets.get(parent_name)
        {
            self.resolve_style_overrides(&parent.preset)
        } else {
            StyleOverrides::default()
        };

        // Then apply our own overrides on top
        merge_style_overrides(&mut resolved, &preset.style_overrides);

        resolved
    }

    /// Get the merged variant preferences from all active presets.
    ///
    /// Returns section_type -> preferred_variant_id mapping.
    #[must_use]
    pub fn merged_variant_preferences(&self) -> HashMap<String, String> {
        let mut merged = HashMap::new();

        for name in &self.active_stack {
            if let Some(loaded) = self.presets.get(name) {
                // Resolve inheritance for variant preferences too
                let resolved = self.resolve_variant_preferences(&loaded.preset);
                for (section, variant) in resolved {
                    merged.insert(section, variant);
                }
            }
        }

        merged
    }

    /// Resolve variant preferences for a preset, following inheritance chain.
    fn resolve_variant_preferences(&self, preset: &Preset) -> HashMap<String, String> {
        let mut resolved = if let Some(ref parent_name) = preset.extends
            && let Some(parent) = self.presets.get(parent_name)
        {
            self.resolve_variant_preferences(&parent.preset)
        } else {
            HashMap::new()
        };

        // Then apply our own preferences on top
        for (section, variant) in &preset.variant_preferences {
            resolved.insert(section.clone(), variant.clone());
        }

        resolved
    }

    /// Check if a component is blacklisted by any active preset.
    #[must_use]
    pub fn is_component_blacklisted(&self, component_id: &str) -> bool {
        for name in &self.active_stack {
            if let Some(loaded) = self.presets.get(name)
                && loaded.preset.blacklist.is_component_blocked(component_id)
            {
                return true;
            }
        }
        false
    }

    /// Check if any tags are blacklisted by active presets.
    #[must_use]
    pub fn has_blacklisted_tag(&self, tags: &[String]) -> bool {
        for name in &self.active_stack {
            if let Some(loaded) = self.presets.get(name)
                && loaded.preset.blacklist.has_blocked_tag(tags)
            {
                return true;
            }
        }
        false
    }

    /// Check if a category is blacklisted by any active preset.
    #[must_use]
    pub fn is_category_blacklisted(&self, category: &str) -> bool {
        for name in &self.active_stack {
            if let Some(loaded) = self.presets.get(name)
                && loaded.preset.blacklist.is_category_blocked(category)
            {
                return true;
            }
        }
        false
    }

    /// Check if a component is whitelisted (preferred) by any active preset.
    #[must_use]
    pub fn is_component_whitelisted(&self, component_id: &str) -> bool {
        for name in &self.active_stack {
            if let Some(loaded) = self.presets.get(name)
                && loaded.preset.whitelist.is_component_preferred(component_id)
            {
                return true;
            }
        }
        false
    }

    /// Load built-in presets embedded in the binary.
    fn load_builtin_presets(&mut self) {
        for preset in builtin_presets() {
            self.presets.insert(
                preset.name.clone(),
                LoadedPreset {
                    preset,
                    source: PresetSource::BuiltIn,
                    path: None,
                },
            );
        }
    }

    /// Load presets from user config directory.
    fn load_user_presets(&mut self) -> Result<(), PresetError> {
        if let Some(config_dir) = dirs::config_dir() {
            let presets_dir = config_dir.join("draftkit").join("presets");
            if presets_dir.exists() {
                self.load_from_directory(&presets_dir, PresetSource::User)?;
            }
        }
        Ok(())
    }

    /// Load presets from project-local .draftkit directory.
    fn load_project_presets(&mut self) -> Result<(), PresetError> {
        let project_dir = PathBuf::from(".draftkit").join("presets");
        if project_dir.exists() {
            self.load_from_directory(&project_dir, PresetSource::Project)?;
        }
        Ok(())
    }

    /// Load all .toml files from a directory.
    fn load_from_directory(&mut self, dir: &Path, source: PresetSource) -> Result<(), PresetError> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "toml") {
                match self.load_preset_file(&path, source) {
                    Ok(loaded) => {
                        self.presets.insert(loaded.preset.name.clone(), loaded);
                    }
                    Err(e) => {
                        // Log warning but continue loading other presets
                        eprintln!("Warning: Failed to load preset {}: {e}", path.display());
                    }
                }
            }
        }
        Ok(())
    }

    /// Load a single preset file.
    fn load_preset_file(
        &self,
        path: &Path,
        source: PresetSource,
    ) -> Result<LoadedPreset, PresetError> {
        let content = std::fs::read_to_string(path)?;
        let preset_file: PresetFile = toml::from_str(&content)?;

        Ok(LoadedPreset {
            preset: preset_file.preset,
            source,
            path: Some(path.to_path_buf()),
        })
    }

    /// Reload presets from all sources.
    ///
    /// # Errors
    /// Returns error if preset directories cannot be read.
    pub fn reload(&mut self) -> Result<(), PresetError> {
        // Preserve active stack
        let active = self.active_stack.clone();

        self.presets.clear();
        self.load_builtin_presets();
        self.load_user_presets()?;
        self.load_project_presets()?;

        // Restore active stack, removing any that no longer exist
        self.active_stack = active
            .into_iter()
            .filter(|name| self.presets.contains_key(name))
            .collect();

        Ok(())
    }
}

impl Default for PresetLoader {
    fn default() -> Self {
        Self::builtin_only()
    }
}

/// Merge source overrides into destination (source wins for conflicts).
fn merge_style_overrides(dest: &mut StyleOverrides, source: &StyleOverrides) {
    if source.visual_weight_max.is_some() {
        dest.visual_weight_max = source.visual_weight_max;
    }
    if source.visual_weight_min.is_some() {
        dest.visual_weight_min = source.visual_weight_min;
    }
    if source.formality_max.is_some() {
        dest.formality_max = source.formality_max;
    }
    if source.formality_min.is_some() {
        dest.formality_min = source.formality_min;
    }
    if source.color_intensity_max.is_some() {
        dest.color_intensity_max = source.color_intensity_max;
    }
    if source.color_intensity_min.is_some() {
        dest.color_intensity_min = source.color_intensity_min;
    }
    if source.spacing_density_max.is_some() {
        dest.spacing_density_max = source.spacing_density_max;
    }
    if source.spacing_density_min.is_some() {
        dest.spacing_density_min = source.spacing_density_min;
    }
    if source.typography_scales.is_some() {
        dest.typography_scales = source.typography_scales.clone();
    }
}

/// Generate built-in presets.
///
/// These are example presets that ship with draftkit.
fn builtin_presets() -> Vec<Preset> {
    vec![
        minimalist_preset(),
        neubrutalism_preset(),
        corporate_preset(),
    ]
}

/// Minimalist aesthetic preset.
fn minimalist_preset() -> Preset {
    use crate::preset::schema::*;

    Preset {
        name: "Minimalist".to_string(),
        version: "1.0.0".to_string(),
        author: "draftkit".to_string(),
        description: "Clean, minimal design with lots of whitespace and subtle colors".to_string(),
        extends: None,
        tags: vec![
            "minimal".into(),
            "clean".into(),
            "whitespace".into(),
            "subtle".into(),
        ],
        style_overrides: StyleOverrides {
            visual_weight_max: Some(0.3),
            spacing_density_min: Some(0.7),
            color_intensity_max: Some(0.3),
            formality_min: Some(0.5),
            ..Default::default()
        },
        variant_preferences: HashMap::from([
            ("hero".into(), "hero-centered-cta".into()),
            ("header".into(), "header-simple-centered".into()),
            ("footer".into(), "footer-minimal".into()),
        ]),
        blacklist: Blacklist {
            components: vec![
                "hero-video-background".into(),
                "cta-branded-background".into(),
            ],
            tags: vec!["animated".into(), "video".into(), "gradient".into()],
            categories: vec![],
        },
        whitelist: Whitelist::default(),
    }
}

/// Neubrutalism aesthetic preset.
fn neubrutalism_preset() -> Preset {
    use crate::preset::schema::*;

    Preset {
        name: "Neubrutalism".to_string(),
        version: "1.0.0".to_string(),
        author: "draftkit".to_string(),
        description: "Bold, high-contrast design with strong shadows and vibrant colors"
            .to_string(),
        extends: None,
        tags: vec![
            "bold".into(),
            "colorful".into(),
            "shadows".into(),
            "playful".into(),
        ],
        style_overrides: StyleOverrides {
            visual_weight_min: Some(0.6),
            color_intensity_min: Some(0.6),
            formality_max: Some(0.4),
            ..Default::default()
        },
        variant_preferences: HashMap::from([
            ("hero".into(), "hero-split-screenshot".into()),
            ("features".into(), "feature-grid-icons".into()),
        ]),
        blacklist: Blacklist {
            tags: vec!["subtle".into(), "muted".into()],
            ..Default::default()
        },
        whitelist: Whitelist {
            tags: vec!["bold".into(), "shadow".into()],
            ..Default::default()
        },
    }
}

/// Corporate/enterprise aesthetic preset.
fn corporate_preset() -> Preset {
    use crate::preset::schema::*;

    Preset {
        name: "Corporate".to_string(),
        version: "1.0.0".to_string(),
        author: "draftkit".to_string(),
        description: "Professional, formal design suitable for enterprise and B2B".to_string(),
        extends: None,
        tags: vec![
            "professional".into(),
            "enterprise".into(),
            "b2b".into(),
            "formal".into(),
        ],
        style_overrides: StyleOverrides {
            formality_min: Some(0.7),
            visual_weight_max: Some(0.5),
            color_intensity_max: Some(0.5),
            ..Default::default()
        },
        variant_preferences: HashMap::from([
            ("header".into(), "header-with-cta".into()),
            ("pricing".into(), "pricing-four-tier-enterprise".into()),
            ("footer".into(), "footer-four-column".into()),
        ]),
        blacklist: Blacklist {
            tags: vec!["playful".into(), "animated".into(), "quirky".into()],
            ..Default::default()
        },
        whitelist: Whitelist::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_presets_load() {
        let loader = PresetLoader::builtin_only();
        assert!(loader.get("Minimalist").is_some());
        assert!(loader.get("Neubrutalism").is_some());
        assert!(loader.get("Corporate").is_some());
    }

    #[test]
    fn activate_and_deactivate() {
        let mut loader = PresetLoader::builtin_only();

        loader.activate("Minimalist").unwrap();
        assert_eq!(loader.active_stack(), &["Minimalist"]);

        loader.activate("Corporate").unwrap();
        assert_eq!(loader.active_stack(), &["Minimalist", "Corporate"]);

        loader.deactivate("Minimalist");
        assert_eq!(loader.active_stack(), &["Corporate"]);
    }

    #[test]
    fn set_stack() {
        let mut loader = PresetLoader::builtin_only();

        loader
            .set_stack(vec!["Corporate".to_string(), "Minimalist".to_string()])
            .unwrap();

        assert_eq!(loader.active_stack(), &["Corporate", "Minimalist"]);
    }

    #[test]
    fn set_stack_not_found() {
        let mut loader = PresetLoader::builtin_only();
        let result = loader.set_stack(vec!["NonExistent".to_string()]);
        assert!(matches!(result, Err(PresetError::NotFound(_))));
    }

    #[test]
    fn merged_style_overrides() {
        let mut loader = PresetLoader::builtin_only();

        // Minimalist: visual_weight_max = 0.3
        loader.activate("Minimalist").unwrap();

        // Neubrutalism: visual_weight_min = 0.6 (stacks on top)
        loader.activate("Neubrutalism").unwrap();

        let merged = loader.merged_style_overrides();
        assert_eq!(merged.visual_weight_max, Some(0.3)); // From Minimalist
        assert_eq!(merged.visual_weight_min, Some(0.6)); // From Neubrutalism
    }

    #[test]
    fn merged_variant_preferences() {
        let mut loader = PresetLoader::builtin_only();

        loader.activate("Minimalist").unwrap();
        loader.activate("Corporate").unwrap();

        let merged = loader.merged_variant_preferences();

        // Corporate overrides Minimalist for header
        assert_eq!(merged.get("header"), Some(&"header-with-cta".to_string()));
        // Minimalist's hero preference remains (Corporate doesn't override)
        assert_eq!(merged.get("hero"), Some(&"hero-centered-cta".to_string()));
    }

    #[test]
    fn blacklist_check() {
        let mut loader = PresetLoader::builtin_only();
        loader.activate("Minimalist").unwrap();

        assert!(loader.is_component_blacklisted("hero-video-background"));
        assert!(!loader.is_component_blacklisted("hero-simple"));
        assert!(loader.has_blacklisted_tag(&["animated".to_string()]));
    }

    #[test]
    fn activate_not_found() {
        let mut loader = PresetLoader::builtin_only();
        let result = loader.activate("NonExistent");
        assert!(matches!(result, Err(PresetError::NotFound(_))));
    }

    #[test]
    fn empty_loader() {
        let loader = PresetLoader::empty();
        assert!(loader.list_names().is_empty());
        assert!(!loader.has_active_presets());
    }
}
