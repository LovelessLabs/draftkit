//! Presets command implementation
//!
//! Manages presets for draftkit. Presets are aesthetic overlays that modify
//! how patterns select components.

use clap::{Args, Subcommand};
use draftkit_core::patterns::{PatternLoader, PatternSource};
use draftkit_core::preset::{PresetFile, PresetLoader, PresetSource};
use serde::Serialize;
use std::path::PathBuf;

use crate::cli::Styler;

#[derive(Args)]
pub struct PresetsArgs {
    #[command(subcommand)]
    pub command: PresetsCommand,
}

#[derive(Subcommand)]
pub enum PresetsCommand {
    /// List available patterns and presets
    List(ListArgs),
    /// Set active preset stack for the current project
    Stack(StackArgs),
    /// Create a new preset
    Create(CreateArgs),
    /// Validate a pattern or preset file
    Validate(ValidateArgs),
    /// Install a preset from URL or registry
    Install(InstallArgs),
}

#[derive(Args)]
pub struct ListArgs {
    /// Show only patterns (not presets)
    #[arg(long)]
    pub patterns_only: bool,

    /// Show only presets (not patterns)
    #[arg(long)]
    pub presets_only: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct StackArgs {
    /// Preset names in stack order (first = base, last = highest priority)
    pub names: Vec<String>,

    /// Clear the preset stack
    #[arg(long)]
    pub clear: bool,

    /// Add a preset to the end of the stack
    #[arg(long)]
    pub add: Option<String>,

    /// Remove a preset from the stack
    #[arg(long)]
    pub remove: Option<String>,
}

#[derive(Args)]
pub struct CreateArgs {
    /// Name for the new preset
    pub name: String,

    /// Output directory (default: .draftkit/presets/)
    #[arg(long, short)]
    pub output: Option<PathBuf>,

    /// Overwrite existing file
    #[arg(long)]
    pub force: bool,
}

#[derive(Args)]
pub struct ValidateArgs {
    /// Path to the TOML file to validate
    pub file: PathBuf,

    /// Validate as a pattern file (default: auto-detect)
    #[arg(long)]
    pub pattern: bool,

    /// Validate as a preset file (default: auto-detect)
    #[arg(long)]
    pub preset: bool,
}

#[derive(Args)]
pub struct InstallArgs {
    /// Preset name from registry, or URL to TOML file
    pub source: String,

    /// Custom name for the installed preset (default: use preset's name)
    #[arg(long, short)]
    pub name: Option<String>,

    /// Install globally to user config (default: project-local)
    #[arg(long, short)]
    pub global: bool,

