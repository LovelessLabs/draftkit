//! Style coherence validation engine.
//!
//! The coherence checker validates that components work well together
//! visually. It uses deterministic rules based on `StyleProfile` data
//! to detect jarring combinations.

use crate::components::StyleProfile;
use crate::patterns::StyleConstraints;

/// Result of checking compatibility between two components.
#[derive(Debug, Clone)]
pub struct CompatibilityScore {
    /// Overall score (0.0 = incompatible, 1.0 = perfect match)
    pub score: f32,
    /// Specific issues found
    pub issues: Vec<CoherenceIssue>,
    /// Suggestions for improving compatibility
    pub suggestions: Vec<String>,
}

impl CompatibilityScore {
    /// Whether this score indicates acceptable compatibility.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.score >= 0.7
    }
}

/// A specific coherence issue detected.
#[derive(Debug, Clone)]
pub struct CoherenceIssue {
    /// Issue category
    pub category: IssueCategory,
    /// Human-readable description
    pub message: String,
    /// How much this impacts the score (0.0-1.0)
    pub severity: f32,
}

/// Categories of coherence issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueCategory {
    /// Visual weight mismatch (heavy vs light)
    VisualWeight,
    /// Spacing density mismatch (tight vs spacious)
    SpacingDensity,
    /// Typography scale mismatch
    Typography,
    /// Formality mismatch (playful vs corporate)
    Formality,
    /// Color intensity mismatch
    ColorIntensity,
}

impl IssueCategory {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::VisualWeight => "visual_weight",
            Self::SpacingDensity => "spacing_density",
            Self::Typography => "typography",
            Self::Formality => "formality",
            Self::ColorIntensity => "color_intensity",
        }
    }
}

/// Result of checking an entire page's coherence.
#[derive(Debug, Clone)]
pub struct PageCoherence {
    /// Overall page score (0.0-1.0)
    pub score: f32,
    /// All issues found across the page
    pub issues: Vec<CoherenceIssue>,
    /// Whether the page passes validation
    pub valid: bool,
    /// Pairwise scores between adjacent sections
    pub pairwise_scores: Vec<(String, String, f32)>,
}

/// Coherence checker for validating component combinations.
#[derive(Debug, Default)]
pub struct CoherenceChecker {
    /// Default thresholds (can be overridden by pattern constraints)
    default_constraints: StyleConstraints,
}

