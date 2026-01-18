//! Example: Analyze TailwindPlus templates from a page-centric perspective
//!
//! Run with: cargo run --example analyze_pages -p draftkit-core
//!
//! This analyzes all templates to understand:
//! - What page types each template has (home, blog, docs, pricing, etc.)
//! - What components are used for each page type
//! - Template strengths (what they're good at)

use camino::Utf8Path;
use draftkit_core::intelligence::{PageAnalyzer, PageType};
use std::collections::HashMap;

fn main() {
    let kits_dir = Utf8Path::new("cache/current/kits");

    if !kits_dir.exists() {
        eprintln!("Kits directory not found at: {kits_dir}");
        eprintln!("Make sure you've run the collector script to extract templates.");
        std::process::exit(1);
    }

    let mut analyzer = PageAnalyzer::new();
    let mut total_pages = 0;
    let mut total_components = 0;
    let mut page_type_stats: HashMap<PageType, Vec<String>> = HashMap::new();

    // Find all template directories
    let entries = std::fs::read_dir(kits_dir.as_std_path()).expect("Failed to read kits directory");

    println!("=== Page-Centric Template Analysis ===\n");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !path.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.'))
        {
            let utf8_path = camino::Utf8PathBuf::try_from(path).expect("Invalid UTF-8 path");
            let template_name = utf8_path.file_name().unwrap_or("unknown");

            match analyzer.analyze_template(&utf8_path) {
                Ok(analysis) => {
                    let page_count = analysis.pages.len();
                    let component_count = analysis.components.len();

                    total_pages += page_count;
                    total_components += component_count;

                    // Collect page types
                    let mut page_types: Vec<_> = analysis
                        .pages
                        .iter()
                        .map(|p| p.page_type)
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();
                    page_types.sort_by_key(|pt| pt.as_str());

                    // Track which templates have which page types
                    for pt in &page_types {
                        page_type_stats
                            .entry(*pt)
                            .or_default()
                            .push(template_name.to_string());
                    }

                    let page_type_strs: Vec<_> = page_types.iter().map(|pt| pt.as_str()).collect();

                    println!("📁 {template_name}");
                    println!("   Pages: {page_count}");
                    println!("   Components: {component_count}");
                    println!("   Page types: {}", page_type_strs.join(", "));

                    if !analysis.strengths.is_empty() {
                        let strengths: Vec<_> =
                            analysis.strengths.iter().map(|s| s.as_str()).collect();
                        println!("   Strengths: {}", strengths.join(", "));
                    }

                    // Show some sample pages
                    if !analysis.pages.is_empty() {
                        println!("   Sample pages:");
                        for page in analysis.pages.iter().take(5) {
                            let component_count = page.components.len();
                            println!(
                                "     {} ({}) - {} components",
                                page.route,
                                page.page_type.as_str(),
                                component_count
                            );
                        }
                        if analysis.pages.len() > 5 {
                            println!("     ... and {} more", analysis.pages.len() - 5);
                        }
                    }

                    println!();
                }
                Err(e) => {
                    println!("📁 {template_name}");
                    println!("   ⚠️  Error: {e}\n");
                }
            }
        }
    }

    // Print summary
    println!("=== Summary ===\n");
    println!("Total templates analyzed: {}", analyzer.analyses().len());
    println!("Total pages found: {total_pages}");
    println!("Total components indexed: {total_components}");

    println!("\n=== Page Types by Template ===\n");
    let mut sorted_types: Vec<_> = page_type_stats.into_iter().collect();
    sorted_types.sort_by_key(|(pt, _)| pt.as_str());

    for (page_type, templates) in sorted_types {
        if page_type != PageType::Unknown {
            println!("{}: {}", page_type.as_str(), templates.join(", "));
        }
    }

    println!("\n=== Cross-Template Insights ===\n");

    // Find templates that could complement each other
    let analyses = analyzer.analyses();

    // Find which template is best for each page type
    let mut best_for_type: HashMap<PageType, (String, usize)> = HashMap::new();
    for (name, analysis) in analyses {
        for page in &analysis.pages {
            let count = analysis
                .pages
                .iter()
                .filter(|p| p.page_type == page.page_type)
                .count();
            let entry = best_for_type.entry(page.page_type).or_insert((name.clone(), 0));
            if count > entry.1 {
                *entry = (name.clone(), count);
            }
        }
    }

    println!("Best template for each page type:");
    let mut sorted_best: Vec<_> = best_for_type.into_iter().collect();
    sorted_best.sort_by_key(|(pt, _)| pt.as_str());

    for (page_type, (template, count)) in sorted_best {
        if page_type != PageType::Unknown && count > 0 {
            println!("  {}: {} ({} pages)", page_type.as_str(), template, count);
        }
    }

    println!("\n✓ Analysis complete");
}