    /// Overwrite existing preset with same name
    #[arg(long)]
    pub force: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// JSON output structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PatternInfo {
    id: String,
    name: String,
    description: String,
    source: String,
    tags: Vec<String>,
    section_count: usize,
}

#[derive(Serialize)]
struct PresetInfo {
    name: String,
    version: String,
    description: String,
    source: String,
    extends: Option<String>,
    tags: Vec<String>,
    has_style_overrides: bool,
    variant_preferences_count: usize,
    blacklist_count: usize,
}

#[derive(Serialize)]
struct ListOutput {
    patterns: Vec<PatternInfo>,
    presets: Vec<PresetInfo>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Command implementations
// ─────────────────────────────────────────────────────────────────────────────

pub fn cmd_presets(args: PresetsArgs, styler: &Styler) -> anyhow::Result<()> {
    match args.command {
        PresetsCommand::List(list_args) => cmd_list(list_args, styler),
        PresetsCommand::Stack(stack_args) => cmd_stack(stack_args, styler),
        PresetsCommand::Create(create_args) => cmd_create(create_args, styler),
        PresetsCommand::Validate(validate_args) => cmd_validate(validate_args, styler),
        PresetsCommand::Install(install_args) => cmd_install(install_args, styler),
    }
}

fn cmd_list(args: ListArgs, styler: &Styler) -> anyhow::Result<()> {
    let pattern_loader = PatternLoader::new()?;
    let preset_loader = PresetLoader::new()?;

    let patterns: Vec<PatternInfo> = if args.presets_only {
        vec![]
    } else {
        pattern_loader
            .list_all()
            .iter()
            .map(|p| PatternInfo {
                id: p.pattern.id.clone(),
                name: p.pattern.name.clone(),
                description: p.pattern.description.clone(),
                source: format_pattern_source(p.source),
                tags: p.pattern.tags.clone(),
                section_count: p.pattern.sections.len(),
            })
            .collect()
    };

    let presets: Vec<PresetInfo> = if args.patterns_only {
        vec![]
    } else {
        preset_loader
            .list_all()
            .iter()
            .map(|p| PresetInfo {
                name: p.preset.name.clone(),
                version: p.preset.version.clone(),
                description: p.preset.description.clone(),
                source: format_preset_source(p.source),
                extends: p.preset.extends.clone(),
                tags: p.preset.tags.clone(),
                has_style_overrides: p.preset.style_overrides.has_overrides(),
                variant_preferences_count: p.preset.variant_preferences.len(),
                blacklist_count: p.preset.blacklist.components.len()
                    + p.preset.blacklist.tags.len()
                    + p.preset.blacklist.categories.len(),
            })
            .collect()
    };

    if args.json {
        let output = ListOutput { patterns, presets };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        // Text output
        if !patterns.is_empty() {
            styler.print_header("Patterns");
            println!();
            for p in &patterns {
                let source_tag = styler.dim().apply_to(format!("[{}]", p.source));
                let name = styler.bold().apply_to(&p.name);
                println!("  {name} {source_tag}");
                println!("    ID: {}", p.id);
                if !p.description.is_empty() {
                    println!("    {}", styler.dim().apply_to(&p.description));
                }
                println!("    Sections: {}", p.section_count);
                if !p.tags.is_empty() {
                    println!("    Tags: {}", p.tags.join(", "));
                }
                println!();
            }
        }

        if !presets.is_empty() {
            styler.print_header("Presets");
            println!();
            for p in &presets {
                let source_tag = styler.dim().apply_to(format!("[{}]", p.source));
                let name = styler.bold().apply_to(&p.name);
                let version = styler.dim().apply_to(format!("v{}", p.version));
                println!("  {name} {version} {source_tag}");
                if let Some(ref extends) = p.extends {
                    println!("    Extends: {extends}");
                }
                if !p.description.is_empty() {
                    println!("    {}", styler.dim().apply_to(&p.description));
                }
                let mut features = vec![];
                if p.has_style_overrides {
                    features.push("style overrides");
                }
                if p.variant_preferences_count > 0 {
                    features.push("variant preferences");
                }
                if p.blacklist_count > 0 {
                    features.push("blacklist");
                }
                if !features.is_empty() {
                    println!("    Features: {}", features.join(", "));
                }
                if !p.tags.is_empty() {
                    println!("    Tags: {}", p.tags.join(", "));
                }
                println!();
            }
        }

        if patterns.is_empty() && presets.is_empty() {
            styler.print_info("No patterns or presets found.");
        }
    }

    Ok(())
}

fn cmd_stack(args: StackArgs, styler: &Styler) -> anyhow::Result<()> {
    let mut loader = PresetLoader::new()?;

    if args.clear {
        loader.clear_active();
        styler.print_success("Cleared preset stack.");
        return Ok(());
    }

    if let Some(ref name) = args.remove {
        loader.deactivate(name);
        styler.print_success(&format!("Removed '{}' from preset stack.", name));
    }

    if let Some(ref name) = args.add {
        loader.activate(name)?;
        styler.print_success(&format!("Added '{}' to preset stack.", name));
    }

    if !args.names.is_empty() {
        loader.set_stack(args.names.clone())?;
        styler.print_success(&format!(
            "Set preset stack: {}",
            args.names.join(" → ")
        ));
    }

    // Show current stack
    let stack = loader.active_stack();
    if stack.is_empty() {
        styler.print_info("No presets active.");
    } else {
        println!();
        styler.print_info(&format!("Active stack: {}", stack.join(" → ")));
    }

    // Note: This is in-memory only for now
    println!();
    styler.print_warning(
        "Note: Stack changes are session-only. Project config persistence coming soon.",
    );

    Ok(())
}

fn cmd_create(args: CreateArgs, styler: &Styler) -> anyhow::Result<()> {
    let output_dir = args
        .output
        .unwrap_or_else(|| PathBuf::from(".draftkit/presets"));

    // Create output directory if needed
    std::fs::create_dir_all(&output_dir)?;

    let filename = format!("{}.toml", args.name.to_lowercase().replace(' ', "-"));
    let output_path = output_dir.join(&filename);

    if output_path.exists() && !args.force {
        anyhow::bail!(
            "File already exists: {}. Use --force to overwrite.",
            output_path.display()
        );
    }

    // Generate scaffold TOML
    let scaffold = format!(
        r#"# Preset: {name}
# Created by draftkit

[preset]
name = "{name}"
version = "1.0.0"
author = ""
description = ""
# extends = "Minimalist"  # Optional: inherit from another preset
tags = []

# Style constraints (all values 0.0-1.0)
# Uncomment and adjust to constrain component selection
[preset.style_overrides]
# visual_weight_max = 0.5
# visual_weight_min = 0.0
# formality_max = 1.0
# formality_min = 0.0
# color_intensity_max = 1.0
# color_intensity_min = 0.0
# spacing_density_max = 1.0
# spacing_density_min = 0.0
# typography_scales = ["small", "medium", "large"]

# Preferred component variants by section type
[preset.variant_preferences]
# hero = "hero-centered-cta"
# header = "header-simple-centered"
# footer = "footer-minimal"

# Components/tags/categories to exclude
[preset.blacklist]
components = []
tags = []
categories = []

# Components/tags to prefer
[preset.whitelist]
components = []
tags = []
"#,
        name = args.name
    );

    std::fs::write(&output_path, scaffold)?;
    styler.print_success(&format!("Created preset: {}", output_path.display()));

    println!();
    styler.print_info("Edit the file to customize your aesthetic preferences.");
    styler.print_info(&format!(
        "Add to stack with: draftkit presets stack --add \"{}\"",
        args.name
    ));

    Ok(())
}

fn cmd_validate(args: ValidateArgs, styler: &Styler) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(&args.file)?;

