//! TailwindPlus Elements documentation with runtime-first, embedded-fallback loading.
//!
//! Loading priority:
//! 1. Runtime data directory (`~/.local/share/draftkit/elements/llms.txt`)
//! 2. Embedded data (compile-time via `include_str!`)
//!
//! Elements is a JavaScript UI component library with 9 interactive Web Components.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::data_dir::runtime_elements_dir;

/// Embedded Elements documentation
#[cfg(feature = "embedded-data")]
static EMBEDDED_ELEMENTS_DOC: &str = include_str!("../cache/docs/elements-llms.txt");

/// Stub documentation when no embedded data
#[cfg(not(feature = "embedded-data"))]
static EMBEDDED_ELEMENTS_DOC: &str = "";

/// Cached Elements documentation (runtime or embedded)
static ELEMENTS_DOC: OnceLock<Cow<'static, str>> = OnceLock::new();

/// Try to load elements doc from runtime directory.
fn load_runtime_elements_doc() -> Option<String> {
    let dir = runtime_elements_dir()?;
    let path = dir.join("llms.txt");
    std::fs::read_to_string(path.as_std_path()).ok()
}

/// Get the Elements documentation, trying runtime first then embedded.
fn get_elements_doc() -> &'static Cow<'static, str> {
    ELEMENTS_DOC.get_or_init(|| {
        // Try runtime first
        if let Some(doc) = load_runtime_elements_doc() {
            return Cow::Owned(doc);
        }
        // Fall back to embedded
        Cow::Borrowed(EMBEDDED_ELEMENTS_DOC)
    })
}

/// Element component names (lowercase for matching)
const ELEMENT_NAMES: &[&str] = &[
    "autocomplete",
    "command-palette",
    "copy-button",
    "dialog",
    "disclosure",
    "dropdown-menu",
    "popover",
    "select",
    "tabs",
];

/// Element component metadata
#[derive(Debug, Clone)]
pub struct ElementInfo {
    pub name: &'static str,
    pub tag: &'static str,
    pub description: &'static str,
    pub use_cases: &'static [&'static str],
}

/// Get metadata for all Elements components
#[must_use]
pub fn list_elements() -> Vec<ElementInfo> {
    vec![
        ElementInfo {
            name: "Autocomplete",
            tag: "<el-autocomplete>",
            description: "Text input with filtered suggestions, like a styled <datalist>",
            use_cases: &[
                "search inputs",
                "form fields with suggestions",
                "comboboxes",
            ],
        },
        ElementInfo {
            name: "Command Palette",
            tag: "<el-command-palette>",
            description: "Keyboard-friendly Cmd+K style command menu",
            use_cases: &["app navigation", "quick actions", "search interfaces"],
        },
        ElementInfo {
            name: "Copy Button",
            tag: "<el-copy>",
            description: "Button that copies text to clipboard with feedback",
            use_cases: &["code snippets", "share links", "API keys"],
        },
        ElementInfo {
            name: "Dialog",
            tag: "<el-dialog>",
            description: "Modal dialog with backdrop, scroll lock, and transitions",
            use_cases: &["confirmations", "forms", "detail views", "alerts"],
        },
        ElementInfo {
            name: "Disclosure",
            tag: "<el-disclosure>",
            description: "Expandable/collapsible content sections",
            use_cases: &["accordions", "FAQs", "expandable details"],
        },
        ElementInfo {
            name: "Dropdown Menu",
            tag: "<el-dropdown>",
            description: "Dropdown menu with keyboard navigation",
            use_cases: &["action menus", "option selectors", "context menus"],
        },
        ElementInfo {
            name: "Popover",
            tag: "<el-popover>",
            description: "Floating panel anchored to a trigger element",
            use_cases: &["tooltips", "flyouts", "info panels", "settings"],
        },
        ElementInfo {
            name: "Select",
            tag: "<el-select>",
            description: "Styled replacement for native select dropdowns",
            use_cases: &["form selects", "option pickers", "filters"],
        },
        ElementInfo {
            name: "Tabs",
            tag: "<el-tab-group>",
            description: "Accessible tabbed interface with keyboard navigation",
            use_cases: &["content sections", "settings panels", "dashboards"],
        },
    ]
}

