//! Pattern schema definitions for page archetypes.
//!
//! Patterns are declarative TOML files that describe page structures.
//! They define what sections a page should have, which component variants
//! work well together, and constraints for visual coherence.

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::components::PagePosition;

/// Root structure of a pattern TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PatternFile {
    /// Pattern definition
    pub pattern: Pattern,
}

/// A page pattern defining structure and style constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Pattern {
    /// Unique identifier (e.g., "saas-landing")
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Description of what this pattern is for
    #[serde(default)]
    pub description: String,

    /// Searchable tags (e.g., ["landing", "saas", "b2b"])
    #[serde(default)]
    pub tags: Vec<String>,

    /// Pattern author
    #[serde(default)]
    pub author: String,

    /// Pattern version (semver)
    #[serde(default = "default_version")]
    pub version: String,

    /// Visual coherence constraints
    #[serde(default)]
    pub style_constraints: StyleConstraints,

    /// Page sections in order
    #[serde(default)]
    pub sections: Vec<SectionSpec>,

    /// Slot schemas for complex types
    #[serde(default)]
    pub schemas: std::collections::HashMap<String, SlotSchema>,

    /// Composition rules
    #[serde(default)]
    pub rules: CompositionRules,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

/// Visual constraints for style coherence validation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StyleConstraints {
    /// Maximum allowed variance in visual weight across sections (0.0-1.0)
    #[serde(default = "default_variance")]
    pub visual_weight_variance: f32,

    /// Maximum allowed variance in spacing density (0.0-1.0)
    #[serde(default = "default_variance")]
    pub spacing_density_variance: f32,

    /// Acceptable formality range [min, max] (0.0-1.0)
    #[serde(default = "default_formality_range")]
    pub formality_range: (f32, f32),

    /// Acceptable color intensity range [min, max] (0.0-1.0)
    #[serde(default = "default_color_range")]
    pub color_intensity_range: (f32, f32),
}

const fn default_variance() -> f32 {
    0.3
}

const fn default_formality_range() -> (f32, f32) {
    (0.0, 1.0)
}

const fn default_color_range() -> (f32, f32) {
    (0.0, 1.0)
}

/// Specification for a page section.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SectionSpec {
    /// Section type (maps to PagePosition)
    #[serde(rename = "type")]
    pub section_type: String,

    /// Whether this section is required
    #[serde(default)]
    pub required: bool,

    /// Display order (0 = first)
    #[serde(default)]
    pub position: u8,

    /// How many times this section can repeat
    #[serde(default)]
    pub count: Option<RepeatCount>,

    /// Available component variants for this section
    #[serde(default)]
    pub variants: Vec<VariantSpec>,

    /// Data slots this section expects
    #[serde(default)]
    pub slots: Vec<SlotSpec>,

    /// Section-specific constraints (as string expressions)
    #[serde(default)]
    pub constraints: Vec<String>,
}

impl SectionSpec {
    /// Get the page position for this section type
    #[must_use]
    pub fn page_position(&self) -> Option<PagePosition> {
        match self.section_type.as_str() {
            "header" => Some(PagePosition::Header),
            "hero" => Some(PagePosition::Hero),
            "feature" | "features" => Some(PagePosition::Feature),
            "social-proof" | "social_proof" | "logos" => Some(PagePosition::SocialProof),
            "pricing" => Some(PagePosition::Pricing),
            "testimonial" | "testimonials" => Some(PagePosition::Testimonial),
            "faq" => Some(PagePosition::Faq),
            "cta" => Some(PagePosition::Cta),
            "footer" => Some(PagePosition::Footer),
            "form" => Some(PagePosition::Form),
            "content" => Some(PagePosition::Content),
            _ => Some(PagePosition::Other),
        }
    }
}

/// How many times a section can repeat.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RepeatCount {
    /// Minimum occurrences
    #[serde(default = "default_one")]
    pub min: u8,

    /// Maximum occurrences
    #[serde(default = "default_one")]
    pub max: u8,
}

const fn default_one() -> u8 {
    1
}

/// A component variant option for a section.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct VariantSpec {
    /// Component identifier (matches ComponentMeta.id or component name)
    pub id: String,

    /// Selection weight (higher = more likely to be chosen)
    #[serde(default = "default_weight")]
    pub weight: f32,

    /// Whether this is the recommended variant
    #[serde(default)]
    pub recommended: bool,
}

