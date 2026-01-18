//! Cross-template site intelligence for combining TailwindPlus templates.
//!
//! This module aggregates page analyses from multiple templates to provide
//! recommendations for building sites that combine components from different
//! templates (e.g., commit's blog + oatmeal's pricing + syntax's docs).
//!
//! # Example
//!
//! ```rust,ignore
//! use draftkit_core::intelligence::{PageAnalyzer, PageType, SiteIntelligence};
//!
//! let mut analyzer = PageAnalyzer::new();
//! // ... analyze templates ...
//!
//! let site = SiteIntelligence::from_analyzer(&analyzer);
//!
//! // Get recommendation for a marketing site with blog and docs
//! let rec = site.recommend_for_site(&[
//!     PageType::Home,
//!     PageType::Blog,
//!     PageType::Docs,
//!     PageType::Pricing,
//! ]);
//!
//! println!("Use {} for home page", rec.template_for(PageType::Home));
//! println!("Use {} for blog", rec.template_for(PageType::Blog));
//! ```

use std::collections::{HashMap, HashSet};

use super::{PageAnalyzer, PageType, TemplatePageAnalysis};

/// Aggregated intelligence across all analyzed templates.
#[derive(Debug)]
pub struct SiteIntelligence {
    /// Best template for each page type (by page count/quality)
    best_for_type: HashMap<PageType, TemplateRanking>,
    /// Components that appear across multiple templates (potential shared components)
    cross_template_components: Vec<CrossTemplateComponent>,
    /// Template names indexed by their strengths
    templates_by_strength: HashMap<PageType, Vec<String>>,
}

/// Ranking of templates for a specific page type.
#[derive(Debug, Clone)]
pub struct TemplateRanking {
    /// Best template name
    pub best: String,
    /// Score (higher is better)
    pub score: u32,
    /// Alternative templates that also support this page type
    pub alternatives: Vec<String>,
}

/// A component that appears in multiple templates.
#[derive(Debug, Clone)]
pub struct CrossTemplateComponent {
    /// Component identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Templates containing this component
    pub templates: Vec<String>,
    /// Page types this component is used in
    pub page_types: HashSet<PageType>,
}

/// Recommendation for building a multi-page-type site.
#[derive(Debug, Clone)]
pub struct SiteRecommendation {
    /// Which template to use for each page type
    pub template_assignments: HashMap<PageType, String>,
    /// Number of unique templates being used
    pub template_count: usize,
    /// Components that could be shared across templates
    pub shareable_components: Vec<String>,
    /// Style bridging notes
    pub style_notes: Vec<String>,
}

impl SiteIntelligence {
    /// Build site intelligence from a page analyzer's cached analyses.
    #[must_use]
    pub fn from_analyzer(analyzer: &PageAnalyzer) -> Self {
        let analyses: Vec<_> = analyzer.analyses().values().cloned().collect();
        Self::from_analyses(&analyses)
    }

