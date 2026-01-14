//! NDJSON component reader with compile-time embedded data.
//!
//! Each NDJSON file contains components, one per line.
//! Files are embedded at compile time via the cache symlink.
//!
//! When the `embedded-data` feature is disabled, this module provides stub implementations
//! that return empty results. This allows CI to build without the cache directory present.

#[cfg(feature = "embedded-data")]
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::types::{Framework, Mode};

/// Embedded component data directory (via symlink: cache -> ../../cache/current)
#[cfg(feature = "embedded-data")]
static COMPONENTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cache/data/components");

/// Snippet data from NDJSON
#[derive(Debug, Clone, Deserialize)]
pub struct NdjsonSnippet {
    pub code: String,
    #[serde(default)]
    pub preview: Option<String>,
}

/// Component record from NDJSON line
#[derive(Debug, Clone, Deserialize)]
pub struct ComponentRecord {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub category: String,
    pub subcategory: String,
    pub sub_subcategory: String,
    pub light: Option<NdjsonSnippet>,
    pub dark: Option<NdjsonSnippet>,
    pub system: Option<NdjsonSnippet>,
}

impl ComponentRecord {
    /// Get snippet for a specific mode.
    ///
    /// For `Mode::None` (used by ecommerce components that don't have theme variants),
    /// falls back to the `light` variant as the base/default styling.
    #[must_use]
    pub const fn get_snippet(&self, mode: Mode) -> Option<&NdjsonSnippet> {
        match mode {
            Mode::Light => self.light.as_ref(),
            Mode::Dark => self.dark.as_ref(),
            Mode::System => self.system.as_ref(),
            // Ecommerce components use "none" mode; fall back to light as default
            Mode::None => self.light.as_ref(),
        }
    }
}

/// Parsed components index, lazily initialized
static COMPONENTS_INDEX: OnceLock<HashMap<Framework, Vec<ComponentRecord>>> = OnceLock::new();

/// Get or initialize the components index
#[cfg(feature = "embedded-data")]
fn get_components() -> &'static HashMap<Framework, Vec<ComponentRecord>> {
    COMPONENTS_INDEX.get_or_init(|| {
        let mut index = HashMap::new();

        for framework in [Framework::React, Framework::Vue, Framework::Html] {
            let filename = framework.ndjson_filename();
            if let Some(file) = COMPONENTS_DIR.get_file(filename) {
                if let Some(contents) = file.contents_utf8() {
                    let components: Vec<ComponentRecord> = contents
                        .lines()
                        .filter(|line| !line.is_empty())
                        .filter_map(|line| serde_json::from_str(line).ok())
                        .collect();
                    index.insert(framework, components);
                }
            }
        }

        index
    })
}

/// Get or initialize the components index (stub when no embedded data)
#[cfg(not(feature = "embedded-data"))]
fn get_components() -> &'static HashMap<Framework, Vec<ComponentRecord>> {
    COMPONENTS_INDEX.get_or_init(HashMap::new)
}

/// Component reader using embedded NDJSON data
#[derive(Clone)]
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
    fn test_component_record_get_snippet() {
        let record = ComponentRecord {
            id: "test".to_string(),
            uuid: "uuid".to_string(),
            name: "Test".to_string(),
            category: "UI".to_string(),
            subcategory: "Forms".to_string(),
            sub_subcategory: "Inputs".to_string(),
            light: Some(NdjsonSnippet {
                code: "light code".to_string(),
                preview: None,
            }),
            dark: Some(NdjsonSnippet {
                code: "dark code".to_string(),
                preview: Some("preview".to_string()),
            }),
            system: None,
        };

        assert!(record.get_snippet(Mode::Light).is_some());
        assert_eq!(record.get_snippet(Mode::Light).unwrap().code, "light code");
        assert!(record.get_snippet(Mode::Dark).is_some());
        assert!(record.get_snippet(Mode::System).is_none());
        // Mode::None falls back to light for ecommerce components
        assert!(record.get_snippet(Mode::None).is_some());
        assert_eq!(record.get_snippet(Mode::None).unwrap().code, "light code");
    }
}
