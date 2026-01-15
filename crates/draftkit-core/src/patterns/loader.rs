//! Pattern loading with directory precedence.
//!
//! Patterns are loaded from multiple directories with later sources
//! overriding earlier ones:
//!
//! 1. Built-in patterns (embedded in binary)
//! 2. User patterns (~/.config/draftkit/patterns/)
//! 3. Project patterns (./.draftkit/patterns/)

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::patterns::schema::{Pattern, PatternFile};

/// Error type for pattern loading operations.
#[derive(Debug, thiserror::Error)]
pub enum PatternError {
    #[error("Pattern not found: {0}")]
    NotFound(String),

    #[error("Failed to read pattern file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse pattern TOML: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Pattern validation failed: {0}")]
    ValidationError(String),
}

/// Pattern source for tracking where a pattern came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternSource {
    /// Embedded in binary
    BuiltIn,
    /// User's config directory
    User,
    /// Project-local .draftkit directory
    Project,
}

/// A loaded pattern with source tracking.
#[derive(Debug, Clone)]
pub struct LoadedPattern {
    /// The pattern definition
    pub pattern: Pattern,
    /// Where this pattern was loaded from
    pub source: PatternSource,
    /// File path (if loaded from disk)
    pub path: Option<PathBuf>,
}

/// Pattern loader with directory precedence.
#[derive(Debug)]
pub struct PatternLoader {
    /// Loaded patterns by ID (later sources override earlier)
    patterns: HashMap<String, LoadedPattern>,
}

impl PatternLoader {
    /// Create a new pattern loader and load from all sources.
    ///
    /// # Errors
    /// Returns error if pattern directories cannot be read.
    pub fn new() -> Result<Self, PatternError> {
        let mut loader = Self {
            patterns: HashMap::new(),
        };

        // Load in precedence order (later overrides earlier)
        loader.load_builtin_patterns();
        loader.load_user_patterns()?;
        loader.load_project_patterns()?;

        Ok(loader)
    }

    /// Create a loader with only built-in patterns.
    #[must_use]
    pub fn builtin_only() -> Self {
        let mut loader = Self {
            patterns: HashMap::new(),
        };
        loader.load_builtin_patterns();
        loader
    }

    /// Get a pattern by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&LoadedPattern> {
        self.patterns.get(id)
    }

    /// List all available pattern IDs.
    #[must_use]
    pub fn list_ids(&self) -> Vec<&str> {
        self.patterns.keys().map(String::as_str).collect()
    }

    /// List all patterns with metadata.
    #[must_use]
    pub fn list_all(&self) -> Vec<&LoadedPattern> {
        self.patterns.values().collect()
    }

    /// Load built-in patterns embedded in the binary.
    fn load_builtin_patterns(&mut self) {
        // Built-in patterns are defined inline for now
        // In the future, these could be embedded via include_str!
        for pattern in builtin_patterns() {
            self.patterns.insert(
                pattern.id.clone(),
                LoadedPattern {
                    pattern,
                    source: PatternSource::BuiltIn,
                    path: None,
                },
            );
        }
    }

    /// Load patterns from user config directory.
    fn load_user_patterns(&mut self) -> Result<(), PatternError> {
        if let Some(config_dir) = dirs::config_dir() {
            let patterns_dir = config_dir.join("draftkit").join("patterns");
            if patterns_dir.exists() {
                self.load_from_directory(&patterns_dir, PatternSource::User)?;
            }
        }
        Ok(())
    }

    /// Load patterns from project-local .draftkit directory.
    fn load_project_patterns(&mut self) -> Result<(), PatternError> {
        let project_dir = PathBuf::from(".draftkit").join("patterns");
        if project_dir.exists() {
            self.load_from_directory(&project_dir, PatternSource::Project)?;
        }
        Ok(())
    }

    /// Load all .toml files from a directory.
    fn load_from_directory(&mut self, dir: &Path, source: PatternSource) -> Result<(), PatternError> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "toml") {
                match self.load_pattern_file(&path, source) {
                    Ok(loaded) => {
                        self.patterns.insert(loaded.pattern.id.clone(), loaded);
                    }
                    Err(e) => {
                        // Log warning but continue loading other patterns
                        eprintln!("Warning: Failed to load pattern {}: {e}", path.display());
                    }
                }
            }
        }
        Ok(())
    }

    /// Load a single pattern file.
    fn load_pattern_file(&self, path: &Path, source: PatternSource) -> Result<LoadedPattern, PatternError> {
        let content = std::fs::read_to_string(path)?;
        let pattern_file: PatternFile = toml::from_str(&content)?;

        Ok(LoadedPattern {
            pattern: pattern_file.pattern,
            source,
            path: Some(path.to_path_buf()),
        })
    }

    /// Reload patterns from all sources.
    ///
    /// # Errors
    /// Returns error if pattern directories cannot be read.
    pub fn reload(&mut self) -> Result<(), PatternError> {
        self.patterns.clear();
        self.load_builtin_patterns();
        self.load_user_patterns()?;
        self.load_project_patterns()?;
        Ok(())
    }
}

