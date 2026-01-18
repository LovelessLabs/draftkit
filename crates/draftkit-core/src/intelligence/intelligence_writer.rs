//! Component intelligence data writer.
//!
//! Generates `component-intelligence.json` from template analysis.

use crate::components::{ComponentIntelligence, PagePosition, StyleProfile, UsageContext};
use crate::intelligence::{SectionAnalysis, SectionType, TemplateAnalysis};
use camino::Utf8Path;
use std::collections::HashMap;

/// Relationship between components (followed_by or preceded_by).
#[derive(Debug, Clone)]
struct Relationship {
    id: String,
    count: usize,
}

/// Component intelligence builder.
///
/// Aggregates template analysis data and converts it to `ComponentIntelligence`
/// format for serialization.
#[derive(Debug, Default)]
pub struct IntelligenceBuilder {
    /// Section ID → all analyses of that section
    sections: HashMap<String, Vec<SectionAnalysis>>,
    /// Section ID → what follows it (and how often)
    followed_by: HashMap<String, HashMap<String, usize>>,
    /// Section ID → what precedes it
    preceded_by: HashMap<String, HashMap<String, usize>>,
    /// Section ID → which templates it appears in
    template_occurrences: HashMap<String, Vec<String>>,
    /// Total number of pages analyzed
    total_pages: usize,
    /// Section ID → number of page appearances
    page_appearances: HashMap<String, usize>,
}

impl IntelligenceBuilder {
    /// Create a new intelligence builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a template analysis to the builder.
    pub fn add_template(&mut self, analysis: &TemplateAnalysis) {
        let template_name = &analysis.name;

        // Index all sections
        for section in &analysis.sections {
            self.sections
                .entry(section.id.clone())
                .or_default()
                .push(section.clone());

            self.template_occurrences
                .entry(section.id.clone())
                .or_default()
                .push(template_name.clone());
        }

        // Analyze page sequences
        for page in &analysis.pages {
            self.total_pages += 1;

            for (i, section_id) in page.sections.iter().enumerate() {
                // Track page appearances
                *self.page_appearances.entry(section_id.clone()).or_insert(0) += 1;

                // Track what follows
                if i + 1 < page.sections.len() {
                    let next = &page.sections[i + 1];
                    *self
                        .followed_by
                        .entry(section_id.clone())
                        .or_default()
                        .entry(next.clone())
                        .or_insert(0) += 1;
                }

                // Track what precedes
                if i > 0 {
                    let prev = &page.sections[i - 1];
                    *self
                        .preceded_by
                        .entry(section_id.clone())
                        .or_default()
                        .entry(prev.clone())
                        .or_insert(0) += 1;
                }
            }
        }
    }

