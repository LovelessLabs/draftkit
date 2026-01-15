//! Tailwind CSS documentation module with runtime-first, embedded-fallback loading.
//!
//! Loading priority:
//! 1. Runtime data directory (`~/.local/share/draftkit/docs/tailwind/`)
//! 2. Embedded data (compile-time via `include_dir!`)
//!
//! When the `embedded-data` feature is disabled and no runtime data exists,
//! this module returns empty results.

use std::borrow::Cow;

#[cfg(feature = "embedded-data")]
use include_dir::{Dir, include_dir};

use crate::components::TailwindVersion;
use crate::data_dir::runtime_docs_dir;

/// Embedded Tailwind CSS v3 documentation
#[cfg(feature = "embedded-data")]
static V3_DOCS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cache/docs/tailwind/v3");

/// Embedded Tailwind CSS v4 documentation
#[cfg(feature = "embedded-data")]
static V4_DOCS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/cache/docs/tailwind/v4");

/// Topic metadata with version availability
#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub v3: bool,
    pub v4: bool,
}

/// Available documentation topics with their descriptions and version availability
pub static TOPICS: &[TopicInfo] = &[
    TopicInfo {
        name: "index",
        description: "Documentation index and quick reference",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "flexbox",
        description: "Flex container, direction, wrap, justify, align, grow, shrink, basis",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "grid",
        description: "Grid container, columns, rows, spans, auto-flow, gap, place utilities",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "position",
        description: "Static, relative, absolute, fixed, sticky, inset, z-index",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "display",
        description: "Block, inline, flex, grid, hidden, visibility, overflow",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "spacing",
        description: "Padding, margin, space-between, negative margins",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "sizing",
        description: "Width, height, min/max width/height, size utility",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "typography",
        description: "Font family, size, weight, line height, text alignment, decoration",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "colors",
        description: "Color palette, text color, background color, opacity modifiers",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "backgrounds",
        description: "Background color, image, gradients, size, position, repeat",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "borders",
        description: "Border width, color, style, radius, divide, outline, ring",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "effects",
        description: "Box shadow, shadow color, opacity, blend modes",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "filters",
        description: "Blur, brightness, contrast, grayscale, backdrop filters",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "transforms",
        description: "Scale, rotate, translate, skew, 3D transforms",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "transitions",
        description: "Transition property, duration, timing, animations",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "interactivity",
        description: "Cursor, pointer events, resize, scroll, user select",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "states",
        description: "Hover, focus, active, disabled, group, peer, has, not",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "responsive",
        description: "Breakpoints, mobile-first, container queries",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "dark-mode",
        description: "Dark mode setup, class strategy, color patterns",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "accessibility",
        description: "Screen reader, focus styles, motion preferences",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "svg",
        description: "Fill, stroke, stroke width, icon patterns",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "forms",
        description: "Form elements: input, checkbox, radio, select (@tailwindcss/forms plugin for v3)",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "prose",
        description: "Typography for rendered content (@tailwindcss/typography plugin)",
        v3: true,
        v4: true,
    },
    TopicInfo {
        name: "v4-changes",
        description: "What's new in v4, migration from v3",
        v3: false,
        v4: true,
    },
];

/// Try to load documentation from runtime data directory.
fn load_runtime_docs(topic: &str, version: TailwindVersion) -> Option<String> {
    let version_str = match version {
        TailwindVersion::V3 => "v3",
        TailwindVersion::V4 => "v4",
    };
    let dir = runtime_docs_dir(version_str)?;
    let path = dir.join(format!("{topic}.md"));
    std::fs::read_to_string(path.as_std_path()).ok()
}

/// Load documentation from embedded data.
#[cfg(feature = "embedded-data")]
fn load_embedded_docs(topic: &str, version: TailwindVersion) -> Option<&'static str> {
    let dir = match version {
        TailwindVersion::V3 => &V3_DOCS,
        TailwindVersion::V4 => &V4_DOCS,
    };
    let filename = format!("{topic}.md");
    dir.get_file(&filename).and_then(|f| f.contents_utf8())
}

/// Load documentation from embedded data (stub when no embedded data).
#[cfg(not(feature = "embedded-data"))]
const fn load_embedded_docs(_topic: &str, _version: TailwindVersion) -> Option<&'static str> {
    None
}

