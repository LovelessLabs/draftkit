//! Example: Get site recommendations for combining TailwindPlus templates
//!
//! Run with: cargo run --example site_recommendation -p draftkit-core
//!
//! This demonstrates how SiteIntelligence can recommend which templates
//! to use for different page types when building a multi-purpose site.

use camino::Utf8Path;
use draftkit_core::intelligence::{PageAnalyzer, PageType, SiteIntelligence};

fn main() {
    let kits_dir = Utf8Path::new("cache/current/kits");

    if !kits_dir.exists() {
        eprintln!("Kits directory not found at: {kits_dir}");
        eprintln!("Make sure you've run the collector script to extract templates.");
        std::process::exit(1);
    }

    // Analyze all templates
    let mut analyzer = PageAnalyzer::new();
    let entries = std::fs::read_dir(kits_dir.as_std_path()).expect("Failed to read kits directory");

    println!("Analyzing templates...\n");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !path.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.'))
        {
            let utf8_path = camino::Utf8PathBuf::try_from(path).expect("Invalid UTF-8 path");
            let _ = analyzer.analyze_template(&utf8_path);
        }
    }

    // Build site intelligence
    let site = SiteIntelligence::from_analyzer(&analyzer);

    println!("=== Site Intelligence ===\n");

    // Show best template for each page type
    println!("Best template for each page type:");
    for page_type in site.supported_page_types() {
        if let Some(ranking) = site.ranking_for(page_type) {
            print!("  {}: {} (score: {})", page_type.as_str(), ranking.best, ranking.score);
            if !ranking.alternatives.is_empty() {
                print!(" | also: {}", ranking.alternatives.join(", "));
            }
            println!();
        }
    }

    // Example: SaaS landing page site
    println!("\n=== Recommendation: SaaS Landing Page ===\n");
    let saas_rec = site.recommend_for_site(&[
        PageType::Home,
        PageType::Pricing,
        PageType::Blog,
        PageType::About,
        PageType::Contact,
        PageType::Legal,
    ]);

    println!("Template assignments:");
    for (page_type, template) in &saas_rec.template_assignments {
        println!("  {} → {}", page_type.as_str(), template);
    }
    println!("\nTemplates used: {}", saas_rec.template_count);
    if !saas_rec.shareable_components.is_empty() {
        println!("Shareable components: {}", saas_rec.shareable_components.join(", "));
    }
    println!("Style notes:");
    for note in &saas_rec.style_notes {
        println!("  • {note}");
    }

    // Example: Documentation site
    println!("\n=== Recommendation: Documentation Site ===\n");
    let docs_rec = site.recommend_for_site(&[
        PageType::Home,
        PageType::Docs,
        PageType::ApiReference,
        PageType::Changelog,
    ]);

    println!("Template assignments:");
    for (page_type, template) in &docs_rec.template_assignments {
        println!("  {} → {}", page_type.as_str(), template);
    }
    println!("\nTemplates used: {}", docs_rec.template_count);
    println!("Style notes:");
    for note in &docs_rec.style_notes {
        println!("  • {note}");
    }

    // Example: Portfolio site
    println!("\n=== Recommendation: Portfolio Site ===\n");
    let portfolio_rec = site.recommend_for_site(&[
        PageType::Home,
        PageType::Portfolio,
        PageType::About,
        PageType::Contact,
        PageType::Blog,
    ]);

    println!("Template assignments:");
    for (page_type, template) in &portfolio_rec.template_assignments {
        println!("  {} → {}", page_type.as_str(), template);
    }
    println!("\nTemplates used: {}", portfolio_rec.template_count);
    println!("Style notes:");
    for note in &portfolio_rec.style_notes {
        println!("  • {note}");
    }

    // Show cross-template components
    let cross = site.cross_template_components();
    if !cross.is_empty() {
        println!("\n=== Cross-Template Components ===\n");
        println!("Components that appear in multiple templates:");
        for comp in cross.iter().take(10) {
            println!(
                "  {} - in {} templates ({:?})",
                comp.name,
                comp.templates.len(),
                comp.templates
            );
        }
        if cross.len() > 10 {
            println!("  ... and {} more", cross.len() - 10);
        }
    }

    println!("\n✓ Recommendations complete");
}
