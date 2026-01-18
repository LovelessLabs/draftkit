//! Template analysis for component intelligence extraction.
//!
//! Analyzes TailwindPlus template kits to extract:
//! - Section component `StyleProfile`s
//! - Page structure sequences
//! - Component co-occurrence patterns
//!
//! # Overview
//!
//! Templates are professionally designed, coherent sites. By analyzing them
//! we learn:
//! 1. What `StyleProfile` values make components "work" together
//! 2. Which sections typically follow which (hero → features → pricing)
//! 3. What page structures are common for different site types
//!
//! # Example
//!
//! ```rust,ignore
//! use draftkit_core::intelligence::TemplateAnalyzer;
//!
//! let analyzer = TemplateAnalyzer::new();
//! let result = analyzer.analyze_template("/path/to/oatmeal")?;
//!
//! for section in &result.sections {
//!     println!("{}: visual_weight={:.2}", section.name, section.style.visual_weight);
//! }
//! ```

use crate::components::{StyleProfile, TypographyScale};
use crate::intelligence::StyleExtractor;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;

/// Capitalize the first letter of a word.
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .map_or_else(String::new, |c| c.to_uppercase().chain(chars).collect())
}

/// Result of analyzing a single template kit.
#[derive(Debug, Clone)]
pub struct TemplateAnalysis {
    /// Template name (e.g., "oatmeal")
    pub name: String,
    /// Path to template root
    pub path: Utf8PathBuf,
    /// Analyzed section components
    pub sections: Vec<SectionAnalysis>,
    /// Page structures found
    pub pages: Vec<PageStructure>,
}

/// Analysis of a single section component.
#[derive(Debug, Clone)]
pub struct SectionAnalysis {
    /// Section identifier derived from filename (e.g., "hero-centered-with-demo")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Inferred section type (hero, features, pricing, etc.)
    pub section_type: SectionType,
    /// Extracted style profile
    pub style: StyleProfile,
    /// Source file path
    pub source_path: Utf8PathBuf,
    /// Raw source code
    pub source_code: String,
}

/// Page structure showing section order.
#[derive(Debug, Clone)]
pub struct PageStructure {
    /// Page name (e.g., "home", "pricing")
    pub name: String,
    /// Ordered list of section IDs used in this page
    pub sections: Vec<String>,
    /// Source file path
    pub source_path: Utf8PathBuf,
}

/// Section type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionType {
    Hero,
    Features,
    Pricing,
    Testimonial,
    Faq,
    Cta,
    Footer,
    Header,
    Team,
    Stats,
    LogoCloud,
    Contact,
    Newsletter,
    Content,
    Document,
    Unknown,
}

impl SectionType {
    /// Infer section type from filename or component name.
    #[must_use]
    pub fn from_name(name: &str) -> Self {
        let lower = name.to_lowercase();

        if lower.contains("hero") {
            Self::Hero
        } else if lower.contains("feature") {
            Self::Features
        } else if lower.contains("pricing") {
            Self::Pricing
        } else if lower.contains("testimonial") {
            Self::Testimonial
        } else if lower.contains("faq") {
            Self::Faq
        } else if lower.contains("cta") || lower.contains("call-to-action") {
            Self::Cta
        } else if lower.contains("footer") {
            Self::Footer
        } else if lower.contains("header") || lower.contains("nav") {
            Self::Header
        } else if lower.contains("team") {
            Self::Team
        } else if lower.contains("stat") {
            Self::Stats
        } else if lower.contains("logo") || lower.contains("brand") {
            Self::LogoCloud
        } else if lower.contains("contact") {
            Self::Contact
        } else if lower.contains("newsletter") {
            Self::Newsletter
        } else if lower.contains("content") || lower.contains("article") {
            Self::Content
        } else if lower.contains("document") {
            Self::Document
        } else {
            Self::Unknown
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Hero => "hero",
            Self::Features => "features",
            Self::Pricing => "pricing",
            Self::Testimonial => "testimonial",
            Self::Faq => "faq",
            Self::Cta => "cta",
            Self::Footer => "footer",
            Self::Header => "header",
            Self::Team => "team",
            Self::Stats => "stats",
            Self::LogoCloud => "logo-cloud",
            Self::Contact => "contact",
            Self::Newsletter => "newsletter",
            Self::Content => "content",
            Self::Document => "document",
            Self::Unknown => "unknown",
        }
    }
}

