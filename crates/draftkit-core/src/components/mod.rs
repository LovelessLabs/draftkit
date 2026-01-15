//! Component data types and access layer.
//!
//! This module provides type definitions for UI components and a reader
//! for accessing embedded component data.

mod reader;
mod types;

pub use reader::{ComponentReader, ComponentRecord};
pub use types::{
    CategoryNode, Component, ComponentMeta, ComponentResponse, Framework, Mode, SearchResult,
    Snippet, TailwindVersion,
};

// Intelligence layer types (for pattern matching and coherence)
pub use types::{
    ComponentIntelligence, PagePosition, StyleProfile, TypographyScale, UsageContext,
};

// Extracted metadata types
pub use types::{DependencyInfo, ExtractedMeta, TailwindCompatibility, TokenInfo};