/// Get the overview section (installation, browser support, etc.)
#[must_use]
pub fn get_overview() -> String {
    let doc = get_elements_doc();
    // Find where the first component section starts (## Autocomplete)
    doc.find("\n## Autocomplete")
        .map_or_else(|| doc.to_string(), |pos| doc[..pos].to_string())
}

/// Parse element name from various formats
fn normalize_name(name: &str) -> Option<&'static str> {
    let lower = name.to_lowercase();
    // Order matters: strip angle brackets first, then el- prefix
    let normalized = lower
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim_start_matches("el-")
        .replace(['_', ' '], "-");

    ELEMENT_NAMES
        .iter()
        .find(|&&n| n == normalized || n.replace('-', "") == normalized.replace('-', ""))
        .copied()
}

/// Get documentation for a specific Element component
#[must_use]
pub fn get_element_docs(name: &str) -> Option<String> {
    let normalized = normalize_name(name)?;
    let doc = get_elements_doc();

    // Build section header patterns
    let section_headers: HashMap<&str, &str> = [
        ("autocomplete", "## Autocomplete"),
        ("command-palette", "## Command palette"),
        ("copy-button", "## Copy button"),
        ("dialog", "## Dialog"),
        ("disclosure", "## Disclosure"),
        ("dropdown-menu", "## Dropdown menu"),
        ("popover", "## Popover"),
        ("select", "## Select"),
        ("tabs", "## Tabs"),
    ]
    .into_iter()
    .collect();

    let header = section_headers.get(normalized)?;

    // Find the start of this section
    let start = doc.find(header)?;

    // Find the end (next ## section or end of file)
    let content_after_header = &doc[start + header.len()..];
    let end = content_after_header
        .find("\n## ")
        .map(|pos| start + header.len() + pos)
        .unwrap_or(doc.len());

    Some(doc[start..end].to_string())
}

/// Get the full Elements documentation
#[must_use]
pub fn get_full_docs() -> Cow<'static, str> {
    get_elements_doc().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that require embedded data
    #[cfg(feature = "embedded-data")]
    mod embedded_tests {
        use super::*;

        #[test]
        fn test_get_overview() {
            let overview = get_overview();
            assert!(overview.contains("Tailwind Plus Elements"));
            assert!(overview.contains("Installing"));
        }

        #[test]
        fn test_get_element_docs() {
            // Test various name formats
            assert!(get_element_docs("dialog").is_some());
            assert!(get_element_docs("Dialog").is_some());
            assert!(get_element_docs("el-dialog").is_some());
            assert!(get_element_docs("<el-dialog>").is_some());
        }
    }

    // Tests that work without embedded data
    #[test]
    fn test_list_elements() {
        let elements = list_elements();
        assert_eq!(elements.len(), 9);
        assert!(elements.iter().any(|e| e.name == "Dialog"));
        assert!(elements.iter().any(|e| e.name == "Tabs"));
    }

    #[test]
    fn test_nonexistent_element() {
        assert!(get_element_docs("nonexistent").is_none());
        assert!(get_element_docs("button").is_none()); // Not an Element
    }

    #[test]
    fn test_normalize_name() {
        assert_eq!(normalize_name("dialog"), Some("dialog"));
        assert_eq!(normalize_name("DIALOG"), Some("dialog"));
        assert_eq!(normalize_name("el-dialog"), Some("dialog"));
        assert_eq!(normalize_name("<el-dialog>"), Some("dialog"));
        assert_eq!(normalize_name("command-palette"), Some("command-palette"));
        assert_eq!(normalize_name("commandpalette"), Some("command-palette"));
        assert_eq!(normalize_name("command_palette"), Some("command-palette"));
        assert_eq!(normalize_name("dropdown-menu"), Some("dropdown-menu"));
        assert_eq!(normalize_name("dropdownmenu"), Some("dropdown-menu"));
    }
}