impl CoherenceChecker {
    /// Create a new coherence checker with default thresholds.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            default_constraints: StyleConstraints {
                visual_weight_variance: 0.3,
                spacing_density_variance: 0.2,
                formality_range: (0.0, 1.0),
                color_intensity_range: (0.0, 1.0),
            },
        }
    }

    /// Create a checker with custom constraints from a pattern.
    #[must_use]
    pub const fn with_constraints(constraints: StyleConstraints) -> Self {
        Self {
            default_constraints: constraints,
        }
    }

    /// Check compatibility between two components.
    #[must_use]
    pub fn check_compatibility(&self, a: &StyleProfile, b: &StyleProfile) -> CompatibilityScore {
        self.check_compatibility_with_constraints(a, b, &self.default_constraints)
    }

    /// Check compatibility with specific constraints.
    #[must_use]
    pub fn check_compatibility_with_constraints(
        &self,
        a: &StyleProfile,
        b: &StyleProfile,
        constraints: &StyleConstraints,
    ) -> CompatibilityScore {
        let mut score = 1.0_f32;
        let mut issues = Vec::new();

        // Visual weight variance
        let weight_diff = (a.visual_weight - b.visual_weight).abs();
        if weight_diff > constraints.visual_weight_variance {
            let severity = (weight_diff - constraints.visual_weight_variance)
                / (1.0 - constraints.visual_weight_variance);
            score -= 0.25 * severity;
            issues.push(CoherenceIssue {
                category: IssueCategory::VisualWeight,
                message: format!(
                    "Visual weight mismatch: {:.1} vs {:.1} (max variance: {:.1})",
                    a.visual_weight, b.visual_weight, constraints.visual_weight_variance
                ),
                severity,
            });
        }

        // Spacing density variance
        let spacing_diff = (a.spacing_density - b.spacing_density).abs();
        if spacing_diff > constraints.spacing_density_variance {
            let severity = (spacing_diff - constraints.spacing_density_variance)
                / (1.0 - constraints.spacing_density_variance);
            score -= 0.2 * severity;
            issues.push(CoherenceIssue {
                category: IssueCategory::SpacingDensity,
                message: format!(
                    "Spacing density mismatch: {:.1} vs {:.1} (max variance: {:.1})",
                    a.spacing_density, b.spacing_density, constraints.spacing_density_variance
                ),
                severity,
            });
        }

        // Typography scale mismatch
        if a.typography_scale != b.typography_scale {
            score -= 0.1;
            issues.push(CoherenceIssue {
                category: IssueCategory::Typography,
                message: format!(
                    "Typography scale mismatch: {} vs {}",
                    a.typography_scale, b.typography_scale
                ),
                severity: 0.3,
            });
        }

        // Formality range check (both should be within range)
        let (form_min, form_max) = constraints.formality_range;
        let a_in_range = a.formality >= form_min && a.formality <= form_max;
        let b_in_range = b.formality >= form_min && b.formality <= form_max;
        if !a_in_range || !b_in_range {
            score -= 0.15;
            issues.push(CoherenceIssue {
                category: IssueCategory::Formality,
                message: format!(
                    "Formality out of range [{:.1}, {:.1}]: {:.1}, {:.1}",
                    form_min, form_max, a.formality, b.formality
                ),
                severity: 0.4,
            });
        }

        // Color intensity range check
        let (color_min, color_max) = constraints.color_intensity_range;
        let a_color_ok = a.color_intensity >= color_min && a.color_intensity <= color_max;
        let b_color_ok = b.color_intensity >= color_min && b.color_intensity <= color_max;
        if !a_color_ok || !b_color_ok {
            score -= 0.1;
            issues.push(CoherenceIssue {
                category: IssueCategory::ColorIntensity,
                message: format!(
                    "Color intensity out of range [{:.1}, {:.1}]: {:.1}, {:.1}",
                    color_min, color_max, a.color_intensity, b.color_intensity
                ),
                severity: 0.3,
            });
        }

        // Generate suggestions
        let suggestions = self.generate_suggestions(&issues);

        CompatibilityScore {
            score: score.max(0.0),
            issues,
            suggestions,
        }
    }

    /// Check coherence across an entire page of components.
    #[must_use]
    pub fn check_page_coherence(&self, components: &[(&str, &StyleProfile)]) -> PageCoherence {
        self.check_page_coherence_with_constraints(components, &self.default_constraints)
    }

    /// Check page coherence with specific constraints.
    #[must_use]
    pub fn check_page_coherence_with_constraints(
        &self,
        components: &[(&str, &StyleProfile)],
        constraints: &StyleConstraints,
    ) -> PageCoherence {
        if components.is_empty() {
            return PageCoherence {
                score: 1.0,
                issues: vec![],
                valid: true,
                pairwise_scores: vec![],
            };
        }

        if components.len() == 1 {
            // Single component - check against constraints
            let profile = components[0].1;
            let mut issues = vec![];
            let mut score = 1.0_f32;

            let (form_min, form_max) = constraints.formality_range;
            if profile.formality < form_min || profile.formality > form_max {
                score -= 0.15;
                issues.push(CoherenceIssue {
                    category: IssueCategory::Formality,
                    message: format!(
                        "Formality {:.1} outside pattern range [{:.1}, {:.1}]",
                        profile.formality, form_min, form_max
                    ),
                    severity: 0.4,
                });
            }

            return PageCoherence {
                score,
                issues,
                valid: score >= 0.7,
                pairwise_scores: vec![],
            };
        }

        let mut total_score = 1.0_f32;
        let mut all_issues = Vec::new();
        let mut pairwise_scores = Vec::new();

        // Pairwise compatibility between adjacent sections
        for window in components.windows(2) {
            let (id_a, profile_a) = window[0];
            let (id_b, profile_b) = window[1];

            let compat =
                self.check_compatibility_with_constraints(profile_a, profile_b, constraints);
            pairwise_scores.push((id_a.to_string(), id_b.to_string(), compat.score));

            // Weight pairwise scores
            total_score *= compat.score.sqrt(); // Geometric mean influence
            all_issues.extend(compat.issues);
        }

        // Global variance checks across all components
        let profiles: Vec<_> = components.iter().map(|(_, p)| *p).collect();

        // Visual weight variance across entire page
        let weights: Vec<f32> = profiles.iter().map(|p| p.visual_weight).collect();
        let weight_variance = variance(&weights);
        if weight_variance > constraints.visual_weight_variance {
            total_score -= 0.15;
            all_issues.push(CoherenceIssue {
                category: IssueCategory::VisualWeight,
                message: format!(
                    "High visual weight variance across page: {:.2} (max: {:.1})",
                    weight_variance, constraints.visual_weight_variance
                ),
                severity: 0.5,
            });
        }

        // Spacing density variance
        let spacings: Vec<f32> = profiles.iter().map(|p| p.spacing_density).collect();
        let spacing_variance = variance(&spacings);
        if spacing_variance > constraints.spacing_density_variance {
            total_score -= 0.1;
            all_issues.push(CoherenceIssue {
                category: IssueCategory::SpacingDensity,
                message: format!(
                    "High spacing density variance across page: {:.2} (max: {:.1})",
                    spacing_variance, constraints.spacing_density_variance
                ),
                severity: 0.4,
            });
        }

        PageCoherence {
            score: total_score.max(0.0),
            issues: all_issues,
            valid: total_score >= 0.7,
            pairwise_scores,
        }
    }

    /// Generate suggestions for fixing coherence issues.
    fn generate_suggestions(&self, issues: &[CoherenceIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();

        for issue in issues {
            match issue.category {
                IssueCategory::VisualWeight => {
                    suggestions.push(
                        "Consider using components with similar visual weight (shadow/gradient usage)".to_string()
                    );
                }
                IssueCategory::SpacingDensity => {
                    suggestions.push(
                        "Components have different spacing densities - consider more uniform padding/margins".to_string()
                    );
                }
                IssueCategory::Typography => {
                    suggestions.push(
                        "Typography scales differ - consider components with matching text sizes"
                            .to_string(),
                    );
                }
                IssueCategory::Formality => {
                    suggestions.push(
                        "Formality mismatch - mix of playful and corporate styles may feel inconsistent".to_string()
                    );
                }
                IssueCategory::ColorIntensity => {
                    suggestions.push(
                        "Color intensity varies - consider more consistent color saturation across components".to_string()
                    );
                }
            }
        }

        // Deduplicate suggestions
        suggestions.sort();
        suggestions.dedup();
        suggestions
    }
}

