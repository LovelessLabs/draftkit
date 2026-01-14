//! Catalyst UI Kit components embedded at compile time.
//!
//! Catalyst is a collection of atomic React components designed to work with Tailwind CSS.
//! Components are available in both TypeScript (.tsx) and JavaScript (.jsx) formats.

use include_dir::{Dir, include_dir};

/// Embedded TypeScript Catalyst components
static TYPESCRIPT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cache/kits/catalyst/typescript");

/// Embedded JavaScript Catalyst components
static JAVASCRIPT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cache/kits/catalyst/javascript");

/// Language variant for Catalyst components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystLanguage {
    TypeScript,
    JavaScript,
}

impl CatalystLanguage {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }

    #[must_use]
    pub const fn extension(&self) -> &'static str {
        match self {
            Self::TypeScript => "tsx",
            Self::JavaScript => "jsx",
        }
    }

    /// Parse language from string (e.g., "typescript", "ts", "tsx")
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "typescript" | "ts" | "tsx" => Some(Self::TypeScript),
            "javascript" | "js" | "jsx" => Some(Self::JavaScript),
            _ => None,
        }
    }
}

impl std::fmt::Display for CatalystLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Catalyst component metadata
#[derive(Debug, Clone)]
pub struct CatalystComponent {
    /// Component name (e.g., "button", "dialog")
    pub name: String,
    /// Component description
    pub description: String,
    /// File extension
    pub extension: &'static str,
}

/// List all available Catalyst component names
pub fn list_components() -> Vec<String> {
    TYPESCRIPT_DIR
        .files()
        .filter_map(|f| {
            f.path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(String::from)
        })
        .collect()
}

/// Get component metadata for all Catalyst components
pub fn get_component_metadata() -> Vec<CatalystComponent> {
    let descriptions = component_descriptions();

    list_components()
        .into_iter()
        .map(|name| {
            let description = descriptions
                .get(name.as_str())
                .copied()
                .unwrap_or("Catalyst UI component")
                .to_string();

            CatalystComponent {
                name,
                description,
                extension: "tsx",
            }
        })
        .collect()
}

/// Get the source code for a Catalyst component
pub fn get_component(name: &str, language: CatalystLanguage) -> Option<&'static str> {
    let dir = match language {
        CatalystLanguage::TypeScript => &TYPESCRIPT_DIR,
        CatalystLanguage::JavaScript => &JAVASCRIPT_DIR,
    };

    let extension = language.extension();
    let filename = format!("{name}.{extension}");

    dir.get_file(&filename).and_then(|f| f.contents_utf8())
}

/// Get all components as (name, code) pairs for a given language
pub fn get_all_components(language: CatalystLanguage) -> Vec<(&'static str, &'static str)> {
    let dir = match language {
        CatalystLanguage::TypeScript => &TYPESCRIPT_DIR,
        CatalystLanguage::JavaScript => &JAVASCRIPT_DIR,
    };

    dir.files()
        .filter_map(|f| {
            let name = f.path().file_stem()?.to_str()?;
            let content = f.contents_utf8()?;
            Some((name, content))
        })
        .collect()
}

