//! NDJSON component reader with runtime-first, embedded-fallback loading.
//!
//! Loading priority:
//! 1. Runtime data directory (`~/.local/share/draftkit/data/components/`)
//! 2. Embedded data (compile-time via `include_dir!`)
//!
//! Each NDJSON file contains components, one per line.
//! When the `embedded-data` feature is disabled and no runtime data exists,
//! this module returns empty results.

#[cfg(feature = "embedded-data")]
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::types::{ExtractedMeta, Framework, Mode};
use crate::data_dir::runtime_components_dir;

/// Embedded component data directory (via symlink: cache -> ../../cache/current)
#[cfg(feature = "embedded-data")]
static COMPONENTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cache/data/components");

/// Component record from NDJSON line (metadata-only format).
///
/// This struct represents the embedded/distributed component data which does NOT
/// include source code (for licensing compliance). Source code is fetched on-demand
/// via authenticated TailwindPlus session.
///
/// Fields:
/// - Identifiers: id, uuid, name, version
/// - Hierarchy: category, subcategory, sub_subcategory
/// - Availability: has_light, has_dark, has_system (which modes exist)
/// - Previews: preview_light, preview_dark, preview_system (image URLs)
/// - Analysis: meta (extracted dependencies, tokens, tailwind features)
#[derive(Debug, Clone, Deserialize)]
pub struct ComponentRecord {
    pub id: String,
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    pub category: String,
    pub subcategory: String,
    pub sub_subcategory: String,

    /// Whether light mode variant exists (code fetched on-demand)
    #[serde(default)]
    pub has_light: bool,
    /// Whether dark mode variant exists (code fetched on-demand)
    #[serde(default)]
    pub has_dark: bool,
    /// Whether system mode variant exists (code fetched on-demand)
    #[serde(default)]
    pub has_system: bool,

    /// Preview image URL for light mode
    #[serde(default)]
    pub preview_light: Option<String>,
    /// Preview image URL for dark mode
    #[serde(default)]
    pub preview_dark: Option<String>,
    /// Preview image URL for system mode
    #[serde(default)]
    pub preview_system: Option<String>,

    /// Extracted metadata (dependencies, tokens, compatibility).
    /// Populated by `scripts/metadata.sh` during data collection.
    #[serde(default)]
    pub meta: Option<ExtractedMeta>,
}

impl ComponentRecord {
    /// Check if a specific mode variant is available.
    ///
    /// For `Mode::None` (used by ecommerce components that don't have theme variants),
    /// falls back to checking the `light` variant as the base/default.
    #[must_use]
    pub const fn has_mode(&self, mode: Mode) -> bool {
        match mode {
            Mode::Light => self.has_light,
            Mode::Dark => self.has_dark,
            Mode::System => self.has_system,
            // Ecommerce components use "none" mode; fall back to light as default
            Mode::None => self.has_light,
        }
    }

    /// Get preview URL for a specific mode.
    #[must_use]
    pub fn preview_url(&self, mode: Mode) -> Option<&str> {
        match mode {
            Mode::Light | Mode::None => self.preview_light.as_deref(),
            Mode::Dark => self.preview_dark.as_deref(),
            Mode::System => self.preview_system.as_deref(),
        }
    }
}

/// Parsed components index, lazily initialized
static COMPONENTS_INDEX: OnceLock<HashMap<Framework, Vec<ComponentRecord>>> = OnceLock::new();

/// Try to load components from runtime data directory.
fn load_runtime_components() -> Option<HashMap<Framework, Vec<ComponentRecord>>> {
    let dir = runtime_components_dir()?;
    let mut index = HashMap::new();

    for framework in [Framework::React, Framework::Vue, Framework::Html] {
        let filename = framework.ndjson_filename();
        let path = dir.join(filename);
        if let Ok(contents) = std::fs::read_to_string(path.as_std_path()) {
            let components: Vec<ComponentRecord> = contents
                .lines()
                .filter(|line| !line.is_empty())
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
            if !components.is_empty() {
                index.insert(framework, components);
            }
        }
    }

    if index.is_empty() { None } else { Some(index) }
}

