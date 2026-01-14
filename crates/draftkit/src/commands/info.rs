//! Info command implementation

use clap::Args;
use draftkit_core::{ComponentReader, Framework, get_manifest};
use serde::Serialize;

#[derive(Args)]
pub struct InfoArgs {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Serialize)]
struct PackageInfo {
    name: &'static str,
    version: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    description: &'static str,
}

impl PackageInfo {
    const fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
        }
    }
}

#[derive(Serialize)]
struct EmbeddedDataInfo {
    licensed_to: String,
    download_date: String,
    tailwind_version: String,
    elements_version: String,
    component_counts: ComponentCounts,
    template_count: usize,
}

#[derive(Serialize)]
struct ComponentCounts {
    react: usize,
    vue: usize,
    html: usize,
    total: usize,
}

#[derive(Serialize)]
struct FullInfo {
    #[serde(flatten)]
    package: PackageInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    embedded_data: Option<EmbeddedDataInfo>,
}

fn get_embedded_data_info() -> Option<EmbeddedDataInfo> {
    let manifest = get_manifest()?;
    let reader = ComponentReader::new();

    let react_count = reader.component_count(Framework::React);
    let vue_count = reader.component_count(Framework::Vue);
    let html_count = reader.component_count(Framework::Html);

    Some(EmbeddedDataInfo {
        licensed_to: manifest.downloaded_by.clone(),
        download_date: manifest.download_date().to_string(),
        tailwind_version: manifest.versions.tailwind.clone(),
        elements_version: manifest.versions.elements.clone(),
        component_counts: ComponentCounts {
            react: react_count,
            vue: vue_count,
            html: html_count,
            total: react_count + vue_count + html_count,
        },
        template_count: manifest.templates.len(),
    })
}

/// Print package information
pub fn cmd_info(args: InfoArgs) -> anyhow::Result<()> {
    let package = PackageInfo::new();
    let embedded_data = get_embedded_data_info();

    if args.json {
        let full_info = FullInfo {
            package,
            embedded_data,
        };
        println!("{}", serde_json::to_string_pretty(&full_info)?);
    } else {
        println!("{} {}", package.name, package.version);
        if !package.description.is_empty() {
            println!("{}", package.description);
        }
        println!();

        if let Some(data) = embedded_data {
            println!("Embedded Data:");
            println!("  TailwindPlus Account: {}", data.licensed_to);
            println!("  Download date:        {}", data.download_date);
            println!("  Tailwind:             v{}", data.tailwind_version);
            println!("  Elements:             v{}", data.elements_version);
            println!();
            println!("Component Counts:");
            println!("  React:  {:>4}", data.component_counts.react);
            println!("  Vue:    {:>4}", data.component_counts.vue);
            println!("  HTML:   {:>4}", data.component_counts.html);
            println!("  Total:  {:>4}", data.component_counts.total);
            println!();
            println!("Templates: {}", data.template_count);
        } else {
            println!("(No embedded data available)");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_info_text_succeeds() {
        assert!(cmd_info(InfoArgs { json: false }).is_ok());
    }

    #[test]
    fn test_cmd_info_json_succeeds() {
        assert!(cmd_info(InfoArgs { json: true }).is_ok());
    }
}
