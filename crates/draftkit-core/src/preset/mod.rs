//! Preset system for aesthetic overlays.
//!
//! Presets are declarative TOML files that modify how patterns select
//! components. They act as aesthetic configurations that can be shared and stacked.
//!
//! # Preset Features
//!
//! - **Style overrides** - Constrain component selection by design DNA
//! - **Variant preferences** - Specify which component variants to prefer
//! - **Blacklists** - Exclude specific components, tags, or categories
//! - **Whitelists** - Prefer specific components or tags
//! - **Inheritance** - Presets can extend other presets via `extends`
//!
//! # Preset Sources (Precedence Order)
//!
//! Presets are loaded from multiple directories. Later sources override
//! earlier ones with the same name:
//!
//! 1. **Built-in** - Core presets embedded in the binary
//! 2. **User** - `~/.config/draftkit/presets/`
//! 3. **Project** - `./.draftkit/presets/`
//!
//! # Preset Stacking
//!
//! Multiple presets can be active simultaneously. The stack order is
//! user-controlled, with later presets in the stack taking precedence
//! for conflicting settings.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use draftkit_core::preset::PresetLoader;
//!
//! let mut loader = PresetLoader::builtin_only();
//!
//! // Set the stack order explicitly
//! loader.set_stack(vec![
//!     "Minimalist".to_string(),
//!     "Corporate".to_string(),
//! ]).unwrap();
//!
//! // Get merged settings (inheritance is resolved automatically)
//! let style_overrides = loader.merged_style_overrides();
//! let variant_prefs = loader.merged_variant_preferences();
//!
//! // Check blacklists
//! if loader.is_component_blacklisted("hero-video-background") {
//!     println!("Component blocked by active preset");
//! }
//!
//! // List available presets
//! for name in loader.list_names() {
//!     println!("Available: {name}");
//! }
//! ```
//!
//! # Example TOML Format
//!
//! ```toml
//! [preset]
//! name = "Corporate Dark"
//! version = "1.0.0"
//! extends = "Minimalist"  # inherit from another preset
//! description = "Professional design with dark theme"
//!
//! [preset.style_overrides]
//! formality_min = 0.8
//!
//! [preset.variant_preferences]
//! hero = "hero-centered-cta"
//! features = "feature-three-column-cards"
//!
//! [preset.blacklist]
//! components = ["hero-video-background"]
//! tags = ["animated", "video"]
//! ```

mod loader;
mod schema;

pub use loader::{LoadedPreset, PresetError, PresetLoader, PresetSource};
pub use schema::{Blacklist, Preset, PresetFile, StyleOverrides, Whitelist};
