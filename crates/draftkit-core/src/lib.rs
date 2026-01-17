//! Core library for draftkit
#![deny(unsafe_code)]

pub mod cache;
pub mod catalog;
pub mod catalyst;
pub mod components;
pub mod config;
pub mod data_dir;
pub mod docs;
pub mod elements;
pub mod error;
pub mod fetch;
pub mod intelligence;
pub mod manifest;
pub mod patterns;
pub mod preset;
pub mod scaffold;

pub use catalog::Catalog;
pub use catalyst::{
    CatalystComponent, CatalystLanguage, get_all_components as get_all_catalyst_components,
    get_component as get_catalyst_component, get_component_metadata as get_catalyst_metadata,
    list_components as list_catalyst_components,
};
pub use components::{
    CategoryNode, Component, ComponentMeta, ComponentReader, ComponentRecord, ComponentResponse,
    Framework, Mode, SearchResult, Snippet, TailwindVersion,
};
pub use config::{Config, ConfigLoader, LogLevel};
pub use data_dir::{DataSource, data_dir, has_runtime_data};
pub use docs::{TopicInfo, get_docs, list_all_topics, list_topics, search_topics};
pub use elements::{ElementInfo, get_element_docs, get_full_docs, get_overview, list_elements};
pub use error::{ConfigError, ConfigResult};
pub use manifest::{
    BuildInfo, Manifest, ManifestCounts, ManifestVersions, TemplateInfo, get_manifest,
    manifest_source,
};
pub use scaffold::{
    FrameworkTarget, GenerateError, GenerateOptions, GeneratedPage, PackageManager, PageGenerator,
    ProjectConfig, SlotValue, TemplateEngine, TemplateError,
};