/// Calculate variance of a slice of f32 values.
fn variance(values: &[f32]) -> f32 {
    if values.len() <= 1 {
        return 0.0;
    }

    let mean = values.iter().sum::<f32>() / values.len() as f32;
    let sq_diff_sum: f32 = values.iter().map(|v| (v - mean).powi(2)).sum();
    (sq_diff_sum / values.len() as f32).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::TypographyScale;

    fn minimal_profile() -> StyleProfile {
        StyleProfile {
            visual_weight: 0.2,
            formality: 0.7,
            color_intensity: 0.3,
            spacing_density: 0.6,
            typography_scale: TypographyScale::Medium,
        }
    }

    fn heavy_profile() -> StyleProfile {
        // Intentionally extreme to ensure incompatibility with minimal_profile
        StyleProfile {
            visual_weight: 0.95,   // diff=0.75 from minimal's 0.2
            formality: 0.85,       // diff=0.15 from minimal's 0.7
            color_intensity: 0.85, // diff=0.55 from minimal's 0.3
            spacing_density: 0.15, // diff=0.45 from minimal's 0.6
            typography_scale: TypographyScale::Large,
        }
    }

    fn similar_profile() -> StyleProfile {
        StyleProfile {
            visual_weight: 0.3,
            formality: 0.75,
            color_intensity: 0.35,
            spacing_density: 0.55,
            typography_scale: TypographyScale::Medium,
        }
    }

    #[test]
    fn compatible_profiles_score_high() {
        let checker = CoherenceChecker::new();
        let score = checker.check_compatibility(&minimal_profile(), &similar_profile());

        assert!(
            score.score > 0.8,
            "Similar profiles should score > 0.8, got {}",
            score.score
        );
        assert!(score.is_compatible());
        assert!(score.issues.is_empty() || score.issues.iter().all(|i| i.severity < 0.5));
    }

    #[test]
    fn incompatible_profiles_score_low() {
        let checker = CoherenceChecker::new();
        let score = checker.check_compatibility(&minimal_profile(), &heavy_profile());

        assert!(
            score.score < 0.7,
            "Mismatched profiles should score < 0.7, got {}",
            score.score
        );
        assert!(!score.is_compatible());
        assert!(!score.issues.is_empty());
    }

    #[test]
    fn visual_weight_mismatch_detected() {
        let checker = CoherenceChecker::new();
        let score = checker.check_compatibility(&minimal_profile(), &heavy_profile());

        let has_weight_issue = score
            .issues
            .iter()
            .any(|i| i.category == IssueCategory::VisualWeight);
        assert!(has_weight_issue, "Should detect visual weight mismatch");
    }

    #[test]
    fn page_coherence_with_similar_components() {
        let checker = CoherenceChecker::new();
        let minimal = minimal_profile();
        let similar = similar_profile();
        let components = vec![("header", &minimal), ("hero", &similar)];

        let coherence = checker.check_page_coherence(&components);
        assert!(coherence.valid);
        assert!(coherence.score > 0.7);
    }

    #[test]
    fn page_coherence_with_mismatched_components() {
        let checker = CoherenceChecker::new();
        let minimal = minimal_profile();
        let heavy = heavy_profile();
        let components = vec![("header", &minimal), ("hero", &heavy)];

        let coherence = checker.check_page_coherence(&components);
        assert!(!coherence.valid || coherence.score < 0.7);
        assert!(!coherence.issues.is_empty());
    }

    #[test]
    fn empty_page_is_valid() {
        let checker = CoherenceChecker::new();
        let coherence = checker.check_page_coherence(&[]);
        assert!(coherence.valid);
        assert_eq!(coherence.score, 1.0);
    }

    #[test]
    fn single_component_page() {
        let checker = CoherenceChecker::new();
        let profile = minimal_profile();
        let coherence = checker.check_page_coherence(&[("header", &profile)]);
        assert!(coherence.valid);
    }

    #[test]
    fn suggestions_generated_for_issues() {
        let checker = CoherenceChecker::new();
        let score = checker.check_compatibility(&minimal_profile(), &heavy_profile());

        assert!(
            !score.suggestions.is_empty(),
            "Should generate suggestions for issues"
        );
    }

    #[test]
    fn custom_constraints_applied() {
        // Strict constraints that should fail even similar profiles
        let strict = StyleConstraints {
            visual_weight_variance: 0.05,
            spacing_density_variance: 0.05,
            formality_range: (0.7, 0.75),
            color_intensity_range: (0.3, 0.35),
        };

        let checker = CoherenceChecker::with_constraints(strict);
        let score = checker.check_compatibility(&minimal_profile(), &similar_profile());

        // Even similar profiles should fail strict constraints
        assert!(score.score < 1.0);
    }

    #[test]
    fn variance_calculation() {
        assert_eq!(variance(&[]), 0.0);
        assert_eq!(variance(&[5.0]), 0.0);

        // [0, 10] has mean 5, variance = sqrt((25 + 25) / 2) = 5
        let var = variance(&[0.0, 10.0]);
        assert!((var - 5.0).abs() < 0.01);
    }
}