    // Auto-detect type if not specified
    let is_pattern = args.pattern || (!args.preset && content.contains("[pattern]"));
    let is_preset = args.preset || (!args.pattern && content.contains("[preset]"));

    if is_pattern {
        match toml::from_str::<draftkit_core::patterns::PatternFile>(&content) {
            Ok(parsed) => {
                styler.print_success(&format!(
                    "Valid pattern file: {} ({})",
                    parsed.pattern.name, parsed.pattern.id
                ));
                println!();
                println!("  Sections: {}", parsed.pattern.sections.len());
                println!("  Tags: {}", parsed.pattern.tags.join(", "));
            }
            Err(e) => {
                styler.print_error(&format!("Invalid pattern file: {e}"));
                return Err(e.into());
            }
        }
    } else if is_preset {
        match toml::from_str::<PresetFile>(&content) {
            Ok(parsed) => {
                styler.print_success(&format!(
                    "Valid preset: {} v{}",
                    parsed.preset.name, parsed.preset.version
                ));
                println!();
                if let Some(ref extends) = parsed.preset.extends {
                    println!("  Extends: {extends}");
                }
                if parsed.preset.style_overrides.has_overrides() {
                    println!("  Has style overrides");
                }
                if !parsed.preset.variant_preferences.is_empty() {
                    println!(
                        "  Variant preferences: {}",
                        parsed.preset.variant_preferences.len()
                    );
                }
                if !parsed.preset.blacklist.is_empty() {
                    println!("  Has blacklist rules");
                }
            }
            Err(e) => {
                styler.print_error(&format!("Invalid preset file: {e}"));
                return Err(e.into());
            }
        }
    } else {
        styler.print_error("Could not detect file type. Use --pattern or --preset to specify.");
        anyhow::bail!("Unknown file type");
    }