    /// Build the final intelligence data.
    #[must_use]
    pub fn build(self) -> HashMap<String, ComponentIntelligence> {
        let mut result = HashMap::new();

        for (section_id, analyses) in &self.sections {
            // Average the style profiles
            let style = Self::average_style_profile(analyses);

            // Get section type from first analysis
            let section_type = analyses.first().map(|a| a.section_type);

            // Calculate frequency
            let frequency = self
                .page_appearances
                .get(section_id)
                .copied()
                .unwrap_or(0) as f32
                / self.total_pages.max(1) as f32;

            // Get templates this section appears in
            let templates = self
                .template_occurrences
                .get(section_id)
                .cloned()
                .unwrap_or_default();

            // Build followed_by list
            let followed_by = self
                .followed_by
                .get(section_id)
                .map(|map| {
                    let mut rels: Vec<_> = map
                        .iter()
                        .map(|(id, &count)| Relationship {
                            id: id.clone(),
                            count,
                        })
                        .collect();
                    rels.sort_by(|a, b| b.count.cmp(&a.count));
                    rels.into_iter()
                        .take(5)
                        .map(|r| r.id)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            // Build preceded_by list
            let preceded_by = self
                .preceded_by
                .get(section_id)
                .map(|map| {
                    let mut rels: Vec<_> = map
                        .iter()
                        .map(|(id, &count)| Relationship {
                            id: id.clone(),
                            count,
                        })
                        .collect();
                    rels.sort_by(|a, b| b.count.cmp(&a.count));
                    rels.into_iter()
                        .take(5)
                        .map(|r| r.id)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            // Determine page types from template names
            let page_types = Self::infer_page_types(&templates);

            // Map section type to page position
            let position = section_type.and_then(Self::section_type_to_position);

            let intel = ComponentIntelligence {
                id: section_id.clone(),
                category: vec![
                    "Marketing".to_string(),
                    section_type.map_or("Unknown", |st| st.as_str()).to_string(),
                ],
                extracted: Default::default(),
                style,
                usage: UsageContext {
                    page_types,
                    position,
                    frequency,
                    followed_by,
                    preceded_by,
                },
            };

            result.insert(section_id.clone(), intel);
        }

        result
    }

    /// Average style profiles from multiple analyses.
    fn average_style_profile(analyses: &[SectionAnalysis]) -> StyleProfile {
        if analyses.is_empty() {
            return StyleProfile::default();
        }

        let n = analyses.len() as f32;
        let mut total = StyleProfile::default();

        for a in analyses {
            total.visual_weight += a.style.visual_weight;
            total.formality += a.style.formality;
            total.color_intensity += a.style.color_intensity;
            total.spacing_density += a.style.spacing_density;
        }

        StyleProfile {
            visual_weight: total.visual_weight / n,
            formality: total.formality / n,
            color_intensity: total.color_intensity / n,
            spacing_density: total.spacing_density / n,
            // Use most common typography scale
            typography_scale: analyses[0].style.typography_scale,
        }
    }

    /// Infer page types from template names.
    fn infer_page_types(templates: &[String]) -> Vec<String> {
        let mut types = Vec::new();

        // Helper to add type if not already present
        let mut add_type = |t: &str| {
            let s = t.to_string();
            if !types.contains(&s) {
                types.push(s);
            }
        };

        for template in templates {
            let lower = template.to_lowercase();

            if lower.contains("saas") || lower.contains("commit") {
                add_type("saas");
            }
            if lower.contains("marketing") || lower.contains("landing") {
                add_type("marketing");
            }
            if lower.contains("docs") || lower.contains("protocol") {
                add_type("documentation");
            }
            if lower.contains("ecommerce") || lower.contains("store") {
                add_type("ecommerce");
            }
        }

        // Default to landing if no types inferred
        if types.is_empty() {
            types.push("landing".to_string());
        }

        types
    }

    /// Map section type to page position.
    const fn section_type_to_position(section_type: SectionType) -> Option<PagePosition> {
        match section_type {
            SectionType::Hero => Some(PagePosition::Hero),
            SectionType::Features => Some(PagePosition::Feature),
            SectionType::Pricing => Some(PagePosition::Pricing),
            SectionType::Testimonial => Some(PagePosition::Testimonial),
            SectionType::Faq => Some(PagePosition::Faq),
            SectionType::Cta => Some(PagePosition::Cta),
            SectionType::Footer => Some(PagePosition::Footer),
            SectionType::Header => Some(PagePosition::Header),
            SectionType::Stats | SectionType::LogoCloud => Some(PagePosition::SocialProof),
            SectionType::Contact => Some(PagePosition::Form),
            SectionType::Content | SectionType::Document => Some(PagePosition::Content),
            _ => None,
        }
    }

    /// Write the intelligence data to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn write_to_file(&self, path: &Utf8Path) -> std::io::Result<()> {
        // Clone self to call build() which consumes self
        let builder = Self {
            sections: self.sections.clone(),
            followed_by: self.followed_by.clone(),
            preceded_by: self.preceded_by.clone(),
            template_occurrences: self.template_occurrences.clone(),
            total_pages: self.total_pages,
            page_appearances: self.page_appearances.clone(),
        };

        let data = builder.build();

        // Wrap in expected structure
        let output = serde_json::json!({
            "components": data,
            "metadata": {
                "total_sections": self.sections.len(),
                "total_pages": self.total_pages,
            }
        });

        let json = serde_json::to_string_pretty(&output).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;

        std::fs::write(path.as_std_path(), json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::TypographyScale;

    fn mock_section(id: &str, section_type: SectionType) -> SectionAnalysis {
        SectionAnalysis {
            id: id.to_string(),
            name: id.replace('-', " "),
            section_type,
            style: StyleProfile {
                visual_weight: 0.5,
                formality: 0.7,
                color_intensity: 0.4,
                spacing_density: 0.6,
                typography_scale: TypographyScale::Medium,
            },
            source_path: Utf8Path::new("/test").to_owned(),
            source_code: String::new(),
        }
    }

    #[test]
    fn builder_empty() {
        let builder = IntelligenceBuilder::new();
        let result = builder.build();
        assert!(result.is_empty());
    }

    #[test]
    fn builder_single_section() {
        let mut builder = IntelligenceBuilder::new();

        let analysis = TemplateAnalysis {
            name: "test".to_string(),
            path: Utf8Path::new("/test").to_owned(),
            sections: vec![mock_section("hero-simple", SectionType::Hero)],
            pages: vec![],
        };

        builder.add_template(&analysis);
        let result = builder.build();

        assert_eq!(result.len(), 1);
        assert!(result.contains_key("hero-simple"));

        let intel = &result["hero-simple"];
        assert_eq!(intel.usage.position, Some(PagePosition::Hero));
    }

    #[test]
    fn builder_tracks_sequences() {
        let mut builder = IntelligenceBuilder::new();

        use crate::intelligence::PageStructure;

        let analysis = TemplateAnalysis {
            name: "test".to_string(),
            path: Utf8Path::new("/test").to_owned(),
            sections: vec![
                mock_section("hero-simple", SectionType::Hero),
                mock_section("features-grid", SectionType::Features),
                mock_section("pricing-simple", SectionType::Pricing),
            ],
            pages: vec![PageStructure {
                name: "home".to_string(),
                sections: vec![
                    "hero-simple".to_string(),
                    "features-grid".to_string(),
                    "pricing-simple".to_string(),
                ],
                source_path: Utf8Path::new("/test/page.tsx").to_owned(),
            }],
        };

        builder.add_template(&analysis);
        let result = builder.build();

        // Hero should be followed by features
        let hero = &result["hero-simple"];
        assert!(hero.usage.followed_by.contains(&"features-grid".to_string()));

        // Features should be preceded by hero and followed by pricing
        let features = &result["features-grid"];
        assert!(features.usage.preceded_by.contains(&"hero-simple".to_string()));
        assert!(features.usage.followed_by.contains(&"pricing-simple".to_string()));
    }
}
