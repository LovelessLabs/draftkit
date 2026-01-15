//! Divan benchmarks for draftkit-core
//!
//! Wall-clock time benchmarks for fast local iteration.
//! Run with: `cargo bench --bench divan_benchmarks`
//!
//! AUTO-GENERATED from crates/draftkit-core/benches/benchmarks.kdl.
//! Do not edit directly. Run `cargo xtask gen-benchmarks` to regenerate.
//!
//! See docs/benchmarks-howto.md for more information.

use std::hint::black_box;

use draftkit_core::config::{Config, ConfigLoader};

fn main() {
    divan::main();
}

mod config {
    use super::*;

    #[divan::bench]
    fn load_defaults() -> Config {
        black_box(
            ConfigLoader::new()
                .with_user_config(false)
                .without_boundary_marker()
                .load()
                .unwrap(),
        )
    }

    #[divan::bench]
    fn construct_loader() -> ConfigLoader {
        black_box(ConfigLoader::new())
    }
}

mod patterns {
    use super::*;

    #[divan::bench]
    fn load_builtin_patterns() -> usize {
        black_box(
            draftkit_core::patterns::PatternLoader::builtin_only()
                .list_all()
                .len(),
        )
    }
}

mod intelligence {
    use super::*;

    #[divan::bench]
    fn generate_recipe() -> draftkit_core::intelligence::Recipe {
        let loader = draftkit_core::patterns::PatternLoader::builtin_only();
        let pattern = loader.get("saas-landing").unwrap();
        let matcher = draftkit_core::intelligence::PatternMatcher::new();
        let opts = draftkit_core::intelligence::RecipeOptions::default();
        black_box(matcher.generate_recipe(&pattern.pattern, &opts))
    }

    #[divan::bench]
    fn coherence_check_pair() -> draftkit_core::intelligence::CompatibilityScore {
        let checker = draftkit_core::intelligence::CoherenceChecker::new();
        let a = draftkit_core::components::StyleProfile {
            visual_weight: 0.3,
            formality: 0.7,
            color_intensity: 0.4,
            spacing_density: 0.6,
            typography_scale: draftkit_core::components::TypographyScale::Medium,
        };
        let b = draftkit_core::components::StyleProfile {
            visual_weight: 0.4,
            formality: 0.8,
            color_intensity: 0.5,
            spacing_density: 0.55,
            typography_scale: draftkit_core::components::TypographyScale::Medium,
        };
        black_box(checker.check_compatibility(&a, &b))
    }

    #[divan::bench]
    fn coherence_check_page() -> draftkit_core::intelligence::PageCoherence {
        let checker = draftkit_core::intelligence::CoherenceChecker::new();
        let profiles: Vec<draftkit_core::components::StyleProfile> = (0..5)
            .map(|i| draftkit_core::components::StyleProfile {
                visual_weight: 0.3 + (i as f32 * 0.05),
                formality: 0.7,
                color_intensity: 0.4,
                spacing_density: 0.6,
                typography_scale: draftkit_core::components::TypographyScale::Medium,
            })
            .collect();
        let components: Vec<(&str, &draftkit_core::components::StyleProfile)> = vec![
            ("header", &profiles[0]),
            ("hero", &profiles[1]),
            ("features", &profiles[2]),
            ("pricing", &profiles[3]),
            ("footer", &profiles[4]),
        ];
        black_box(checker.check_page_coherence(&components))
    }

    #[divan::bench]
    fn suggest_next_section() -> Vec<draftkit_core::intelligence::SectionSuggestion> {
        let loader = draftkit_core::patterns::PatternLoader::builtin_only();
        let pattern = loader.get("saas-landing").unwrap();
        let matcher = draftkit_core::intelligence::PatternMatcher::new();
        let current = vec!["header".to_string(), "hero".to_string()];
        black_box(matcher.suggest_next_section(&pattern.pattern, &current))
    }
}