impl Default for PatternLoader {
    fn default() -> Self {
        Self::builtin_only()
    }
}

/// Generate built-in patterns.
///
/// These are the core patterns that ship with draftkit.
fn builtin_patterns() -> Vec<Pattern> {
    vec![
        saas_landing_pattern(),
        marketing_pattern(),
        portfolio_pattern(),
    ]
}

/// SaaS landing page pattern.
fn saas_landing_pattern() -> Pattern {
    use crate::patterns::schema::*;

    Pattern {
        id: "saas-landing".to_string(),
        name: "SaaS Landing Page".to_string(),
        description: "Standard SaaS product landing page with pricing focus".to_string(),
        tags: vec!["landing".into(), "saas".into(), "b2b".into(), "marketing".into()],
        author: "draftkit-core".to_string(),
        version: "1.0.0".to_string(),
        style_constraints: StyleConstraints {
            visual_weight_variance: 0.3,
            spacing_density_variance: 0.2,
            formality_range: (0.6, 1.0),
            color_intensity_range: (0.3, 0.7),
        },
        sections: vec![
            SectionSpec {
                section_type: "header".into(),
                required: true,
                position: 0,
                count: None,
                variants: vec![
                    VariantSpec { id: "header-simple-centered".into(), weight: 0.4, recommended: false },
                    VariantSpec { id: "header-with-cta".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![
                    SlotSpec { name: "logo".into(), slot_type: SlotType::Image, required: true, ..Default::default() },
                    SlotSpec { name: "nav_items".into(), slot_type: SlotType::Array, required: false, default: Some("Features, Pricing, About".into()), ..Default::default() },
                    SlotSpec { name: "cta_text".into(), slot_type: SlotType::String, required: false, default: Some("Get Started".into()), ..Default::default() },
                ],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "hero".into(),
                required: true,
                position: 1,
                count: None,
                variants: vec![
                    VariantSpec { id: "hero-split-screenshot".into(), weight: 0.5, recommended: true },
                    VariantSpec { id: "hero-centered-cta".into(), weight: 0.3, recommended: false },
                    VariantSpec { id: "hero-video-background".into(), weight: 0.2, recommended: false },
                ],
                slots: vec![
                    SlotSpec { name: "headline".into(), slot_type: SlotType::String, required: true, example: Some("Ship faster with AI".into()), ..Default::default() },
                    SlotSpec { name: "subheadline".into(), slot_type: SlotType::String, required: false, ..Default::default() },
                    SlotSpec { name: "cta_primary".into(), slot_type: SlotType::String, required: false, default: Some("Start free trial".into()), ..Default::default() },
                ],
                constraints: vec!["must_include_cta".into()],
            },
            SectionSpec {
                section_type: "features".into(),
                required: true,
                position: 2,
                count: Some(RepeatCount { min: 1, max: 3 }),
                variants: vec![
                    VariantSpec { id: "feature-grid-icons".into(), weight: 0.5, recommended: true },
                    VariantSpec { id: "feature-alternating-screenshots".into(), weight: 0.4, recommended: false },
                ],
                slots: vec![
                    SlotSpec { name: "heading".into(), slot_type: SlotType::String, required: false, example: Some("Everything you need".into()), ..Default::default() },
                    SlotSpec { name: "features".into(), slot_type: SlotType::Array, required: true, min: Some(3), max: Some(6), schema: Some("feature_item".into()), ..Default::default() },
                ],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "pricing".into(),
                required: true,
                position: 3,
                count: None,
                variants: vec![
                    VariantSpec { id: "pricing-three-tier".into(), weight: 0.6, recommended: true },
                    VariantSpec { id: "pricing-four-tier-enterprise".into(), weight: 0.3, recommended: false },
                ],
                slots: vec![
                    SlotSpec { name: "billing_period".into(), slot_type: SlotType::Enum, required: false, values: Some(vec!["monthly".into(), "annual".into(), "both".into()]), default: Some("both".into()), ..Default::default() },
                ],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "cta".into(),
                required: true,
                position: 4,
                count: None,
                variants: vec![
                    VariantSpec { id: "cta-simple-centered".into(), weight: 0.5, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "footer".into(),
                required: true,
                position: 5,
                count: None,
                variants: vec![
                    VariantSpec { id: "footer-four-column".into(), weight: 0.7, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
        ],
        schemas: HashMap::from([
            ("feature_item".to_string(), SlotSchema {
                fields: HashMap::from([
                    ("icon".to_string(), FieldSpec { field_type: SlotType::String, required: true, default: None, min: None }),
                    ("title".to_string(), FieldSpec { field_type: SlotType::String, required: true, default: None, min: None }),
                    ("description".to_string(), FieldSpec { field_type: SlotType::String, required: true, default: None, min: None }),
                ]),
            }),
        ]),
        rules: CompositionRules::default(),
    }
}

/// Marketing/product page pattern.
fn marketing_pattern() -> Pattern {
    use crate::patterns::schema::*;

    Pattern {
        id: "marketing".to_string(),
        name: "Marketing Product Page".to_string(),
        description: "Product marketing page with feature showcase".to_string(),
        tags: vec!["marketing".into(), "product".into(), "showcase".into()],
        author: "draftkit-core".to_string(),
        version: "1.0.0".to_string(),
        style_constraints: StyleConstraints {
            visual_weight_variance: 0.4,
            spacing_density_variance: 0.3,
            formality_range: (0.3, 0.9),
            color_intensity_range: (0.4, 0.8),
        },
        sections: vec![
            SectionSpec {
                section_type: "header".into(),
                required: true,
                position: 0,
                count: None,
                variants: vec![
                    VariantSpec { id: "header-simple-centered".into(), weight: 0.5, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "hero".into(),
                required: true,
                position: 1,
                count: None,
                variants: vec![
                    VariantSpec { id: "hero-centered-cta".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "features".into(),
                required: true,
                position: 2,
                count: Some(RepeatCount { min: 1, max: 4 }),
                variants: vec![
                    VariantSpec { id: "feature-alternating-screenshots".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "testimonial".into(),
                required: false,
                position: 3,
                count: None,
                variants: vec![
                    VariantSpec { id: "testimonial-grid".into(), weight: 0.5, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "cta".into(),
                required: true,
                position: 4,
                count: None,
                variants: vec![
                    VariantSpec { id: "cta-simple-centered".into(), weight: 0.5, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "footer".into(),
                required: true,
                position: 5,
                count: None,
                variants: vec![
                    VariantSpec { id: "footer-simple".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
        ],
        schemas: HashMap::new(),
        rules: CompositionRules::default(),
    }
}

/// Portfolio/personal page pattern.
fn portfolio_pattern() -> Pattern {
    use crate::patterns::schema::*;

    Pattern {
        id: "portfolio".to_string(),
        name: "Portfolio Page".to_string(),
        description: "Personal or agency portfolio showcase".to_string(),
        tags: vec!["portfolio".into(), "personal".into(), "agency".into()],
        author: "draftkit-core".to_string(),
        version: "1.0.0".to_string(),
        style_constraints: StyleConstraints {
            visual_weight_variance: 0.5,
            spacing_density_variance: 0.4,
            formality_range: (0.2, 0.8),
            color_intensity_range: (0.2, 0.9),
        },
        sections: vec![
            SectionSpec {
                section_type: "header".into(),
                required: true,
                position: 0,
                count: None,
                variants: vec![
                    VariantSpec { id: "header-minimal".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "hero".into(),
                required: true,
                position: 1,
                count: None,
                variants: vec![
                    VariantSpec { id: "hero-personal-intro".into(), weight: 0.5, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "content".into(),
                required: true,
                position: 2,
                count: Some(RepeatCount { min: 1, max: 5 }),
                variants: vec![
                    VariantSpec { id: "portfolio-grid".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
            SectionSpec {
                section_type: "footer".into(),
                required: true,
                position: 3,
                count: None,
                variants: vec![
                    VariantSpec { id: "footer-minimal".into(), weight: 0.6, recommended: true },
                ],
                slots: vec![],
                constraints: vec![],
            },
        ],
        schemas: HashMap::new(),
        rules: CompositionRules::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_patterns_load() {
        let loader = PatternLoader::builtin_only();
        assert!(loader.get("saas-landing").is_some());
        assert!(loader.get("marketing").is_some());
        assert!(loader.get("portfolio").is_some());
    }

    #[test]
    fn list_pattern_ids() {
        let loader = PatternLoader::builtin_only();
        let ids = loader.list_ids();
        assert!(ids.contains(&"saas-landing"));
        assert!(ids.contains(&"marketing"));
        assert!(ids.contains(&"portfolio"));
    }

    #[test]
    fn saas_pattern_structure() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();

        assert_eq!(saas.pattern.sections.len(), 6);
        assert_eq!(saas.pattern.sections[0].section_type, "header");
        assert_eq!(saas.pattern.sections[1].section_type, "hero");
        assert!(saas.pattern.sections[0].required);
    }

    #[test]
    fn pattern_source_tracking() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        assert_eq!(saas.source, PatternSource::BuiltIn);
        assert!(saas.path.is_none());
    }
}
