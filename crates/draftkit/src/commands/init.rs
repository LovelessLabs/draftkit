//! Init command implementation
//!
//! Scaffolds a new project with Tailwind CSS and optional pattern generation.
//!
//! ```bash
//! draftkit init my-site --framework vite-react --pattern saas-landing
//! ```

use std::process::Command;

use anyhow::{Context, Result, bail};
use camino::Utf8PathBuf;
use clap::Args;
use draftkit_core::{
    FrameworkTarget, PackageManager, ProjectConfig, TailwindVersion, TemplateEngine,
    intelligence::PatternMatcher, patterns::PatternLoader,
};

use crate::cli::Styler;

#[derive(Args)]
pub struct InitArgs {
    /// Project name (directory name)
    pub name: String,

    /// Target framework
    #[arg(long, short, default_value = "vite-react", value_parser = parse_framework)]
    pub framework: FrameworkTarget,

    /// Package manager (auto-detect, npm, pnpm, yarn, bun)
    #[arg(long, short = 'm', value_parser = parse_package_manager)]
    pub package_manager: Option<PackageManager>,

    /// Start with a page pattern (e.g., saas-landing)
    #[arg(long, short)]
    pub pattern: Option<String>,

    /// Apply aesthetic preset
    #[arg(long)]
    pub preset: Option<String>,

    /// Tailwind CSS version
    #[arg(long, default_value = "v4", value_parser = parse_tailwind_version)]
    pub tailwind: TailwindVersion,

    /// Skip running package install
    #[arg(long)]
    pub skip_install: bool,

    /// Accept defaults non-interactively
    #[arg(long, short)]
    pub yes: bool,
}

fn parse_framework(s: &str) -> Result<FrameworkTarget, String> {
    FrameworkTarget::parse(s).ok_or_else(|| {
        format!(
            "Unknown framework '{}'. Valid options: html, vite-react, nextjs",
            s
        )
    })
}

fn parse_package_manager(s: &str) -> Result<PackageManager, String> {
    PackageManager::parse(s).ok_or_else(|| {
        format!(
            "Unknown package manager '{}'. Valid options: npm, pnpm, yarn, bun",
            s
        )
    })
}

fn parse_tailwind_version(s: &str) -> Result<TailwindVersion, String> {
    TailwindVersion::parse(s)
        .ok_or_else(|| format!("Unknown Tailwind version '{}'. Valid options: v3, v4", s))
}

/// Initialize a new project
pub fn cmd_init(args: InitArgs, styler: &Styler) -> Result<()> {
    // Determine base directory (current working directory)
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let base_dir = Utf8PathBuf::try_from(cwd).context("Path is not valid UTF-8")?;

    // Check if directory already exists
    let project_path = base_dir.join(&args.name);
    if project_path.exists() {
        bail!(
            "Directory '{}' already exists. Choose a different name or remove it first.",
            args.name
        );
    }

    // Detect or use specified package manager
    let package_manager = args
        .package_manager
        .unwrap_or_else(|| PackageManager::detect(&base_dir, None));

    styler.print_header("Creating new project");
    println!();
    const KV_WIDTH: usize = 16;
    styler.print_kv("Name", &args.name, KV_WIDTH);
    styler.print_kv("Framework", args.framework.as_str(), KV_WIDTH);
    styler.print_kv("Package Manager", package_manager.as_str(), KV_WIDTH);
    styler.print_kv("Tailwind", args.tailwind.as_str(), KV_WIDTH);

    if let Some(ref pattern) = args.pattern {
        styler.print_kv("Pattern", pattern, KV_WIDTH);
    }
    if let Some(ref preset) = args.preset {
        styler.print_kv("Preset", preset, KV_WIDTH);
    }
    println!();

    // Build project configuration
    let mut config = ProjectConfig::new(&args.name, &base_dir)
        .with_framework(args.framework)
        .with_package_manager(package_manager)
        .with_tailwind_version(args.tailwind);

    if let Some(ref pattern) = args.pattern {
        config = config.with_pattern(pattern);
    }
    if let Some(ref preset) = args.preset {
        config = config.with_preset(preset);
    }
    if args.skip_install {
        config = config.skip_install();
    }

    // Scaffold the project
    let spinner = styler.spinner("Scaffolding project...");
    let engine = TemplateEngine::from_config(&config);
    let created_files = engine.scaffold(&config)?;
    spinner.finish_with_message(format!("Created {} files", created_files.len()));

    // Generate initial page from pattern if specified
    if let Some(ref pattern_id) = args.pattern {
        generate_initial_page(&config, pattern_id, styler)?;
    }

    // Run package install unless skipped
    if !args.skip_install {
        run_install(&config, styler)?;
    }

    // Print success and next steps
    println!();
    styler.print_success(&format!("Project '{}' created successfully!", args.name));
    println!();
    println!("Next steps:");
    println!("  cd {}", args.name);

    if args.skip_install {
        let install_cmd = package_manager.install_cmd().join(" ");
        println!("  {}", install_cmd);
    }

    let dev_cmd = package_manager.dev_cmd().join(" ");
    println!("  {}", dev_cmd);
    println!();

    let port = args.framework.default_port();
    styler.print_info(&format!(
        "Your site will be available at http://localhost:{port}"
    ));

    Ok(())
}

