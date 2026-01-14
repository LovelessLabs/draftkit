//! Component type definitions.

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// UI framework for component code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Framework {
    Html,
    React,
    Vue,
}

impl Framework {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::React => "react",
            Self::Vue => "vue",
        }
    }

    /// Get the NDJSON filename for this framework (v4 only)
    #[must_use]
    pub const fn ndjson_filename(&self) -> &'static str {
        match self {
            Self::Html => "html-v4.ndjson",
            Self::React => "react-v4.ndjson",
            Self::Vue => "vue-v4.ndjson",
        }
    }

    /// Parse framework from string
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "html" => Some(Self::Html),
            "react" => Some(Self::React),
            "vue" => Some(Self::Vue),
            _ => None,
        }
    }
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Tailwind CSS version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum TailwindVersion {
    #[serde(rename = "3")]
    V3,
    #[default]
    #[serde(rename = "4")]
    V4,
}

impl TailwindVersion {
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::V3 => 3,
            Self::V4 => 4,
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::V3 => "v3",
            Self::V4 => "v4",
        }
    }

    /// Parse version from string (e.g., "v3", "v4", "3", "4")
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().trim_start_matches('v') {
            "3" => Some(Self::V3),
            "4" => Some(Self::V4),
            _ => None,
        }
    }
}

impl std::fmt::Display for TailwindVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Theme mode for component styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Light,
    Dark,
    System,
    #[serde(rename = "none")]
    None, // For ecommerce components
}

impl Mode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
            Self::None => "none",
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Lightweight component metadata (stored in index)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMeta {
    /// Unique component identifier (UUID from TailwindPlus)
    pub id: String,
    /// Human-readable component name
    pub name: String,
    /// Category path: ["Application UI", "Forms", "Input Groups"]
    pub path: Vec<String>,
    /// Available frameworks for this component
    pub frameworks: Vec<Framework>,
    /// Whether component supports dark mode
    pub has_dark_mode: bool,
    /// Chunk file containing this component's code
    pub chunk_file: String,
}

/// Full component snippet with code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    /// Component code (HTML/JSX/Vue template)
    pub code: String,
    /// Programming language (html, jsx)
    pub language: String,
    /// Framework (html, react, vue)
    pub framework: Framework,
    /// Tailwind CSS version
    pub version: TailwindVersion,
    /// Theme mode
    pub mode: Mode,
    /// Whether this component supports dark mode
    #[serde(rename = "supportsDarkMode")]
    pub supports_dark_mode: bool,
    /// Preview HTML for rendering
    #[serde(default)]
    pub preview: Option<String>,
}

/// Component with all its snippets (stored in chunk files)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Component name
    pub name: String,
    /// All code snippets for this component
    pub snippets: Vec<Snippet>,
}

/// Search result returned to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub path: Vec<String>,
    pub frameworks: Vec<Framework>,
    pub has_dark_mode: bool,
}

impl From<&ComponentMeta> for SearchResult {
    fn from(meta: &ComponentMeta) -> Self {
        Self {
            id: meta.id.clone(),
            name: meta.name.clone(),
            path: meta.path.clone(),
            frameworks: meta.frameworks.clone(),
            has_dark_mode: meta.has_dark_mode,
        }
    }
}

/// Component code response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentResponse {
    pub id: String,
    pub name: String,
    pub path: Vec<String>,
    pub code: String,
    pub language: String,
    pub framework: Framework,
    pub version: TailwindVersion,
    pub mode: Mode,
    pub supports_dark_mode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

/// Category tree node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryNode {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_count: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framework_as_str() {
        assert_eq!(Framework::Html.as_str(), "html");
        assert_eq!(Framework::React.as_str(), "react");
        assert_eq!(Framework::Vue.as_str(), "vue");
    }

    #[test]
    fn framework_display() {
        assert_eq!(format!("{}", Framework::Html), "html");
        assert_eq!(format!("{}", Framework::React), "react");
        assert_eq!(format!("{}", Framework::Vue), "vue");
    }

    #[test]
    fn tailwind_version_as_u8() {
        assert_eq!(TailwindVersion::V3.as_u8(), 3);
        assert_eq!(TailwindVersion::V4.as_u8(), 4);
    }

    #[test]
    fn tailwind_version_display() {
        assert_eq!(format!("{}", TailwindVersion::V3), "v3");
        assert_eq!(format!("{}", TailwindVersion::V4), "v4");
    }

    #[test]
    fn mode_as_str() {
        assert_eq!(Mode::Light.as_str(), "light");
        assert_eq!(Mode::Dark.as_str(), "dark");
        assert_eq!(Mode::System.as_str(), "system");
        assert_eq!(Mode::None.as_str(), "none");
    }

    #[test]
    fn mode_display() {
        assert_eq!(format!("{}", Mode::Light), "light");
        assert_eq!(format!("{}", Mode::Dark), "dark");
        assert_eq!(format!("{}", Mode::System), "system");
        assert_eq!(format!("{}", Mode::None), "none");
    }

    #[test]
    fn search_result_from_component_meta() {
        let meta = ComponentMeta {
            id: "test-id".to_string(),
            name: "Test Component".to_string(),
            path: vec!["UI".to_string(), "Forms".to_string()],
            frameworks: vec![Framework::React, Framework::Vue],
            has_dark_mode: true,
            chunk_file: "chunk-0.json".to_string(),
        };

        let result = SearchResult::from(&meta);

        assert_eq!(result.id, "test-id");
        assert_eq!(result.name, "Test Component");
        assert_eq!(result.path, vec!["UI", "Forms"]);
        assert_eq!(result.frameworks, vec![Framework::React, Framework::Vue]);
        assert!(result.has_dark_mode);
    }

    #[test]
    fn framework_serde_roundtrip() {
        let json = serde_json::to_string(&Framework::React).unwrap();
        assert_eq!(json, "\"react\"");

        let parsed: Framework = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Framework::React);
    }

    #[test]
    fn mode_serde_roundtrip() {
        let json = serde_json::to_string(&Mode::System).unwrap();
        assert_eq!(json, "\"system\"");

        let parsed: Mode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Mode::System);
    }

    #[test]
    fn tailwind_version_serde_roundtrip() {
        let json = serde_json::to_string(&TailwindVersion::V4).unwrap();
        assert_eq!(json, "\"4\"");

        let parsed: TailwindVersion = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, TailwindVersion::V4);
    }
}