/// Analyzes TailwindPlus template kits.
#[derive(Debug, Clone, Default)]
pub struct TemplateAnalyzer {
    /// Cached analyses
    analyses: HashMap<String, TemplateAnalysis>,
}

impl TemplateAnalyzer {
    /// Create a new template analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyses: HashMap::new(),
        }
    }

    /// Analyze a single template kit directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the template directory doesn't exist or can't be read.
    pub fn analyze_template(
        &mut self,
        path: &Utf8Path,
    ) -> Result<&TemplateAnalysis, AnalysisError> {
        let name = path
            .file_name()
            .ok_or(AnalysisError::InvalidPath)?
            .to_string();

        if self.analyses.contains_key(&name) {
            return Ok(self.analyses.get(&name).unwrap());
        }

        let analysis = Self::do_analyze(path)?;
        self.analyses.insert(name.clone(), analysis);

        Ok(self.analyses.get(&name).unwrap())
    }

    /// Perform the actual analysis.
    fn do_analyze(path: &Utf8Path) -> Result<TemplateAnalysis, AnalysisError> {
        let name = path
            .file_name()
            .ok_or(AnalysisError::InvalidPath)?
            .to_string();

        // Find sections directory
        let sections_dir = Self::find_sections_dir(path)?;
        let sections = Self::analyze_sections(&sections_dir)?;

        // Find page files
        let pages_dir = Self::find_pages_dir(path);
        let pages = if let Some(dir) = pages_dir {
            Self::analyze_pages(&dir, &sections)?
        } else {
            Vec::new()
        };

        Ok(TemplateAnalysis {
            name,
            path: path.to_owned(),
            sections,
            pages,
        })
    }

    /// Find the sections directory in a template.
    fn find_sections_dir(path: &Utf8Path) -> Result<Utf8PathBuf, AnalysisError> {
        // Common patterns
        let candidates = [
            path.join("demo/src/components/sections"),
            path.join("src/components/sections"),
            path.join("components/sections"),
            path.join("app/components/sections"),
        ];

        for candidate in candidates {
            if candidate.exists() && candidate.is_dir() {
                return Ok(candidate);
            }
        }

        Err(AnalysisError::NoSectionsDir)
    }

    /// Find the pages directory in a template.
    fn find_pages_dir(path: &Utf8Path) -> Option<Utf8PathBuf> {
        let candidates = [
            path.join("demo/src/app"),
            path.join("src/app"),
            path.join("app"),
            path.join("demo/src/pages"),
            path.join("src/pages"),
            path.join("pages"),
        ];

        candidates
            .into_iter()
            .find(|candidate| candidate.exists() && candidate.is_dir())
    }

    /// Analyze all section components in a directory.
    fn analyze_sections(dir: &Utf8Path) -> Result<Vec<SectionAnalysis>, AnalysisError> {
        let mut sections = Vec::new();

        let entries = std::fs::read_dir(dir.as_std_path())
            .map_err(|e| AnalysisError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| AnalysisError::IoError(e.to_string()))?;
            let path = entry.path();

            // Only process .tsx files
            if path.extension().and_then(|e| e.to_str()) != Some("tsx") {
                continue;
            }

            let path = Utf8PathBuf::try_from(path)
                .map_err(|_| AnalysisError::InvalidPath)?;

            if let Some(analysis) = Self::analyze_section_file(&path)? {
                sections.push(analysis);
            }
        }

        Ok(sections)
    }

    /// Analyze a single section file.
    fn analyze_section_file(path: &Utf8Path) -> Result<Option<SectionAnalysis>, AnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| AnalysisError::IoError(e.to_string()))?;

        // Skip very small files (likely index.ts exports)
        if code.len() < 100 {
            return Ok(None);
        }

        // Extract ID from filename
        let id = path
            .file_stem()
            .ok_or(AnalysisError::InvalidPath)?
            .to_string();

        // Generate human-readable name
        let name = id
            .split('-')
            .map(capitalize_word)
            .collect::<Vec<_>>()
            .join(" ");

        // Infer section type
        let section_type = SectionType::from_name(&id);

        // Extract style profile
        let style = StyleExtractor::extract(&code);

        Ok(Some(SectionAnalysis {
            id,
            name,
            section_type,
            style,
            source_path: path.to_owned(),
            source_code: code,
        }))
    }

    /// Analyze page files to extract section sequences.
    fn analyze_pages(
        dir: &Utf8Path,
        sections: &[SectionAnalysis],
    ) -> Result<Vec<PageStructure>, AnalysisError> {
        let mut pages = Vec::new();

        // Build a map of component names to section IDs
        let mut name_to_id: HashMap<String, String> = HashMap::new();
        for section in sections {
            // Map PascalCase export name to section ID
            let pascal_name = section
                .id
                .split('-')
                .map(capitalize_word)
                .collect::<String>();
            name_to_id.insert(pascal_name, section.id.clone());
        }

        // Find page.tsx files
        Self::find_page_files(dir, &mut pages, &name_to_id)?;

        Ok(pages)
    }

    /// Recursively find page.tsx files.
    fn find_page_files(
        dir: &Utf8Path,
        pages: &mut Vec<PageStructure>,
        name_to_id: &HashMap<String, String>,
    ) -> Result<(), AnalysisError> {
        let entries = std::fs::read_dir(dir.as_std_path())
            .map_err(|e| AnalysisError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| AnalysisError::IoError(e.to_string()))?;
            let path = entry.path();

            if path.is_dir() {
                let utf8_path = Utf8PathBuf::try_from(path)
                    .map_err(|_| AnalysisError::InvalidPath)?;
                Self::find_page_files(&utf8_path, pages, name_to_id)?;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("page.tsx") {
                let utf8_path = Utf8PathBuf::try_from(path.clone())
                    .map_err(|_| AnalysisError::InvalidPath)?;

                if let Some(structure) = Self::analyze_page_file(&utf8_path, name_to_id)? {
                    pages.push(structure);
                }
            }
        }

        Ok(())
    }

    /// Analyze a page file to extract section sequence.
    fn analyze_page_file(
        path: &Utf8Path,
        name_to_id: &HashMap<String, String>,
    ) -> Result<Option<PageStructure>, AnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| AnalysisError::IoError(e.to_string()))?;

        // Extract page name from directory
        let name = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "home".to_string());

        // Find component usages in JSX
        let mut sections = Vec::new();

        // Look for <ComponentName patterns
        let component_pattern = regex_lite::Regex::new(r"<([A-Z][a-zA-Z]+)").unwrap();

        for cap in component_pattern.captures_iter(&code) {
            if let Some(m) = cap.get(1) {
                let component_name = m.as_str();
                if let Some(section_id) = name_to_id.get(component_name) {
                    // Avoid duplicates
                    if !sections.contains(section_id) {
                        sections.push(section_id.clone());
                    }
                }
            }
        }

        if sections.is_empty() {
            return Ok(None);
        }

        Ok(Some(PageStructure {
            name,
            sections,
            source_path: path.to_owned(),
        }))
    }

    /// Get all cached analyses.
    #[must_use]
    pub const fn analyses(&self) -> &HashMap<String, TemplateAnalysis> {
        &self.analyses
    }

    /// Calculate aggregate statistics across all sections.
    #[must_use]
    pub fn aggregate_stats(&self) -> AggregateStats {
        let mut stats = AggregateStats::default();

        for analysis in self.analyses.values() {
            for section in &analysis.sections {
                stats.section_count += 1;
                stats.visual_weight_sum += section.style.visual_weight;
                stats.formality_sum += section.style.formality;
                stats.color_intensity_sum += section.style.color_intensity;
                stats.spacing_density_sum += section.style.spacing_density;

                *stats.section_types.entry(section.section_type).or_insert(0) += 1;
                *stats
                    .typography_scales
                    .entry(section.style.typography_scale)
                    .or_insert(0) += 1;
            }
        }

        stats
    }
}

