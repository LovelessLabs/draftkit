//! Example: Analyze TailwindPlus templates and generate intelligence data
//!
//! Run with: cargo run --example analyze_template
//! Or with output: cargo run --example analyze_template -- --output data/component-intelligence.json

use camino::Utf8Path;
use draftkit_core::intelligence::{IntelligenceBuilder, TemplateAnalyzer};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_path = args
        .iter()
        .position(|a| a == "--output")
        .and_then(|i| args.get(i + 1))
        .map(|s| Utf8Path::new(s));

    let kits_dir = Utf8Path::new("cache/current/kits");

    if !kits_dir.exists() {
        eprintln!("Kits directory not found at: {kits_dir}");
        eprintln!("Make sure you've run the collector script to extract templates.");
        std::process::exit(1);
    }

    let mut analyzer = TemplateAnalyzer::new();
    let mut builder = IntelligenceBuilder::new();

    // Find all extracted template directories (not .zip files)
    let entries = std::fs::read_dir(kits_dir.as_std_path()).expect("Failed to read kits directory");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !path.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.')) {
            let utf8_path = camino::Utf8PathBuf::try_from(path).expect("Invalid UTF-8 path");
            println!("Analyzing: {}", utf8_path.file_name().unwrap_or("unknown"));

            match analyzer.analyze_template(&utf8_path) {
                Ok(analysis) => {
                    println!(
                        "  → {} sections, {} pages",
                        analysis.sections.len(),
                        analysis.pages.len()
                    );
                    builder.add_template(analysis);
                }
                Err(e) => {
                    eprintln!("  → Error: {e}");
                }
            }
        }
    }

    // Print summary
    let stats = analyzer.aggregate_stats();
    println!("\n=== Aggregate Stats ===");
    println!("Total sections: {}", stats.section_count);
    println!("Avg visual weight: {:.2}", stats.avg_visual_weight());
    println!("Avg formality: {:.2}", stats.avg_formality());
    println!("Avg color intensity: {:.2}", stats.avg_color_intensity());
    println!("Avg spacing density: {:.2}", stats.avg_spacing_density());

    // Write output if requested
    if let Some(path) = output_path {
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent.as_std_path()).ok();
        }

        match builder.write_to_file(path) {
            Ok(()) => println!("\n✓ Wrote intelligence data to: {path}"),
            Err(e) => eprintln!("\n✗ Failed to write output: {e}"),
        }
    } else {
        println!("\nTip: Use --output <path> to write component-intelligence.json");
    }
}
