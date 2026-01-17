//! Generate command implementation
//!
//! Generates pages from patterns with component assembly.
//!
//! ```bash
//! draftkit generate index --pattern saas-landing --preset Minimalist
//! ```

use anyhow::{Context, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;
use draftkit_core::{
    FrameworkTarget, GenerateOptions, Mode, PackageManager, PageGenerator, ProjectConfig,
    TailwindVersion, intelligence::PatternMatcher, patterns::PatternLoader,
};

use crate::cli::Styler;

#[derive(Args)]
pub struct GenerateArgs {
    /// Page name to generate (e.g., "index", "about", "pricing")
    pub page_name: String,

    /// Generate from pattern
    #[arg(long, short)]
    pub pattern: Option<String>,

    /// Apply preset aesthetic
    #[arg(long)]
    pub preset: Option<String>,

    /// Use recipe from JSON file
    #[arg(long)]
    pub recipe: Option<Utf8PathBuf>,

    /// Fill content slots (JSON string)
    #[arg(long)]
    pub slots: Option<String>,

    /// Theme mode
    #[arg(long, short, default_value = "light", value_parser = parse_mode)]
    pub mode: Mode,

    /// Output path (overrides framework default)
    #[arg(long, short)]
    pub output: Option<Utf8PathBuf>,
}

fn parse_mode(s: &str) -> Result<Mode, String> {
    match s.to_lowercase().as_str() {
        "light" => Ok(Mode::Light),
        "dark" => Ok(Mode::Dark),
        "system" => Ok(Mode::System),
        _ => Err(format!(
            "Unknown mode '{}'. Valid options: light, dark, system",
            s
        )),
    }
}

/// Generate a page from patterns
pub fn cmd_generate(args: GenerateArgs, styler: &Styler) -> Result<()> {
    // Determine project context
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let project_path = Utf8PathBuf::try_from(cwd).context("Path is not valid UTF-8")?;

    // Detect project configuration
    let config = detect_project_config(&project_path)?;

    styler.print_header("Generating page");
    println!();
    const KV_WIDTH: usize = 12;
    styler.print_kv("Page", &args.page_name, KV_WIDTH);
    styler.print_kv("Framework", config.framework.as_str(), KV_WIDTH);

    if let Some(ref pattern) = args.pattern {
        styler.print_kv("Pattern", pattern, KV_WIDTH);
    }
    if let Some(ref preset) = args.preset {
        styler.print_kv("Preset", preset, KV_WIDTH);
    }
    styler.print_kv("Mode", args.mode.as_str(), KV_WIDTH);
    println!();

    // Build generation options
    let mut options = GenerateOptions {
        mode: args.mode,
        pattern: args.pattern.clone(),
        preset: args.preset.clone(),
        output_path: args.output.clone(),
        ..Default::default()
    };

    // Parse slots if provided
    if let Some(ref slots_json) = args.slots {
        let slots: std::collections::HashMap<String, draftkit_core::SlotValue> =
            serde_json::from_str(slots_json).context("Invalid JSON in --slots")?;
        options.slots = slots;
    }

    // Generate the page
    let generator = PageGenerator::new();

    let page = if let Some(ref pattern_id) = args.pattern {
        generate_from_pattern(&generator, &config, pattern_id, &options, styler)?
    } else {
        // No pattern - generate placeholder
        let spinner = styler.spinner("Generating placeholder page...");
        let page = generator.generate_placeholder(&config);
        spinner.finish_with_message("Generated placeholder");
        page
    };

    // Write the page
    let spinner = styler.spinner("Writing page...");
    generator.write_page(&page)?;
    spinner.finish_with_message(format!("Wrote {}", page.path));

    // Report any dependencies that need to be installed
    if !page.dependencies.is_empty() {
        println!();
        styler.print_warning("Additional dependencies may be required:");
        for dep in &page.dependencies {
            println!("  - {dep}");
        }
        println!();

        let add_cmd = config.package_manager.add_cmd(
            &page
                .dependencies
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            false,
        );
        println!("Run: {}", add_cmd.join(" "));
    }

    println!();
    styler.print_success(&format!("Page '{}' generated!", args.page_name));

    Ok(())
}

/// Generate a page from a pattern
fn generate_from_pattern(
    generator: &PageGenerator,
    config: &ProjectConfig,
    pattern_id: &str,
    options: &GenerateOptions,
    styler: &Styler,
) -> Result<draftkit_core::GeneratedPage> {
    let spinner = styler.spinner(&format!("Loading pattern '{pattern_id}'..."));

    // Load the pattern
    let loader = PatternLoader::new()?;
    let loaded_pattern = loader.get(pattern_id).ok_or_else(|| {
        anyhow::anyhow!(
            "Pattern '{}' not found. Run 'draftkit presets list' to see available patterns.",
            pattern_id
        )
    })?;

    spinner.finish_with_message(format!(
        "Loaded '{}' ({} sections)",
        loaded_pattern.pattern.name,
        loaded_pattern.pattern.sections.len()
    ));

    // Generate recipe
    let spinner = styler.spinner("Generating recipe...");
    let matcher = PatternMatcher::new();
    let recipe = matcher.generate_recipe(&loaded_pattern.pattern, &Default::default());
    spinner.finish_with_message(format!(
        "Recipe: {} sections, coherence score: {:.2}",
        recipe.sections.len(),
        recipe.coherence.score
    ));

    // Generate page from recipe
    let spinner = styler.spinner("Assembling page...");
    let page = generator.generate_from_recipe(&recipe, config, options)?;
    spinner.finish_with_message("Page assembled");

    Ok(page)
}

/// Detect project configuration from the current directory
fn detect_project_config(project_path: &Utf8Path) -> Result<ProjectConfig> {
    // Check for package.json
    let package_json = project_path.join("package.json");
    if !package_json.exists() {
        bail!("No package.json found. Run 'draftkit init' first or cd into a project directory.");
    }

    // Read package.json to get project name
    let content = std::fs::read_to_string(&package_json)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    let name = json["name"].as_str().unwrap_or("project").to_string();

    // Detect framework from project structure
    let framework = detect_framework(project_path);

    // Detect package manager from lockfile
    let package_manager = PackageManager::detect(project_path, None);

    // Detect Tailwind version from package.json
    let tailwind_version = detect_tailwind_version(&json);

    // Build a config that points to the existing project
    let mut config = ProjectConfig::new(&name, project_path.parent().unwrap_or(project_path));
    config.path = project_path.to_path_buf();
    config.framework = framework;
    config.package_manager = package_manager;
    config.tailwind_version = tailwind_version;

    Ok(config)
}

/// Detect the framework from project structure
fn detect_framework(project_path: &Utf8Path) -> FrameworkTarget {
    // Check for Next.js
    if project_path.join("next.config.js").exists()
        || project_path.join("next.config.ts").exists()
        || project_path.join("next.config.mjs").exists()
    {
        return FrameworkTarget::NextJs;
    }

    // Check for Vite
    if project_path.join("vite.config.ts").exists() || project_path.join("vite.config.js").exists()
    {
        return FrameworkTarget::ViteReact;
    }

    // Check for src/App.tsx (React)
    if project_path.join("src/App.tsx").exists() || project_path.join("src/App.jsx").exists() {
        return FrameworkTarget::ViteReact;
    }

    // Default to HTML for simple projects
    FrameworkTarget::Html
}

/// Detect Tailwind version from package.json dependencies
fn detect_tailwind_version(json: &serde_json::Value) -> TailwindVersion {
    let deps = json
        .get("devDependencies")
        .or_else(|| json.get("dependencies"));

    if let Some(deps) = deps {
        if let Some(version) = deps.get("tailwindcss").and_then(|v| v.as_str()) {
            // Check if version starts with ^4, ~4, 4, etc.
            let version_clean = version.trim_start_matches(['^', '~', '>', '=', '<', ' ']);
            if version_clean.starts_with('4') {
                return TailwindVersion::V4;
            }
            if version_clean.starts_with('3') {
                return TailwindVersion::V3;
            }
        }
    }

    // Default to v4
    TailwindVersion::V4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mode_valid() {
        assert_eq!(parse_mode("light").unwrap(), Mode::Light);
        assert_eq!(parse_mode("dark").unwrap(), Mode::Dark);
        assert_eq!(parse_mode("system").unwrap(), Mode::System);
        assert_eq!(parse_mode("LIGHT").unwrap(), Mode::Light);
    }

    #[test]
    fn parse_mode_invalid() {
        assert!(parse_mode("invalid").is_err());
    }

    #[test]
    fn detect_tailwind_v4() {
        let json: serde_json::Value = serde_json::json!({
            "devDependencies": {
                "tailwindcss": "^4.0.0"
            }
        });
        assert_eq!(detect_tailwind_version(&json), TailwindVersion::V4);
    }

    #[test]
    fn detect_tailwind_v3() {
        let json: serde_json::Value = serde_json::json!({
            "devDependencies": {
                "tailwindcss": "^3.4.0"
            }
        });
        assert_eq!(detect_tailwind_version(&json), TailwindVersion::V3);
    }

    #[test]
    fn detect_tailwind_default() {
        let json: serde_json::Value = serde_json::json!({});
        assert_eq!(detect_tailwind_version(&json), TailwindVersion::V4);
    }
}
