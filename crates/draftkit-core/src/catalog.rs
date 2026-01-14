//! Unified catalog API for all Draftkit data access.
//!
//! The [`Catalog`] struct provides a single entry point for accessing:
//! - TailwindPlus UI components (HTML, React, Vue)
//! - Catalyst UI Kit components (TypeScript, JavaScript)
//! - Tailwind CSS documentation (v3, v4)
//! - TailwindPlus Elements documentation

use crate::catalyst::{self, CatalystComponent, CatalystLanguage};
use crate::components::{
    ComponentReader, ComponentRecord, Framework, Mode, NdjsonSnippet, TailwindVersion,
};
use crate::docs::{self, TopicInfo};
use crate::elements::{self, ElementInfo};

/// Unified facade for all Draftkit data access.
///
/// This is the primary API for consumers of draftkit-core. It provides
/// methods for searching, retrieving, and listing all embedded content.
#[derive(Clone, Default)]
pub struct Catalog {
    components: ComponentReader,
}

impl Catalog {
    /// Create a new Catalog instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // TailwindPlus Components
    // ─────────────────────────────────────────────────────────────────────────

    /// Search components by keyword in name/category/subcategory.
    #[must_use]
    pub fn search_components(
        &self,
        framework: Framework,
        query: &str,
    ) -> Vec<&'static ComponentRecord> {
        self.components.search(framework, query)
    }

    /// Get a component by its ID.
    #[must_use]
    pub fn get_component_by_id(
        &self,
        framework: Framework,
        id: &str,
    ) -> Option<&'static ComponentRecord> {
        self.components.find_by_id(framework, id)
    }

    /// Get a specific code snippet from a component.
    #[must_use]
    pub fn get_component_snippet(
        &self,
        framework: Framework,
        id: &str,
        mode: Mode,
    ) -> Option<&'static NdjsonSnippet> {
        self.components
            .find_by_id(framework, id)
            .and_then(|c| c.get_snippet(mode))
    }

    /// Get all components for a framework.
    #[must_use]
    pub fn list_components(&self, framework: Framework) -> &'static [ComponentRecord] {
        self.components.all(framework)
    }

    /// Get component count for a framework.
    #[must_use]
    pub fn component_count(&self, framework: Framework) -> usize {
        self.components.component_count(framework)
    }

    /// Check if a framework is available.
    #[must_use]
    pub fn has_framework(&self, framework: Framework) -> bool {
        self.components.has_framework(framework)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Catalyst UI Kit
    // ─────────────────────────────────────────────────────────────────────────

    /// List all Catalyst component names.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Not const when embedded-data is enabled
    pub fn list_catalyst_names(&self) -> Vec<String> {
        catalyst::list_components()
    }

    /// Get metadata for all Catalyst components.
    #[must_use]
    pub fn list_catalyst_components(&self) -> Vec<CatalystComponent> {
        catalyst::get_component_metadata()
    }

    /// Get source code for a Catalyst component.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Not const when embedded-data is enabled
    pub fn get_catalyst_component(
        &self,
        name: &str,
        language: CatalystLanguage,
    ) -> Option<&'static str> {
        catalyst::get_component(name, language)
    }

    /// Get all Catalyst components for a language.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Not const when embedded-data is enabled
    pub fn get_all_catalyst_components(
        &self,
        language: CatalystLanguage,
    ) -> Vec<(&'static str, &'static str)> {
        catalyst::get_all_components(language)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Tailwind CSS Documentation
    // ─────────────────────────────────────────────────────────────────────────

    /// Get documentation content for a topic.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Not const when embedded-data is enabled
    pub fn get_tailwind_docs(&self, topic: &str, version: TailwindVersion) -> Option<&'static str> {
        docs::get_docs(topic, version)
    }

    /// List available topics for a Tailwind version.
    #[must_use]
    pub fn list_tailwind_topics(
        &self,
        version: TailwindVersion,
    ) -> Vec<(&'static str, &'static str)> {
        docs::list_topics(version)
    }

    /// Get all topics with version availability info.
    #[must_use]
    pub fn list_all_tailwind_topics(&self) -> &'static [TopicInfo] {
        docs::list_all_topics()
    }

    /// Search documentation topics by keyword.
    #[must_use]
    pub fn search_tailwind_topics(
        &self,
        query: &str,
        version: TailwindVersion,
    ) -> Vec<(&'static str, &'static str)> {
        docs::search_topics(query, version)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // TailwindPlus Elements
    // ─────────────────────────────────────────────────────────────────────────

    /// List all Elements components with metadata.
    #[must_use]
    pub fn list_elements(&self) -> Vec<ElementInfo> {
        elements::list_elements()
    }

    /// Get documentation for a specific Element component.
    #[must_use]
    pub fn get_element_docs(&self, name: &str) -> Option<String> {
        elements::get_element_docs(name)
    }

    /// Get the Elements overview documentation.
    #[must_use]
    pub fn get_elements_overview(&self) -> &'static str {
        elements::get_overview()
    }

    /// Get full Elements documentation.
    #[must_use]
    pub fn get_full_elements_docs(&self) -> &'static str {
        elements::get_full_docs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_new() {
        let catalog = Catalog::new();
        // Should not panic
        let _ = catalog;
    }

    #[test]
    fn test_catalog_default() {
        let catalog = Catalog::default();
        let _ = catalog;
    }

    #[test]
    fn test_component_operations() {
        let catalog = Catalog::new();

        // These should work without panicking
        let _count = catalog.component_count(Framework::React);
        let _all = catalog.list_components(Framework::React);
        let _search = catalog.search_components(Framework::React, "button");
    }

    // Tests that require embedded data
    #[cfg(feature = "embedded-data")]
    mod embedded_tests {
        use super::*;

        #[test]
        fn test_catalyst_operations() {
            let catalog = Catalog::new();

            let names = catalog.list_catalyst_names();
            assert!(!names.is_empty());
            assert!(names.contains(&"button".to_string()));

            let metadata = catalog.list_catalyst_components();
            assert!(!metadata.is_empty());

            let button = catalog.get_catalyst_component("button", CatalystLanguage::TypeScript);
            assert!(button.is_some());
        }

        #[test]
        fn test_docs_operations() {
            let catalog = Catalog::new();

            let topics = catalog.list_tailwind_topics(TailwindVersion::V4);
            assert!(!topics.is_empty());

            let flexbox = catalog.get_tailwind_docs("flexbox", TailwindVersion::V4);
            assert!(flexbox.is_some());

            let search = catalog.search_tailwind_topics("flex", TailwindVersion::V4);
            assert!(!search.is_empty());
        }

        #[test]
        fn test_elements_operations() {
            let catalog = Catalog::new();

            let elements = catalog.list_elements();
            assert_eq!(elements.len(), 9);

            let dialog = catalog.get_element_docs("dialog");
            assert!(dialog.is_some());

            let overview = catalog.get_elements_overview();
            assert!(overview.contains("Tailwind Plus Elements"));
        }
    }
}