/// Aggregate statistics from template analysis.
#[derive(Debug, Clone, Default)]
pub struct AggregateStats {
    /// Total number of sections analyzed
    pub section_count: usize,
    /// Sum of visual weights
    pub visual_weight_sum: f32,
    /// Sum of formality scores
    pub formality_sum: f32,
    /// Sum of color intensity scores
    pub color_intensity_sum: f32,
    /// Sum of spacing density scores
    pub spacing_density_sum: f32,
    /// Count by section type
    pub section_types: HashMap<SectionType, usize>,
    /// Count by typography scale
    pub typography_scales: HashMap<TypographyScale, usize>,
}

impl AggregateStats {
    /// Get average visual weight.
    #[must_use]
    pub fn avg_visual_weight(&self) -> f32 {
        if self.section_count == 0 {
            0.0
        } else {
            self.visual_weight_sum / self.section_count as f32
        }
    }

    /// Get average formality.
    #[must_use]
    pub fn avg_formality(&self) -> f32 {
        if self.section_count == 0 {
            0.0
        } else {
            self.formality_sum / self.section_count as f32
        }
    }

    /// Get average color intensity.
    #[must_use]
    pub fn avg_color_intensity(&self) -> f32 {
        if self.section_count == 0 {
            0.0
        } else {
            self.color_intensity_sum / self.section_count as f32
        }
    }

