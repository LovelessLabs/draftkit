//! Component data types and access layer.
//!
//! This module provides type definitions for UI components and a reader
//! for accessing embedded component data.

mod reader;
mod types;

pub use reader::{ComponentReader, ComponentRecord, NdjsonSnippet};
pub use types::{
    CategoryNode, Component, ComponentMeta, ComponentResponse, Framework, Mode, SearchResult,
    Snippet, TailwindVersion,
};