const fn default_weight() -> f32 {
    0.5
}

/// Data slot specification for content placeholders.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SlotSpec {
    /// Slot name (e.g., "headline", "cta_text")
    pub name: String,

    /// Slot data type
    #[serde(rename = "type")]
    pub slot_type: SlotType,

    /// Whether this slot is required
    #[serde(default)]
    pub required: bool,

    /// Default value (for strings)
    #[serde(default)]
    pub default: Option<String>,

    /// Example value for documentation
    #[serde(default)]
    pub example: Option<String>,

    /// For arrays: minimum items
    #[serde(default)]
    pub min: Option<usize>,

    /// For arrays: maximum items
    #[serde(default)]
    pub max: Option<usize>,

    /// For enums: allowed values
    #[serde(default)]
    pub values: Option<Vec<String>>,

    /// For complex types: reference to schema
    #[serde(default)]
    pub schema: Option<String>,
}

/// Slot data types.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum SlotType {
    #[default]
    String,
    Integer,
    Boolean,
    Image,
    Array,
    Enum,
}

/// Schema for complex slot types (like feature_item, pricing_tier).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SlotSchema {
    /// Field definitions (name -> field spec)
    #[serde(flatten)]
    pub fields: std::collections::HashMap<String, FieldSpec>,
}

/// Field specification within a slot schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FieldSpec {
    /// Field data type
    #[serde(rename = "type")]
    pub field_type: SlotType,

    /// Whether this field is required
    #[serde(default)]
    pub required: bool,

    /// Default value
    #[serde(default)]
    pub default: Option<String>,

    /// For arrays: minimum items
    #[serde(default)]
    pub min: Option<usize>,
}

/// Composition rules for pattern validation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CompositionRules {
    /// Coherence rules (style variance checks)
    #[serde(default)]
    pub coherence: Vec<String>,

    /// Flow rules (section ordering constraints)
    #[serde(default)]
    pub flow: Vec<String>,

    /// Accessibility rules
    #[serde(default)]
    pub accessibility: Vec<String>,

    /// Responsive design rules
    #[serde(default)]
    pub responsive: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_pattern() {
        let toml = r#"
[pattern]
id = "test-pattern"
name = "Test Pattern"
"#;
        let parsed: PatternFile = toml::from_str(toml).unwrap();
        assert_eq!(parsed.pattern.id, "test-pattern");
        assert_eq!(parsed.pattern.name, "Test Pattern");
        assert_eq!(parsed.pattern.version, "1.0.0");
    }

    #[test]
    fn parse_pattern_with_sections() {
        let toml = r#"
[pattern]
id = "saas-landing"
name = "SaaS Landing Page"
tags = ["saas", "landing"]

[[pattern.sections]]
type = "header"
required = true
position = 0

[[pattern.sections.variants]]
id = "header-simple"
weight = 0.5
recommended = true

[[pattern.sections]]
type = "hero"
required = true
position = 1
"#;
        let parsed: PatternFile = toml::from_str(toml).unwrap();
        assert_eq!(parsed.pattern.sections.len(), 2);
        assert_eq!(parsed.pattern.sections[0].section_type, "header");
        assert!(parsed.pattern.sections[0].required);
        assert_eq!(parsed.pattern.sections[0].variants.len(), 1);
        assert!(parsed.pattern.sections[0].variants[0].recommended);
    }

    #[test]
    fn parse_style_constraints() {
        let toml = r#"
[pattern]
id = "minimal"
name = "Minimal"

[pattern.style_constraints]
visual_weight_variance = 0.2
formality_range = [0.7, 1.0]
"#;
        let parsed: PatternFile = toml::from_str(toml).unwrap();
        assert_eq!(parsed.pattern.style_constraints.visual_weight_variance, 0.2);
        assert_eq!(parsed.pattern.style_constraints.formality_range, (0.7, 1.0));
    }

    #[test]
    fn section_page_position_mapping() {
        let section = SectionSpec {
            section_type: "hero".to_string(),
            required: true,
            position: 0,
            count: None,
            variants: vec![],
            slots: vec![],
            constraints: vec![],
        };
        assert_eq!(section.page_position(), Some(PagePosition::Hero));
    }
}
