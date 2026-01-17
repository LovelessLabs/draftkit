//! Component matching for recipes.
//!
//! Maps pattern section types and variant hints to actual components
//! in the Tailwind Plus catalog.

use crate::components::{ComponentReader, Framework, Mode};

/// A recommended component from the catalog.
#[derive(Debug, Clone)]
pub struct ComponentRecommendation {
    /// Component ID (can be passed to `get_component`)
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Category path for context
    pub category: String,
    /// Subcategory (e.g., "Page Sections")
    pub subcategory: String,
    /// Sub-subcategory (e.g., "Hero Sections")
    pub sub_subcategory: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Preview image URL if available
    pub preview_url: Option<String>,
}

/// Component matcher that maps section types to catalog components.
#[derive(Debug, Clone)]
pub struct ComponentMatcher {
    reader: ComponentReader,
    framework: Framework,
}

impl ComponentMatcher {
    /// Create a new component matcher for a specific framework.
    #[must_use]
    pub const fn new(reader: ComponentReader, framework: Framework) -> Self {
        Self { reader, framework }
    }

    /// Create a matcher for React (the most common framework).
    #[must_use]
    pub const fn react() -> Self {
        Self::new(ComponentReader::new(), Framework::React)
    }

    /// Match components for a section type and variant hint.
    ///
    /// Returns up to `limit` recommendations sorted by confidence.
    #[must_use]
    pub fn match_section(
        &self,
        section_type: &str,
        variant_id: &str,
        limit: usize,
    ) -> Vec<ComponentRecommendation> {
        // Get the sub-subcategory for this section type
        let Some(sub_subcategory) = section_to_sub_subcategory(section_type) else {
            // Unknown section type - fall back to keyword search
            return self.search_by_keywords(section_type, variant_id, limit);
        };

        // Extract keywords from variant ID
        let keywords = variant_to_keywords(variant_id);

        // Get all components and filter by sub-subcategory
        let components = self.reader.all(self.framework);
        let mut matches: Vec<ComponentRecommendation> = components
            .iter()
            .filter(|c| {
                c.category == "Marketing"
                    && c.subcategory == "Page Sections"
                    && c.sub_subcategory == sub_subcategory
            })
            .map(|c| {
                let confidence = calculate_confidence(&c.name, &keywords);
                ComponentRecommendation {
                    id: c.id.clone(),
                    name: c.name.clone(),
                    category: c.category.clone(),
                    subcategory: c.subcategory.clone(),
                    sub_subcategory: c.sub_subcategory.clone(),
                    confidence,
                    preview_url: c.preview_url(Mode::Light).map(ToString::to_string),
                }
            })
            .collect();

        // Sort by confidence (highest first)
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches.truncate(limit);
        matches
    }

    /// Fallback search using keywords when section type is unknown.
    fn search_by_keywords(
        &self,
        section_type: &str,
        variant_id: &str,
        limit: usize,
    ) -> Vec<ComponentRecommendation> {
        // Combine section type and variant keywords for search
        let mut keywords = variant_to_keywords(variant_id);
        keywords.push(section_type.to_string());

        let search_query = keywords.join(" ");
        let results = self.reader.search(self.framework, &search_query);

        results
            .into_iter()
            .take(limit)
            .map(|c| ComponentRecommendation {
                id: c.id.clone(),
                name: c.name.clone(),
                category: c.category.clone(),
                subcategory: c.subcategory.clone(),
                sub_subcategory: c.sub_subcategory.clone(),
                confidence: 0.5, // Lower confidence for fallback
                preview_url: c.preview_url(Mode::Light).map(ToString::to_string),
            })
            .collect()
    }
}

impl Default for ComponentMatcher {
    fn default() -> Self {
        Self::react()
    }
}

/// Map section type to Tailwind Plus sub-subcategory.
///
/// These mappings are based on the actual catalog structure:
/// Marketing → Page Sections → {sub-subcategory}
fn section_to_sub_subcategory(section_type: &str) -> Option<&'static str> {
    match section_type.to_lowercase().as_str() {
        "hero" => Some("Hero Sections"),
        "header" | "navbar" | "nav" => Some("Header Sections"),
        "features" | "feature" => Some("Feature Sections"),
        "pricing" => Some("Pricing Sections"),
        "cta" | "call-to-action" => Some("CTA Sections"),
        "testimonial" | "testimonials" => Some("Testimonials"),
        "faq" | "faqs" => Some("FAQs"),
        "footer" => Some("Footers"),
        "stats" | "statistics" => Some("Stats"),
        "logos" | "brands" | "logo-cloud" | "social-proof" => Some("Logo Clouds"),
        "team" => Some("Team Sections"),
        "contact" => Some("Contact Sections"),
        "blog" => Some("Blog Sections"),
        "newsletter" => Some("Newsletter Sections"),
        "content" => Some("Content Sections"),
        "bento" | "bento-grid" => Some("Bento Grids"),
        _ => None,
    }
}

