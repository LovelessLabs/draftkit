//! Pattern matching and recipe generation.
//!
//! The pattern matcher suggests appropriate components based on
//! pattern definitions and generates complete page recipes.

use std::collections::HashMap;

use crate::components::StyleProfile;
use crate::intelligence::{CoherenceChecker, PageCoherence};
use crate::patterns::{Pattern, SectionSpec, StyleConstraints, VariantSpec};

/// A suggestion for the next section to add.
#[derive(Debug, Clone)]
pub struct SectionSuggestion {
    /// Section type (e.g., "hero", "features")
    pub section_type: String,
    /// Recommended component variants
    pub variants: Vec<VariantSpec>,
    /// Why this section is suggested
    pub reason: String,
    /// Priority score (higher = more important)
    pub priority: f32,
    /// Whether this section is required by the pattern
    pub required: bool,
}

/// A complete page recipe with all sections and validation.
#[derive(Debug, Clone)]
pub struct Recipe {
    /// Pattern this recipe is based on
    pub pattern_id: String,
    /// Ordered list of section selections
    pub sections: Vec<RecipeSection>,
    /// Coherence validation result
    pub coherence: PageCoherence,
    /// All dependencies needed (npm packages, etc.)
    pub dependencies: Vec<String>,
}

impl Recipe {
    /// Whether this recipe passes coherence validation.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.coherence.valid
    }
}

/// A section in a recipe with its selected variant.
#[derive(Debug, Clone)]
pub struct RecipeSection {
    /// Section type
    pub section_type: String,
    /// Selected component variant ID
    pub variant_id: String,
    /// Position in page
    pub position: u8,
    /// Data slots for this section
    pub slots: HashMap<String, String>,
}

/// Options for recipe generation.
#[derive(Debug, Clone, Default)]
pub struct RecipeOptions {
    /// Section type to emphasize (use recommended variant)
    pub emphasis: Option<String>,
    /// Preferred style (affects variant selection)
    pub style_preference: Option<StylePreference>,
    /// Component intelligence data for coherence checking
    pub component_profiles: HashMap<String, StyleProfile>,
}

/// Style preference for variant selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StylePreference {
    /// Prefer minimal, clean components
    Minimal,
    /// Prefer balanced, standard components
    Balanced,
    /// Prefer bold, heavy components
    Bold,
}

/// Pattern matcher for suggesting sections and generating recipes.
#[derive(Debug)]
pub struct PatternMatcher {
    coherence_checker: CoherenceChecker,
}

