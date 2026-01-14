//! Core library for draftkit
#![deny(unsafe_code)]

pub mod catalog;
pub mod catalyst;
pub mod components;
pub mod config;
pub mod docs;
pub mod elements;
pub mod error;

pub use catalog::Catalog;
pub use catalyst::{
    CatalystComponent, CatalystLanguage, get_all_components as get_all_catalyst_components,
    get_component as get_catalyst_component, get_component_metadata as get_catalyst_metadata,
    list_components as list_catalyst_components,
};
pub use components::{
    CategoryNode, Component, ComponentMeta, ComponentReader, ComponentRecord, ComponentResponse,
    Framework, Mode, NdjsonSnippet, SearchResult, Snippet, TailwindVersion,
};
pub use config::{Config, ConfigLoader, LogLevel};
pub use docs::{TopicInfo, get_docs, list_all_topics, list_topics, search_topics};
pub use elements::{ElementInfo, get_element_docs, get_full_docs, get_overview, list_elements};
pub use error::{ConfigError, ConfigResult};
