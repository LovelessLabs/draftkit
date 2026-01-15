//! Pattern library for page archetypes.
//!
//! Patterns are declarative TOML files that describe page structures,
//! defining what sections a page needs and which component variants
//! work well together.
//!
//! # Pattern Sources (Precedence Order)
//!
//! Patterns are loaded from multiple directories. Later sources override
//! earlier ones with the same ID:
//!
//! 1. **Built-in** - Core patterns embedded in the binary
//! 2. **User** - `~/.config/draftkit/patterns/`
//! 3. **Project** - `./.draftkit/patterns/`
//!
//! # Example Pattern Usage
//!
//! ```rust,no_run
//! use draftkit_core::patterns::PatternLoader;
//!
//! let loader = PatternLoader::builtin_only();
//!
//! // Get a specific pattern
//! if let Some(saas) = loader.get("saas-landing") {
//!     println!("Pattern: {}", saas.pattern.name);
//!     for section in &saas.pattern.sections {
//!         println!("  - {} (required: {})", section.section_type, section.required);
//!     }
//! }
//!
//! // List all available patterns
//! for id in loader.list_ids() {
//!     println!("Available: {id}");
//! }
//! ```

mod loader;
mod schema;

pub use loader::{LoadedPattern, PatternError, PatternLoader, PatternSource};
pub use schema::{
    CompositionRules, FieldSpec, Pattern, PatternFile, RepeatCount, SectionSpec, SlotSchema,
    SlotSpec, SlotType, StyleConstraints, VariantSpec,
};