    /// Get average spacing density.
    #[must_use]
    pub fn avg_spacing_density(&self) -> f32 {
        if self.section_count == 0 {
            0.0
        } else {
            self.spacing_density_sum / self.section_count as f32
        }
    }
}

/// Errors that can occur during template analysis.
#[derive(Debug, Clone)]
pub enum AnalysisError {
    /// Invalid path provided
    InvalidPath,
    /// No sections directory found in template
    NoSectionsDir,
    /// IO error
    IoError(String),
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "invalid path"),
            Self::NoSectionsDir => write!(f, "no sections directory found"),
            Self::IoError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_type_from_name() {
        assert_eq!(
            SectionType::from_name("hero-centered-with-demo"),
            SectionType::Hero
        );
        assert_eq!(
            SectionType::from_name("features-two-column"),
            SectionType::Features
        );
        assert_eq!(
            SectionType::from_name("pricing-three-tier"),
            SectionType::Pricing
        );
        assert_eq!(
            SectionType::from_name("testimonial-grid"),
            SectionType::Testimonial
        );
        assert_eq!(SectionType::from_name("footer-simple"), SectionType::Footer);
        assert_eq!(
            SectionType::from_name("random-component"),
            SectionType::Unknown
        );
    }

    #[test]
    fn section_type_case_insensitive() {
        assert_eq!(SectionType::from_name("HERO-SECTION"), SectionType::Hero);
        assert_eq!(SectionType::from_name("Hero_Centered"), SectionType::Hero);
    }

    #[test]
    fn aggregate_stats_empty() {
        let stats = AggregateStats::default();
        assert_eq!(stats.avg_visual_weight(), 0.0);
        assert_eq!(stats.avg_formality(), 0.0);
    }
}
