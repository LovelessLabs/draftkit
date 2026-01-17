//! Project scaffolding for rapid site creation.
//!
//! This module provides:
//! - Package manager detection and command generation
//! - Framework templates (Vite+React, HTML, Next.js)
//! - Project initialization
//! - Page generation from patterns

mod framework;
mod generator;
mod package_manager;
mod templates;

pub use framework::{FrameworkTarget, ProjectConfig};
pub use generator::{GenerateError, GenerateOptions, GeneratedPage, PageGenerator, SlotValue};
pub use package_manager::PackageManager;
pub use templates::{TemplateEngine, TemplateError};