impl PatternMatcher {
    /// Create a new pattern matcher.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            coherence_checker: CoherenceChecker::new(),
        }
    }

    /// Create a matcher with custom coherence constraints.
    #[must_use]
    pub const fn with_constraints(constraints: StyleConstraints) -> Self {
        Self {
            coherence_checker: CoherenceChecker::with_constraints(constraints),
        }
    }

    /// Suggest the next section(s) to add based on pattern and current state.
    #[must_use]
    pub fn suggest_next_section(
        &self,
        pattern: &Pattern,
        current_sections: &[String],
    ) -> Vec<SectionSuggestion> {
        let mut suggestions = Vec::new();

        // Find which required sections are missing
        for section_spec in &pattern.sections {
            let section_present = current_sections
                .iter()
                .any(|s| s == &section_spec.section_type);

            if section_spec.required && !section_present {
                suggestions.push(SectionSuggestion {
                    section_type: section_spec.section_type.clone(),
                    variants: section_spec.variants.clone(),
                    reason: "Required by pattern".to_string(),
                    priority: 1.0 - (section_spec.position as f32 / 10.0), // Earlier = higher priority
                    required: true,
                });
            }
        }

        // Suggest based on typical flow (what follows the last section)
        if let Some(last) = current_sections.last()
            && let Some(next_types) = self.typical_next_sections(last)
        {
            for next_type in next_types {
                // Don't suggest if already present
                if current_sections.contains(&next_type) {
                    continue;
                }

                // Find the spec for this section type
                if let Some(spec) = pattern
                    .sections
                    .iter()
                    .find(|s| s.section_type == next_type)
                {
                    // Don't duplicate required suggestions
                    if !suggestions.iter().any(|s| s.section_type == next_type) {
                        suggestions.push(SectionSuggestion {
                            section_type: next_type.clone(),
                            variants: spec.variants.clone(),
                            reason: format!("Commonly follows {}", last),
                            priority: 0.7,
                            required: spec.required,
                        });
                    }
                }
            }
        }

        // Sort by priority (highest first)
        suggestions.sort_by(|a, b| {
            b.priority
                .partial_cmp(&a.priority)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        suggestions
    }

    /// Generate a complete recipe from a pattern.
    #[must_use]
    pub fn generate_recipe(&self, pattern: &Pattern, opts: &RecipeOptions) -> Recipe {
        let mut sections = Vec::new();
        let dependencies = Vec::new();

        // Process each section in the pattern
        for section_spec in &pattern.sections {
            // Select variant based on options
            let variant = self.select_variant(section_spec, opts);

            sections.push(RecipeSection {
                section_type: section_spec.section_type.clone(),
                variant_id: variant.id.clone(),
                position: section_spec.position,
                slots: self.default_slots(section_spec),
            });

            // Collect dependencies (would come from component metadata in real impl)
            // For now, we track the variant IDs
        }

        // Sort by position
        sections.sort_by_key(|s| s.position);

        // Validate coherence if we have profile data
        let coherence = if opts.component_profiles.is_empty() {
            // No profile data - assume valid
            PageCoherence {
                score: 1.0,
                issues: vec![],
                valid: true,
                pairwise_scores: vec![],
            }
        } else {
            // Build profile list for validation
            let profiles: Vec<(&str, &StyleProfile)> = sections
                .iter()
                .filter_map(|s| {
                    opts.component_profiles
                        .get(&s.variant_id)
                        .map(|p| (s.variant_id.as_str(), p))
                })
                .collect();

            self.coherence_checker
                .check_page_coherence_with_constraints(&profiles, &pattern.style_constraints)
        };

        Recipe {
            pattern_id: pattern.id.clone(),
            sections,
            coherence,
            dependencies,
        }
    }

    /// Select the best variant for a section based on options.
    fn select_variant<'a>(
        &self,
        section: &'a SectionSpec,
        opts: &RecipeOptions,
    ) -> &'a VariantSpec {
        // If this section is emphasized, use recommended variant
        if opts.emphasis.as_ref() == Some(&section.section_type)
            && let Some(recommended) = section.variants.iter().find(|v| v.recommended)
        {
            return recommended;
        }

        // Style preference affects selection
        match opts.style_preference {
            Some(StylePreference::Minimal) => {
                // Prefer lower-weight variants (assuming first variants are simpler)
                section.variants.first()
            }
            Some(StylePreference::Bold) => {
                // Prefer higher-weight variants
                section.variants.last()
            }
            Some(StylePreference::Balanced) | None => {
                // Use highest-weighted variant
                section.variants.iter().max_by(|a, b| {
                    a.weight
                        .partial_cmp(&b.weight)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            }
        }
        .unwrap_or_else(|| {
            section
                .variants
                .first()
                .expect("Section must have at least one variant")
        })
    }

    /// Generate default slot values for a section.
    fn default_slots(&self, section: &SectionSpec) -> HashMap<String, String> {
        let mut slots = HashMap::new();

        for slot in &section.slots {
            if let Some(default) = &slot.default {
                slots.insert(slot.name.clone(), default.clone());
            } else if let Some(example) = &slot.example {
                slots.insert(slot.name.clone(), example.clone());
            }
        }

        slots
    }

    /// Get typical sections that follow a given section type.
    fn typical_next_sections(&self, section_type: &str) -> Option<Vec<String>> {
        // Common page flow patterns
        match section_type {
            "header" => Some(vec!["hero".to_string()]),
            "hero" => Some(vec!["social-proof".to_string(), "features".to_string()]),
            "social-proof" => Some(vec!["features".to_string()]),
            "features" => Some(vec!["pricing".to_string(), "testimonial".to_string()]),
            "pricing" => Some(vec!["faq".to_string(), "cta".to_string()]),
            "testimonial" => Some(vec!["cta".to_string(), "pricing".to_string()]),
            "faq" => Some(vec!["cta".to_string()]),
            "cta" => Some(vec!["footer".to_string()]),
            _ => None,
        }
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::PatternLoader;

    #[test]
    fn suggest_required_sections_first() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        let matcher = PatternMatcher::new();

        // Empty page - should suggest header first (required, position 0)
        let suggestions = matcher.suggest_next_section(&saas.pattern, &[]);

        assert!(!suggestions.is_empty());
        assert!(suggestions[0].required);
        // First suggestion should be early in the page
        assert!(suggestions[0].priority > 0.5);
    }

    #[test]
    fn suggest_based_on_flow() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        let matcher = PatternMatcher::new();

        // Page has header - should suggest hero next
        let suggestions = matcher.suggest_next_section(&saas.pattern, &["header".to_string()]);

        let has_hero_suggestion = suggestions.iter().any(|s| s.section_type == "hero");
        assert!(has_hero_suggestion, "Should suggest hero after header");
    }

    #[test]
    fn generate_complete_recipe() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        let matcher = PatternMatcher::new();

        let recipe = matcher.generate_recipe(&saas.pattern, &RecipeOptions::default());

        // Should have all required sections
        assert!(!recipe.sections.is_empty());

        // Sections should be ordered by position
        let positions: Vec<u8> = recipe.sections.iter().map(|s| s.position).collect();
        let mut sorted = positions.clone();
        sorted.sort();
        assert_eq!(positions, sorted, "Sections should be ordered by position");
    }

    #[test]
    fn recipe_respects_emphasis() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        let matcher = PatternMatcher::new();

        let opts = RecipeOptions {
            emphasis: Some("pricing".to_string()),
            ..Default::default()
        };

        let recipe = matcher.generate_recipe(&saas.pattern, &opts);

        // Find pricing section
        let pricing = recipe
            .sections
            .iter()
            .find(|s| s.section_type == "pricing")
            .expect("Should have pricing section");

        // Should use recommended variant (pricing-three-tier)
        assert_eq!(pricing.variant_id, "pricing-three-tier");
    }

    #[test]
    fn recipe_has_default_slots() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        let matcher = PatternMatcher::new();

        let recipe = matcher.generate_recipe(&saas.pattern, &RecipeOptions::default());

        // Header should have default cta_text
        let header = recipe
            .sections
            .iter()
            .find(|s| s.section_type == "header")
            .expect("Should have header");

        assert!(header.slots.contains_key("cta_text"));
    }

    #[test]
    fn no_duplicate_suggestions() {
        let loader = PatternLoader::builtin_only();
        let saas = loader.get("saas-landing").unwrap();
        let matcher = PatternMatcher::new();

        let suggestions = matcher.suggest_next_section(&saas.pattern, &[]);

        // No duplicate section types
        let types: Vec<_> = suggestions.iter().map(|s| &s.section_type).collect();
        let unique: std::collections::HashSet<_> = types.iter().collect();
        assert_eq!(
            types.len(),
            unique.len(),
            "Should not have duplicate suggestions"
        );
    }
}