/// Component descriptions for Catalyst UI Kit
fn component_descriptions() -> std::collections::HashMap<&'static str, &'static str> {
    [
        (
            "alert",
            "Alert banners for displaying important messages with various severity levels",
        ),
        (
            "auth-layout",
            "Layout component for authentication pages (login, signup, etc.)",
        ),
        (
            "avatar",
            "User avatar with image, initials, or placeholder fallback",
        ),
        ("badge", "Small status indicators and labels"),
        (
            "button",
            "Primary, secondary, and soft button variants with loading states",
        ),
        (
            "checkbox",
            "Checkbox input with label and description support",
        ),
        (
            "combobox",
            "Searchable select dropdown with keyboard navigation",
        ),
        (
            "description-list",
            "Key-value pairs in a structured list format",
        ),
        ("dialog", "Modal dialog with backdrop and focus trapping"),
        ("divider", "Horizontal or vertical separator lines"),
        (
            "dropdown",
            "Menu dropdown with items, sections, and keyboard navigation",
        ),
        ("fieldset", "Form fieldset with legend and field grouping"),
        (
            "heading",
            "Typography component for page and section headings",
        ),
        (
            "input",
            "Text input field with label, description, and error states",
        ),
        ("link", "Styled anchor links with hover and focus states"),
        (
            "listbox",
            "Selection list with single or multiple selection",
        ),
        ("navbar", "Top navigation bar with responsive menu"),
        (
            "pagination",
            "Page navigation with previous/next and page numbers",
        ),
        ("radio", "Radio button group for single selection"),
        ("select", "Native select dropdown with custom styling"),
        (
            "sidebar-layout",
            "Layout with collapsible sidebar navigation",
        ),
        ("sidebar", "Vertical navigation sidebar with sections"),
        (
            "stacked-layout",
            "Full-width layout with header and content sections",
        ),
        ("switch", "Toggle switch for boolean settings"),
        (
            "table",
            "Data table with sorting, pagination, and row actions",
        ),
        ("text", "Typography component for body text and paragraphs"),
        ("textarea", "Multi-line text input with auto-resize option"),
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_components() {
        let components = list_components();
        assert!(!components.is_empty());
        assert!(components.contains(&"button".to_string()));
        assert!(components.contains(&"dialog".to_string()));
    }

    #[test]
    fn test_get_component_typescript() {
        let button = get_component("button", CatalystLanguage::TypeScript);
        assert!(button.is_some());
        assert!(button.unwrap().contains("Button"));
    }

    #[test]
    fn test_get_component_javascript() {
        let button = get_component("button", CatalystLanguage::JavaScript);
        assert!(button.is_some());
        assert!(button.unwrap().contains("Button"));
    }

    #[test]
    fn test_nonexistent_component() {
        let result = get_component("nonexistent", CatalystLanguage::TypeScript);
        assert!(result.is_none());
    }

    #[test]
    fn test_language_parse() {
        assert_eq!(
            CatalystLanguage::parse("typescript"),
            Some(CatalystLanguage::TypeScript)
        );
        assert_eq!(
            CatalystLanguage::parse("ts"),
            Some(CatalystLanguage::TypeScript)
        );
        assert_eq!(
            CatalystLanguage::parse("javascript"),
            Some(CatalystLanguage::JavaScript)
        );
        assert_eq!(
            CatalystLanguage::parse("js"),
            Some(CatalystLanguage::JavaScript)
        );
        assert_eq!(CatalystLanguage::parse("invalid"), None);
    }

    #[test]
    fn test_language_as_str() {
        assert_eq!(CatalystLanguage::TypeScript.as_str(), "typescript");
        assert_eq!(CatalystLanguage::JavaScript.as_str(), "javascript");
    }

    #[test]
    fn test_language_extension() {
        assert_eq!(CatalystLanguage::TypeScript.extension(), "tsx");
        assert_eq!(CatalystLanguage::JavaScript.extension(), "jsx");
    }

    #[test]
    fn test_language_display() {
        assert_eq!(format!("{}", CatalystLanguage::TypeScript), "typescript");
        assert_eq!(format!("{}", CatalystLanguage::JavaScript), "javascript");
    }

    #[test]
    fn test_language_parse_tsx_jsx() {
        assert_eq!(
            CatalystLanguage::parse("tsx"),
            Some(CatalystLanguage::TypeScript)
        );
        assert_eq!(
            CatalystLanguage::parse("jsx"),
            Some(CatalystLanguage::JavaScript)
        );
    }

    #[test]
    fn test_get_component_metadata() {
        let metadata = get_component_metadata();
        assert!(!metadata.is_empty());

        // Find button component
        let button = metadata.iter().find(|c| c.name == "button");
        assert!(button.is_some());
        let button = button.unwrap();
        assert!(!button.description.is_empty());
        assert_eq!(button.extension, "tsx");
    }

    #[test]
    fn test_get_all_components_typescript() {
        let components = get_all_components(CatalystLanguage::TypeScript);
        assert!(!components.is_empty());

        // Should have button
        let button = components.iter().find(|(name, _)| *name == "button");
        assert!(button.is_some());
        assert!(button.unwrap().1.contains("Button"));
    }

    #[test]
    fn test_get_all_components_javascript() {
        let components = get_all_components(CatalystLanguage::JavaScript);
        assert!(!components.is_empty());

        // Should have button
        let button = components.iter().find(|(name, _)| *name == "button");
        assert!(button.is_some());
    }

    #[test]
    fn test_component_descriptions_coverage() {
        // Ensure descriptions map has reasonable size
        let descriptions = component_descriptions();
        assert!(descriptions.len() >= 20); // We defined ~27 descriptions
    }
}