/// Extract keywords from a variant ID.
///
/// "hero-split-screenshot" → ["split", "screenshot"]
/// "pricing-three-tier" → ["three", "tier"]
fn variant_to_keywords(variant_id: &str) -> Vec<String> {
    variant_id
        .split('-')
        .skip(1) // Skip the section type prefix
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

/// Calculate confidence score based on keyword matches.
fn calculate_confidence(component_name: &str, keywords: &[String]) -> f32 {
    if keywords.is_empty() {
        return 0.5; // Base confidence when no keywords
    }

    let name_lower = component_name.to_lowercase();
    let matched = keywords
        .iter()
        .filter(|kw| name_lower.contains(&kw.to_lowercase()))
        .count();

    // Base confidence + keyword bonus
    let keyword_score = matched as f32 / keywords.len() as f32;
    keyword_score.mul_add(0.7, 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_mapping_hero() {
        assert_eq!(section_to_sub_subcategory("hero"), Some("Hero Sections"));
        assert_eq!(section_to_sub_subcategory("HERO"), Some("Hero Sections"));
    }

    #[test]
    fn section_mapping_header_variants() {
        assert_eq!(
            section_to_sub_subcategory("header"),
            Some("Header Sections")
        );
        assert_eq!(
            section_to_sub_subcategory("navbar"),
            Some("Header Sections")
        );
        assert_eq!(section_to_sub_subcategory("nav"), Some("Header Sections"));
    }

    #[test]
    fn section_mapping_all_types() {
        assert_eq!(
            section_to_sub_subcategory("features"),
            Some("Feature Sections")
        );
        assert_eq!(
            section_to_sub_subcategory("pricing"),
            Some("Pricing Sections")
        );
        assert_eq!(section_to_sub_subcategory("cta"), Some("CTA Sections"));
        assert_eq!(
            section_to_sub_subcategory("testimonial"),
            Some("Testimonials")
        );
        assert_eq!(section_to_sub_subcategory("faq"), Some("FAQs"));
        assert_eq!(section_to_sub_subcategory("footer"), Some("Footers"));
        assert_eq!(section_to_sub_subcategory("stats"), Some("Stats"));
        assert_eq!(section_to_sub_subcategory("logos"), Some("Logo Clouds"));
    }

    #[test]
    fn section_mapping_unknown() {
        assert_eq!(section_to_sub_subcategory("unknown"), None);
        assert_eq!(section_to_sub_subcategory("custom"), None);
    }

    #[test]
    fn variant_keywords_extraction() {
        assert_eq!(
            variant_to_keywords("hero-split-screenshot"),
            vec!["split", "screenshot"]
        );
        assert_eq!(
            variant_to_keywords("pricing-three-tier"),
            vec!["three", "tier"]
        );
        assert_eq!(
            variant_to_keywords("cta-simple-centered"),
            vec!["simple", "centered"]
        );
    }

    #[test]
    fn variant_keywords_single() {
        assert_eq!(variant_to_keywords("footer-simple"), vec!["simple"]);
    }

    #[test]
    fn variant_keywords_empty() {
        assert_eq!(variant_to_keywords("hero"), Vec::<String>::new());
        assert_eq!(variant_to_keywords(""), Vec::<String>::new());
    }

    #[test]
    fn confidence_no_keywords() {
        assert!((calculate_confidence("Any Component", &[]) - 0.5).abs() < 0.01);
    }

    #[test]
    fn confidence_full_match() {
        let keywords = vec!["split".to_string(), "image".to_string()];
        let score = calculate_confidence("Split with image", &keywords);
        assert!(score > 0.9, "Full match should score > 0.9, got {score}");
    }

    #[test]
    fn confidence_partial_match() {
        let keywords = vec!["split".to_string(), "video".to_string()];
        let score = calculate_confidence("Split with image", &keywords);
        // Only "split" matches
        assert!(
            (0.5..0.8).contains(&score),
            "Partial match should score 0.5-0.8, got {score}"
        );
    }

    #[test]
    fn confidence_no_match() {
        let keywords = vec!["video".to_string(), "background".to_string()];
        let score = calculate_confidence("Split with image", &keywords);
        assert!(score < 0.4, "No match should score < 0.4, got {score}");
    }

    // Tests that require embedded data
    #[cfg(feature = "embedded-data")]
    mod embedded_tests {
        use super::*;

        #[test]
        fn match_hero_section() {
            let matcher = ComponentMatcher::react();
            let matches = matcher.match_section("hero", "hero-split-screenshot", 5);

            assert!(!matches.is_empty(), "Should find hero components");
            assert!(
                matches.iter().all(|m| m.sub_subcategory == "Hero Sections"),
                "All matches should be Hero Sections"
            );
        }

        #[test]
        fn match_pricing_section() {
            let matcher = ComponentMatcher::react();
            let matches = matcher.match_section("pricing", "pricing-three-tier", 5);

            assert!(!matches.is_empty(), "Should find pricing components");
            assert!(
                matches
                    .iter()
                    .all(|m| m.sub_subcategory == "Pricing Sections"),
                "All matches should be Pricing Sections"
            );
        }

        #[test]
        fn matches_sorted_by_confidence() {
            let matcher = ComponentMatcher::react();
            let matches = matcher.match_section("hero", "hero-split-image", 10);

            // Check that confidence is descending
            for i in 1..matches.len() {
                assert!(
                    matches[i - 1].confidence >= matches[i].confidence,
                    "Matches should be sorted by confidence"
                );
            }
        }

        #[test]
        fn matches_include_preview_urls() {
            let matcher = ComponentMatcher::react();
            let matches = matcher.match_section("hero", "hero-centered", 3);

            // At least some should have preview URLs
            let has_previews = matches.iter().any(|m| m.preview_url.is_some());
            assert!(has_previews, "Some matches should have preview URLs");
        }
    }
}