    Ok(())
}

fn cmd_install(args: InstallArgs, styler: &Styler) -> anyhow::Result<()> {
    // Determine if source is a URL or registry name
    let content = if args.source.starts_with("http://") || args.source.starts_with("https://") {
        // Direct URL - fetch via reqwest (blocking)
        styler.print_info(&format!("Fetching from {}...", args.source));

        let response = reqwest::blocking::get(&args.source)
            .map_err(|e| anyhow::anyhow!("Failed to fetch URL: {e}"))?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        response
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to read response: {e}"))?
    } else if args.source.starts_with('@') {
        // Scoped package from npm registry (e.g., @lovelesslabs/minimalist)
        // TODO: Implement npm registry fetch
        anyhow::bail!(
            "npm registry support coming soon.\n\
             For now, use a direct URL to the preset TOML file."
        );
    } else {
        // Registry name - construct URL from community repo
        let registry_url = format!(
            "https://raw.githubusercontent.com/lovelesslabs/draftkit-presets-community/main/presets/{}.toml",
            args.source.to_lowercase().replace(' ', "-")
        );

        styler.print_info(&format!(
            "Looking up '{}' in community registry...",
            args.source
        ));

        let response = reqwest::blocking::get(&registry_url)
            .map_err(|e| anyhow::anyhow!("Failed to fetch from registry: {e}"))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!(
                "Preset '{}' not found in community registry.\n\
                 Try providing a direct URL or check https://github.com/lovelesslabs/draftkit-presets-community",
                args.source
            );
        }

        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        response
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to read response: {e}"))?
    };

    // Validate the TOML
    let preset_file: PresetFile = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Invalid preset TOML: {e}"))?;

    // Determine output name
    let preset_name = args.name.unwrap_or_else(|| preset_file.preset.name.clone());
    let filename = format!("{}.toml", preset_name.to_lowercase().replace(' ', "-"));

    // Get target directory based on --global flag
    let presets_dir = if args.global {
        dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("draftkit")
            .join("presets")
    } else {
        PathBuf::from(".draftkit/presets")
    };

    // Create directory if needed
    std::fs::create_dir_all(&presets_dir)?;

    let output_path = presets_dir.join(&filename);

    // Check for existing file
    if output_path.exists() && !args.force {
        anyhow::bail!(
            "Preset already exists: {}\nUse --force to overwrite.",
            output_path.display()
        );
    }

    // Write the file
    std::fs::write(&output_path, &content)?;

    let scope = if args.global { "globally" } else { "locally" };
    styler.print_success(&format!(
        "Installed '{}' v{} {} to {}",
        preset_file.preset.name,
        preset_file.preset.version,
        scope,
        output_path.display()
    ));

    println!();
    styler.print_info(&format!(
        "Add to stack with: draftkit presets stack --add \"{}\"",
        preset_file.preset.name
    ));

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn format_pattern_source(source: PatternSource) -> String {
    match source {
        PatternSource::BuiltIn => "built-in".to_string(),
        PatternSource::User => "user".to_string(),
        PatternSource::Project => "project".to_string(),
    }
}

fn format_preset_source(source: PresetSource) -> String {
    match source {
        PresetSource::BuiltIn => "built-in".to_string(),
        PresetSource::User => "user".to_string(),
        PresetSource::Project => "project".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_command() {
        let args = ListArgs {
            patterns_only: false,
            presets_only: false,
            json: true,
        };
        let styler = Styler::new("never");
        assert!(cmd_list(args, &styler).is_ok());
    }

    #[test]
    fn test_list_patterns_only() {
        let args = ListArgs {
            patterns_only: true,
            presets_only: false,
            json: true,
        };
        let styler = Styler::new("never");
        assert!(cmd_list(args, &styler).is_ok());
    }

    #[test]
    fn test_list_presets_only() {
        let args = ListArgs {
            patterns_only: false,
            presets_only: true,
            json: true,
        };
        let styler = Styler::new("never");
        assert!(cmd_list(args, &styler).is_ok());
    }

    #[test]
    fn test_stack_nonexistent_preset() {
        let args = StackArgs {
            names: vec!["NonExistentPreset".to_string()],
            clear: false,
            add: None,
            remove: None,
        };
        let styler = Styler::new("never");
        assert!(cmd_stack(args, &styler).is_err());
    }

    #[test]
    fn test_stack_builtin_preset() {
        let args = StackArgs {
            names: vec!["Minimalist".to_string()],
            clear: false,
            add: None,
            remove: None,
        };
        let styler = Styler::new("never");
        assert!(cmd_stack(args, &styler).is_ok());
    }
}