    /// Build site intelligence from a collection of template analyses.
    #[must_use]
    pub fn from_analyses(analyses: &[TemplatePageAnalysis]) -> Self {
        let mut best_for_type: HashMap<PageType, TemplateRanking> = HashMap::new();
        let mut templates_by_strength: HashMap<PageType, Vec<String>> = HashMap::new();
        let mut component_occurrences: HashMap<String, CrossTemplateComponent> = HashMap::new();

        for analysis in analyses {
            // Count pages per type for this template
            let mut type_counts: HashMap<PageType, u32> = HashMap::new();
            for page in &analysis.pages {
                *type_counts.entry(page.page_type).or_insert(0) += 1;
            }

            // Update best template for each type
            for (page_type, count) in type_counts {
                if page_type == PageType::Unknown {
                    continue;
                }

                let entry = best_for_type
                    .entry(page_type)
                    .or_insert_with(|| TemplateRanking {
                        best: analysis.name.clone(),
                        score: 0,
                        alternatives: Vec::new(),
                    });

                if count > entry.score {
                    // This template is better
                    if entry.score > 0 {
                        entry.alternatives.push(entry.best.clone());
                    }
                    entry.best = analysis.name.clone();
                    entry.score = count;
                } else if count > 0 {
                    entry.alternatives.push(analysis.name.clone());
                }

                // Track template strengths
                templates_by_strength
                    .entry(page_type)
                    .or_default()
                    .push(analysis.name.clone());
            }

            // Track cross-template components
            for (id, comp) in &analysis.components {
                component_occurrences
                    .entry(id.clone())
                    .and_modify(|c| {
                        if !c.templates.contains(&analysis.name) {
                            c.templates.push(analysis.name.clone());
                        }
                        c.page_types.extend(&comp.page_types);
                    })
                    .or_insert_with(|| CrossTemplateComponent {
                        id: id.clone(),
                        name: comp.name.clone(),
                        templates: vec![analysis.name.clone()],
                        page_types: comp.page_types.clone(),
                    });
            }
        }

        // Filter to only components that appear in multiple templates
        let cross_template_components: Vec<_> = component_occurrences
            .into_values()
            .filter(|c| c.templates.len() > 1)
            .collect();

        Self {
            best_for_type,
            cross_template_components,
            templates_by_strength,
        }
    }

    /// Get the best template for a specific page type.
    #[must_use]
    pub fn best_template_for(&self, page_type: PageType) -> Option<&str> {
        self.best_for_type.get(&page_type).map(|r| r.best.as_str())
    }

    /// Get the ranking for a specific page type.
    #[must_use]
    pub fn ranking_for(&self, page_type: PageType) -> Option<&TemplateRanking> {
        self.best_for_type.get(&page_type)
    }

    /// Get all templates that support a page type.
    #[must_use]
    pub fn templates_supporting(&self, page_type: PageType) -> &[String] {
        self.templates_by_strength
            .get(&page_type)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Get components that appear in multiple templates.
    #[must_use]
    pub fn cross_template_components(&self) -> &[CrossTemplateComponent] {
        &self.cross_template_components
    }

    /// Generate a recommendation for building a site with multiple page types.
    #[must_use]
    pub fn recommend_for_site(&self, page_types: &[PageType]) -> SiteRecommendation {
        let mut template_assignments: HashMap<PageType, String> = HashMap::new();
        let mut templates_used: HashSet<String> = HashSet::new();

        // Assign best template for each page type
        for &page_type in page_types {
            if let Some(ranking) = self.best_for_type.get(&page_type) {
                template_assignments.insert(page_type, ranking.best.clone());
                templates_used.insert(ranking.best.clone());
            }
        }

        // Find components that could be shared
        let shareable_components: Vec<_> = self
            .cross_template_components
            .iter()
            .filter(|c| {
                // Component is shareable if it appears in templates we're using
                c.templates.iter().any(|t| templates_used.contains(t))
            })
            .filter(|c| {
                // Focus on layout components (header, footer, nav)
                let lower = c.id.to_lowercase();
                lower.contains("header")
                    || lower.contains("footer")
                    || lower.contains("nav")
                    || lower.contains("layout")
            })
            .map(|c| c.name.clone())
            .collect();

        // Generate style notes
        let style_notes = self.generate_style_notes(&templates_used, page_types);

        SiteRecommendation {
            template_assignments,
            template_count: templates_used.len(),
            shareable_components,
            style_notes,
        }
    }

    /// Generate style bridging notes for a set of templates.
    fn generate_style_notes(
        &self,
        templates: &HashSet<String>,
        page_types: &[PageType],
    ) -> Vec<String> {
        let mut notes = Vec::new();

        if templates.len() > 1 {
            notes.push(format!(
                "Combining {} templates - ensure consistent color palette and typography",
                templates.len()
            ));
        }

        // Check for potential conflicts
        let has_blog = page_types.contains(&PageType::Blog);
        let has_docs = page_types.contains(&PageType::Docs);

        if has_blog && has_docs {
            notes.push(
                "Both blog and docs pages detected - consider using a unified reading experience"
                    .to_string(),
            );
        }

        if templates.len() == 1 {
            notes.push(format!(
                "Single template ({}) can handle all page types - easiest integration",
                templates.iter().next().unwrap_or(&String::new())
            ));
        }

        notes
    }

    /// Get all supported page types.
    #[must_use]
    pub fn supported_page_types(&self) -> Vec<PageType> {
        self.best_for_type.keys().copied().collect()
    }
}

impl SiteRecommendation {
    /// Get the recommended template for a specific page type.
    #[must_use]
    pub fn template_for(&self, page_type: PageType) -> Option<&str> {
        self.template_assignments
            .get(&page_type)
            .map(String::as_str)
    }