/// Load components from embedded data.
#[cfg(feature = "embedded-data")]
fn load_embedded_components() -> HashMap<Framework, Vec<ComponentRecord>> {
    let mut index = HashMap::new();

    for framework in [Framework::React, Framework::Vue, Framework::Html] {
        let filename = framework.ndjson_filename();
        if let Some(file) = COMPONENTS_DIR.get_file(filename)
            && let Some(contents) = file.contents_utf8()
        {
            let components: Vec<ComponentRecord> = contents
                .lines()
                .filter(|line| !line.is_empty())
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
            index.insert(framework, components);
        }
    }

    index
}

/// Load components from embedded data (stub when no embedded data).
#[cfg(not(feature = "embedded-data"))]
fn load_embedded_components() -> HashMap<Framework, Vec<ComponentRecord>> {
    HashMap::new()
}

/// Get or initialize the components index with runtime-first, embedded-fallback.
fn get_components() -> &'static HashMap<Framework, Vec<ComponentRecord>> {
    COMPONENTS_INDEX.get_or_init(|| {
        // Try runtime first
        if let Some(components) = load_runtime_components() {
            return components;
        }
        // Fall back to embedded
        load_embedded_components()
    })
}

/// Component reader using embedded NDJSON data
#[derive(Debug, Clone)]
pub struct ComponentReader;