/// Generate the initial page from a pattern
fn generate_initial_page(config: &ProjectConfig, pattern_id: &str, styler: &Styler) -> Result<()> {
    let spinner = styler.spinner(&format!("Generating page from '{pattern_id}' pattern..."));

    // Load the pattern
    let loader = PatternLoader::new()?;
    let loaded_pattern = loader
        .get(pattern_id)
        .ok_or_else(|| anyhow::anyhow!("Pattern '{}' not found", pattern_id))?;

    // Generate recipe from pattern
    let matcher = PatternMatcher::new();
    let recipe = matcher.generate_recipe(&loaded_pattern.pattern, &Default::default());

    // Generate page content
    let generator = draftkit_core::PageGenerator::new();
    let page = generator.generate_from_recipe(&recipe, config, &Default::default())?;

    // Write the page
    generator.write_page(&page)?;

    spinner.finish_with_message(format!(
        "Generated page with {} sections",
        recipe.sections.len()
    ));

    Ok(())
}

/// Run package manager install
fn run_install(config: &ProjectConfig, styler: &Styler) -> Result<()> {
    let spinner = styler.spinner(&format!(
        "Installing dependencies with {}...",
        config.package_manager
    ));

    let install_cmd = config.package_manager.install_cmd();
    let status = Command::new(install_cmd[0])
        .args(&install_cmd[1..])
        .current_dir(&config.path)
        .status()
        .context("Failed to run package manager")?;

    if status.success() {
        spinner.finish_with_message("Dependencies installed");
        Ok(())
    } else {
        spinner.finish_with_message("Install failed");
        bail!(
            "Package install failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_framework_valid() {
        assert_eq!(
            parse_framework("vite-react").unwrap(),
            FrameworkTarget::ViteReact
        );
        assert_eq!(
            parse_framework("react").unwrap(),
            FrameworkTarget::ViteReact
        );
        assert_eq!(parse_framework("html").unwrap(), FrameworkTarget::Html);
        assert_eq!(parse_framework("nextjs").unwrap(), FrameworkTarget::NextJs);
    }

    #[test]
    fn parse_framework_invalid() {
        assert!(parse_framework("invalid").is_err());
    }

    #[test]
    fn parse_package_manager_valid() {
        assert_eq!(parse_package_manager("npm").unwrap(), PackageManager::Npm);
        assert_eq!(parse_package_manager("pnpm").unwrap(), PackageManager::Pnpm);
        assert_eq!(parse_package_manager("yarn").unwrap(), PackageManager::Yarn);
        assert_eq!(parse_package_manager("bun").unwrap(), PackageManager::Bun);
    }

    #[test]
    fn parse_tailwind_version_valid() {
        assert_eq!(parse_tailwind_version("v3").unwrap(), TailwindVersion::V3);
        assert_eq!(parse_tailwind_version("v4").unwrap(), TailwindVersion::V4);
        assert_eq!(parse_tailwind_version("4").unwrap(), TailwindVersion::V4);
    }
}