    /// Check if this recommendation uses multiple templates.
    #[must_use]
    pub const fn is_multi_template(&self) -> bool {
        self.template_count > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_analysis(name: &str, page_types: &[PageType]) -> TemplatePageAnalysis {
        use crate::intelligence::PageAnalysis;
        use camino::Utf8PathBuf;

        let pages: Vec<PageAnalysis> = page_types
            .iter()
            .map(|pt| PageAnalysis {
                route: format!("/{}", pt.as_str()),
                page_type: *pt,
                components: vec![],
                source_path: Utf8PathBuf::new(),
                template_name: name.to_string(),
            })
            .collect();

        TemplatePageAnalysis {
            name: name.to_string(),
            path: Utf8PathBuf::new(),
            pages,
            components: HashMap::new(),
            layouts: vec![],
            strengths: page_types.to_vec(),
        }
    }

    #[test]
    fn best_template_for_page_type() {
        let analyses = vec![
            mock_analysis("oatmeal", &[PageType::Home, PageType::Pricing]),
            mock_analysis("syntax", &[PageType::Docs, PageType::Docs, PageType::Docs]),
            mock_analysis("commit", &[PageType::Blog, PageType::Blog]),
        ];

        let site = SiteIntelligence::from_analyses(&analyses);

        assert_eq!(site.best_template_for(PageType::Docs), Some("syntax"));
        assert_eq!(site.best_template_for(PageType::Blog), Some("commit"));
        // Both oatmeal has pricing, it should be best
        assert_eq!(site.best_template_for(PageType::Pricing), Some("oatmeal"));
    }

    #[test]
    fn recommend_for_site_assigns_templates() {
        let analyses = vec![
            mock_analysis("oatmeal", &[PageType::Home, PageType::Pricing]),
            mock_analysis("syntax", &[PageType::Docs]),
            mock_analysis("commit", &[PageType::Blog]),
        ];

        let site = SiteIntelligence::from_analyses(&analyses);
        let rec = site.recommend_for_site(&[
            PageType::Home,
            PageType::Blog,
            PageType::Docs,
            PageType::Pricing,
        ]);

        assert_eq!(rec.template_for(PageType::Home), Some("oatmeal"));
        assert_eq!(rec.template_for(PageType::Blog), Some("commit"));
        assert_eq!(rec.template_for(PageType::Docs), Some("syntax"));
        assert_eq!(rec.template_for(PageType::Pricing), Some("oatmeal"));
        assert_eq!(rec.template_count, 3);
        assert!(rec.is_multi_template());
    }

    #[test]
    fn single_template_recommendation() {
        let analyses = vec![mock_analysis(
            "studio",
            &[
                PageType::Home,
                PageType::Blog,
                PageType::About,
                PageType::Contact,
            ],
        )];

        let site = SiteIntelligence::from_analyses(&analyses);
        let rec = site.recommend_for_site(&[PageType::Home, PageType::Blog, PageType::About]);

        assert_eq!(rec.template_count, 1);
        assert!(!rec.is_multi_template());
        assert!(
            rec.style_notes
                .iter()
                .any(|n| n.contains("Single template"))
        );
    }
}