impl ComponentReader {
    /// Create a new component reader (no-op, data is embedded)
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Get a component by line number and framework
    #[must_use]
    pub fn get_by_line(
        &self,
        framework: Framework,
        line_num: usize,
    ) -> Option<&'static ComponentRecord> {
        get_components()
            .get(&framework)
            .and_then(|components| components.get(line_num))
    }

    /// Get component count for a framework
    #[must_use]
    pub fn component_count(&self, framework: Framework) -> usize {
        get_components().get(&framework).map_or(0, Vec::len)
    }

    /// Check if a framework is available
    #[must_use]
    pub fn has_framework(&self, framework: Framework) -> bool {
        get_components().contains_key(&framework)
    }

    /// Find a component by ID
    #[must_use]
    pub fn find_by_id(&self, framework: Framework, id: &str) -> Option<&'static ComponentRecord> {
        get_components()
            .get(&framework)
            .and_then(|components| components.iter().find(|c| c.id == id))
    }

    /// Search components by keyword in name/category/subcategory
    #[must_use]
    pub fn search(&self, framework: Framework, query: &str) -> Vec<&'static ComponentRecord> {
        let query_lower = query.to_lowercase();
        get_components()
            .get(&framework)
            .map(|components| {
                components
                    .iter()
                    .filter(|c| {
                        c.name.to_lowercase().contains(&query_lower)
                            || c.category.to_lowercase().contains(&query_lower)
                            || c.subcategory.to_lowercase().contains(&query_lower)
                            || c.sub_subcategory.to_lowercase().contains(&query_lower)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all components for a framework
    #[must_use]
    pub fn all(&self, framework: Framework) -> &'static [ComponentRecord] {
        get_components().get(&framework).map_or(&[], Vec::as_slice)
    }
}

impl Default for ComponentReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that require embedded data
    #[cfg(feature = "embedded-data")]
    mod embedded_tests {
        use super::*;

        #[test]
        fn test_has_framework_with_data() {
            let reader = ComponentReader::new();
            // At least one framework should have data
            let has_any = reader.has_framework(Framework::React)
                || reader.has_framework(Framework::Vue)
                || reader.has_framework(Framework::Html);
            assert!(has_any);
        }
    }

    // Tests that work without embedded data
    #[test]
    fn test_framework_ndjson_filename() {
        assert_eq!(Framework::React.ndjson_filename(), "react-v4.ndjson");
        assert_eq!(Framework::Vue.ndjson_filename(), "vue-v4.ndjson");
        assert_eq!(Framework::Html.ndjson_filename(), "html-v4.ndjson");
    }

    #[test]
    fn test_framework_parse() {
        assert_eq!(Framework::parse("react"), Some(Framework::React));
        assert_eq!(Framework::parse("REACT"), Some(Framework::React));
        assert_eq!(Framework::parse("vue"), Some(Framework::Vue));
        assert_eq!(Framework::parse("html"), Some(Framework::Html));
        assert_eq!(Framework::parse("invalid"), None);
    }

    #[test]
    fn test_component_reader_new() {
        let reader = ComponentReader::new();
        // Should not panic - just creates the reader
        let _ = reader;
    }

    #[test]
    fn test_component_reader_default() {
        let reader: ComponentReader = Default::default();
        let _ = reader;
    }

    #[test]
    fn test_component_count_returns_zero_or_more() {
        let reader = ComponentReader::new();
        // Count should be non-negative (0 if no data, >0 if data present)
        let _count = reader.component_count(Framework::React);
    }

    #[test]
    fn test_has_framework_consistency() {
        let reader = ComponentReader::new();
        // has_framework should be consistent with component_count
        for framework in [Framework::React, Framework::Vue, Framework::Html] {
            let has = reader.has_framework(framework);
            let count = reader.component_count(framework);
            if has {
                assert!(
                    count > 0,
                    "has_framework=true but count=0 for {framework:?}"
                );
            }
        }
    }

    #[test]
    fn test_get_by_line_out_of_bounds() {
        let reader = ComponentReader::new();
        // Very large index should return None
        let result = reader.get_by_line(Framework::React, usize::MAX);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_by_id_nonexistent() {
        let reader = ComponentReader::new();
        let result = reader.find_by_id(Framework::React, "nonexistent-uuid-12345");
        assert!(result.is_none());
    }

    #[test]
    fn test_search_empty_query() {
        let reader = ComponentReader::new();
        // Empty query should still return results (matches everything)
        let results = reader.search(Framework::React, "");
        // Just verify it doesn't panic
        let _ = results;
    }

    #[test]
    fn test_search_no_matches() {
        let reader = ComponentReader::new();
        let results = reader.search(Framework::React, "xyzzy-impossible-match-12345");
        assert!(results.is_empty());
    }

    #[test]
    fn test_all_returns_slice() {
        let reader = ComponentReader::new();
        let all = reader.all(Framework::React);
        // Should be consistent with component_count
        assert_eq!(all.len(), reader.component_count(Framework::React));
    }

    #[test]
    fn test_component_record_has_mode() {
        let record = ComponentRecord {
            id: "test".to_string(),
            uuid: "uuid".to_string(),
            name: "Test".to_string(),
            version: Some("v4".to_string()),
            category: "UI".to_string(),
            subcategory: "Forms".to_string(),
            sub_subcategory: "Inputs".to_string(),
            has_light: true,
            has_dark: true,
            has_system: false,
            preview_light: None,
            preview_dark: Some("https://example.com/preview.png".to_string()),
            preview_system: None,
            meta: None,
        };

        assert!(record.has_mode(Mode::Light));
        assert!(record.has_mode(Mode::Dark));
        assert!(!record.has_mode(Mode::System));
        // Mode::None falls back to light for ecommerce components
        assert!(record.has_mode(Mode::None));
    }

    #[test]
    fn test_component_record_preview_url() {
        let record = ComponentRecord {
            id: "test".to_string(),
            uuid: "uuid".to_string(),
            name: "Test".to_string(),
            version: Some("v4".to_string()),
            category: "UI".to_string(),
            subcategory: "Forms".to_string(),
            sub_subcategory: "Inputs".to_string(),
            has_light: true,
            has_dark: true,
            has_system: false,
            preview_light: Some("https://example.com/light.png".to_string()),
            preview_dark: Some("https://example.com/dark.png".to_string()),
            preview_system: None,
            meta: None,
        };

        assert_eq!(
            record.preview_url(Mode::Light),
            Some("https://example.com/light.png")
        );
        assert_eq!(
            record.preview_url(Mode::Dark),
            Some("https://example.com/dark.png")
        );
        assert_eq!(record.preview_url(Mode::System), None);
        // Mode::None falls back to light
        assert_eq!(
            record.preview_url(Mode::None),
            Some("https://example.com/light.png")
        );
    }
}
