//! Intelligence layer for design system validation.
//!
//! This module provides the "magic" that transforms draftkit from a
//! component access tool into a design system intelligence layer.
//!
//! # Components
//!
//! - **Coherence Checker**: Validates that components work well together
//!   visually, detecting jarring combinations based on `StyleProfile` data.
//!
//! # Example
//!
//! ```rust
//! use draftkit_core::intelligence::CoherenceChecker;
//! use draftkit_core::components::{StyleProfile, TypographyScale};
//!
//! let checker = CoherenceChecker::new();
//!
//! let header = StyleProfile {
//!     visual_weight: 0.3,
//!     formality: 0.8,
//!     color_intensity: 0.4,
//!     spacing_density: 0.6,
//!     typography_scale: TypographyScale::Medium,
//! };
//!
//! let hero = StyleProfile {
//!     visual_weight: 0.4,
//!     formality: 0.75,
//!     color_intensity: 0.5,
//!     spacing_density: 0.55,
//!     typography_scale: TypographyScale::Medium,
//! };
//!
//! let score = checker.check_compatibility(&header, &hero);
//! println!("Compatibility: {:.0}%", score.score * 100.0);
//!
//! if !score.is_compatible() {
//!     for issue in &score.issues {
//!         println!("Issue: {}", issue.message);
//!     }
//! }
//! ```

mod coherence;
mod component_matcher;
mod intelligence_writer;
mod matcher;
mod page_analyzer;
mod site_intelligence;
mod style_extractor;
mod template_analyzer;

pub use coherence::{
    CoherenceChecker, CoherenceIssue, CompatibilityScore, IssueCategory, PageCoherence,
};
pub use component_matcher::{ComponentMatcher, ComponentRecommendation};
pub use intelligence_writer::IntelligenceBuilder;
pub use matcher::{
    PatternMatcher, Recipe, RecipeOptions, RecipeSection, SectionSuggestion, StylePreference,
};
pub use page_analyzer::{
    ComponentAnalysis, ComponentUsage, LayoutAnalysis, PageAnalysis, PageAnalysisError,
    PageAnalyzer, PageType, TemplatePageAnalysis,
};
pub use site_intelligence::{
    CrossTemplateComponent, SiteIntelligence, SiteRecommendation, TemplateRanking,
};
pub use style_extractor::StyleExtractor;
pub use template_analyzer::{
    AggregateStats, AnalysisError, PageStructure, SectionAnalysis, SectionType, TemplateAnalysis,
    TemplateAnalyzer,
};