/// Get documentation content for a topic with runtime-first, embedded-fallback.
///
/// # Arguments
/// * `topic` - The topic name (without .md extension)
/// * `version` - The Tailwind CSS version (v3 or v4)
///
/// # Returns
/// The markdown content if found, None otherwise
#[must_use]
pub fn get_docs(topic: &str, version: TailwindVersion) -> Option<Cow<'static, str>> {
    // Try runtime first
    if let Some(content) = load_runtime_docs(topic, version) {
        return Some(Cow::Owned(content));
    }
    // Fall back to embedded
    load_embedded_docs(topic, version).map(Cow::Borrowed)
}

/// List all available documentation topics for a given version
#[must_use]
pub fn list_topics(version: TailwindVersion) -> Vec<(&'static str, &'static str)> {
    TOPICS
        .iter()
        .filter(|t| match version {
            TailwindVersion::V3 => t.v3,
            TailwindVersion::V4 => t.v4,
        })
        .map(|t| (t.name, t.description))
        .collect()
}

/// List all topics with their version availability
#[must_use]
pub fn list_all_topics() -> &'static [TopicInfo] {
    TOPICS
}

/// Search documentation topics by keyword for a given version
///
/// Returns topics where the name or description contains the query
#[must_use]
pub fn search_topics(query: &str, version: TailwindVersion) -> Vec<(&'static str, &'static str)> {
    let query_lower = query.to_lowercase();
    TOPICS
        .iter()
        .filter(|t| match version {
            TailwindVersion::V3 => t.v3,
            TailwindVersion::V4 => t.v4,
        })
        .filter(|t| {
            t.name.to_lowercase().contains(&query_lower)
                || t.description.to_lowercase().contains(&query_lower)
        })
        .map(|t| (t.name, t.description))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that require embedded data
    #[cfg(feature = "embedded-data")]
    mod embedded_tests {
        use super::*;

        #[test]
        fn test_get_docs_v4() {
            let content = get_docs("flexbox", TailwindVersion::V4);
            assert!(content.is_some());
            let text = content.unwrap();
            assert!(text.contains("Flexbox"));
            // v4-specific: should mention CSS variables or safe alignment
            assert!(text.contains("v4") || text.contains("safe") || text.contains("var(--"));
        }

        #[test]
        fn test_get_docs_v3() {
            let content = get_docs("flexbox", TailwindVersion::V3);
            assert!(content.is_some());
            let text = content.unwrap();
            assert!(text.contains("Flexbox") || text.contains("flex"));
        }
    }

    // Tests that work without embedded data
    #[test]
    fn test_list_topics_v4() {
        let topics = list_topics(TailwindVersion::V4);
        assert!(!topics.is_empty());
        assert!(topics.iter().any(|(name, _)| *name == "flexbox"));
        assert!(topics.iter().any(|(name, _)| *name == "grid"));
        // shared topics
        assert!(topics.iter().any(|(name, _)| *name == "forms"));
        assert!(topics.iter().any(|(name, _)| *name == "prose"));
        // v4-only topics
        assert!(topics.iter().any(|(name, _)| *name == "v4-changes"));
    }

    #[test]
    fn test_list_topics_v3() {
        let topics = list_topics(TailwindVersion::V3);
        assert!(!topics.is_empty());
        assert!(topics.iter().any(|(name, _)| *name == "flexbox"));
        // forms and prose are available in both versions
        assert!(topics.iter().any(|(name, _)| *name == "forms"));
        assert!(topics.iter().any(|(name, _)| *name == "prose"));
        // v4-only topics should NOT be in v3
        assert!(!topics.iter().any(|(name, _)| *name == "v4-changes"));
    }

    #[test]
    fn test_search_topics() {
        let results = search_topics("flex", TailwindVersion::V4);
        assert!(!results.is_empty());
        assert!(results.iter().any(|(name, _)| *name == "flexbox"));
    }

    #[test]
    fn test_get_docs_nonexistent() {
        assert!(get_docs("nonexistent", TailwindVersion::V4).is_none());
        assert!(get_docs("nonexistent", TailwindVersion::V3).is_none());
    }

    #[test]
    fn test_version_parse() {
        assert_eq!(TailwindVersion::parse("v3"), Some(TailwindVersion::V3));
        assert_eq!(TailwindVersion::parse("v4"), Some(TailwindVersion::V4));
        assert_eq!(TailwindVersion::parse("3"), Some(TailwindVersion::V3));
        assert_eq!(TailwindVersion::parse("4"), Some(TailwindVersion::V4));
        assert_eq!(TailwindVersion::parse("V4"), Some(TailwindVersion::V4));
        assert_eq!(TailwindVersion::parse("invalid"), None);
    }

    #[test]
    fn test_version_default() {
        assert_eq!(TailwindVersion::default(), TailwindVersion::V4);
    }
}
