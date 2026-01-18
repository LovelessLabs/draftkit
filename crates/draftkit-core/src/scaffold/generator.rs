//! Page generation from patterns and recipes.
//!
//! Pipeline:
//! ```text
//! Recipe
//!   ↓
//! For each section:
//!   → Match section to real component (ComponentMatcher)
//!   → Fetch component code (ComponentFetcher)
//!   → Fill slots (headlines, CTAs, features)
//!   → Transform for framework
//!   ↓
//! Assemble into page:
//!   → Generate imports
//!   → Concatenate sections
//!   → Wrap in layout
//!   ↓
//! Write to file
//! ```

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{FrameworkTarget, ProjectConfig};
use crate::components::{Framework, Mode};
use crate::fetch::{ComponentFetcher, FetchError};
use crate::intelligence::{ComponentMatcher, Recipe, RecipeSection};

/// Page generation error.
#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Failed to fetch component: {0}")]
    FetchError(#[from] FetchError),

    #[error("No matching component for section: {0}")]
    NoMatchingComponent(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Framework mismatch: expected {expected}, got {got}")]
    FrameworkMismatch { expected: String, got: String },
}

/// Slot value for content injection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlotValue {
    /// Simple text content
    Text(String),
    /// Image with src and alt
    Image { src: String, alt: String },
    /// Array of slot values (for features, nav items, etc.)
    Array(Vec<Self>),
    /// Nested object
    Object(HashMap<String, Self>),
}

impl SlotValue {
    /// Get as string if this is a text value.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }
}

impl From<&str> for SlotValue {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}

impl From<String> for SlotValue {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

/// Options for page generation.
#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    /// Pattern ID to generate from
    pub pattern: Option<String>,
    /// Preset to apply
    pub preset: Option<String>,
    /// Theme mode
    pub mode: Mode,
    /// Slot values to fill
    pub slots: HashMap<String, SlotValue>,
    /// Output path override
    pub output_path: Option<Utf8PathBuf>,
}

impl GenerateOptions {
    /// Create new options with a pattern.
    #[must_use]
    pub fn with_pattern(pattern: impl Into<String>) -> Self {
        Self {
            pattern: Some(pattern.into()),
            ..Default::default()
        }
    }

    /// Set the preset.
    #[must_use]
    pub fn with_preset(mut self, preset: impl Into<String>) -> Self {
        self.preset = Some(preset.into());
        self
    }

    /// Set the theme mode.
    #[must_use]
    pub const fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Add a slot value.
    #[must_use]
    pub fn with_slot(mut self, key: impl Into<String>, value: impl Into<SlotValue>) -> Self {
        self.slots.insert(key.into(), value.into());
        self
    }
}

/// Generated page content ready for writing.
#[derive(Debug, Clone)]
pub struct GeneratedPage {
    /// Page name/identifier
    pub name: String,
    /// Target file path
    pub path: Utf8PathBuf,
    /// Generated code content
    pub content: String,
    /// NPM dependencies required
    pub dependencies: Vec<String>,
    /// Dev dependencies required
    pub dev_dependencies: Vec<String>,
}

/// Page generator for assembling components into pages.
pub struct PageGenerator {
    /// Component fetcher for authenticated access (used in real component assembly).
    fetcher: Option<ComponentFetcher>,
    /// Component matcher for finding real components from section types.
    matcher: ComponentMatcher,
}

impl PageGenerator {
    /// Create a new page generator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fetcher: None,
            matcher: ComponentMatcher::react(),
        }
    }

    /// Create a generator with a component fetcher for authenticated access.
    #[must_use]
    pub fn with_fetcher(fetcher: ComponentFetcher) -> Self {
        Self {
            fetcher: Some(fetcher),
            matcher: ComponentMatcher::react(),
        }
    }

    /// Create a generator with a specific framework's matcher.
    #[must_use]
    pub fn for_framework(framework: Framework) -> Self {
        Self {
            fetcher: None,
            matcher: ComponentMatcher::new(crate::components::ComponentReader::new(), framework),
        }
    }

    /// Check if this generator has a fetcher for real component access.
    #[must_use]
    pub const fn has_fetcher(&self) -> bool {
        self.fetcher.is_some()
    }

    /// Generate a page from a recipe.
    ///
    /// This is the main entry point for page generation. It:
    /// 1. Fetches component code for each section
    /// 2. Fills slots with provided values or defaults
    /// 3. Assembles into a complete page
    pub fn generate_from_recipe(
        &self,
        recipe: &Recipe,
        config: &ProjectConfig,
        options: &GenerateOptions,
    ) -> Result<GeneratedPage, GenerateError> {
        let framework = self.framework_from_target(config.framework);
        let mut sections_code = Vec::new();
        let mut all_dependencies = Vec::new();
        let mut imports = Vec::new();

        for section in &recipe.sections {
            // Generate component name from section
            let component_name = self.section_to_component_name(&section.section_type);

            // Generate placeholder code for MVP (real fetch would use self.fetcher)
            let section_code =
                self.generate_section_placeholder(section, &component_name, framework, options);

            // Collect imports
            imports.push(format!(
                "// Section: {} (variant: {})",
                section.section_type, section.variant_id
            ));

            sections_code.push(section_code);
        }

        // Assemble final page
        let content = self.assemble_react_page(&config.name, &imports, &sections_code);

        // Collect dependencies from recipe
        all_dependencies.extend(recipe.dependencies.clone());

        // Determine output path
        let path = options
            .output_path
            .clone()
            .unwrap_or_else(|| config.path.join(config.framework.main_source_path()));

        Ok(GeneratedPage {
            name: "index".to_string(),
            path,
            content,
            dependencies: all_dependencies,
            dev_dependencies: vec![],
        })
    }

    /// Generate a simple placeholder page (for when no recipe is provided).
    #[must_use]
    pub fn generate_placeholder(&self, config: &ProjectConfig) -> GeneratedPage {
        let content = match config.framework {
            FrameworkTarget::ViteReact => self.react_placeholder(&config.name),
            FrameworkTarget::Html => self.html_placeholder(&config.name),
            FrameworkTarget::NextJs => self.nextjs_placeholder(&config.name),
        };

        let path = config.path.join(config.framework.main_source_path());

        GeneratedPage {
            name: "index".to_string(),
            path,
            content,
            dependencies: vec![],
            dev_dependencies: vec![],
        }
    }

    /// Write a generated page to disk.
    pub fn write_page(&self, page: &GeneratedPage) -> Result<(), GenerateError> {
        if let Some(parent) = page.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&page.path, &page.content)?;
        Ok(())
    }

    /// Generate a page from a recipe with real component fetching.
    ///
    /// This async method:
    /// 1. Matches each section to real components in the catalog
    /// 2. Fetches actual component source code via authenticated TailwindPlus access
    /// 3. Extracts imports and transforms code for the target framework
    /// 4. Assembles into a complete page
    ///
    /// Requires a `ComponentFetcher` to be configured (use `with_fetcher()`).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No fetcher is configured
    /// - A section has no matching component in the catalog
    /// - Component fetching fails (auth, network, etc.)
    pub async fn generate_from_recipe_async(
        &self,
        recipe: &Recipe,
        config: &ProjectConfig,
        options: &GenerateOptions,
    ) -> Result<GeneratedPage, GenerateError> {
        let fetcher = self
            .fetcher
            .as_ref()
            .ok_or_else(|| GenerateError::ComponentNotFound("No fetcher configured".to_string()))?;

        let framework = self.framework_from_target(config.framework);
        let mut sections_code = Vec::new();
        let mut all_dependencies = HashSet::new();
        let mut all_imports = HashSet::new();

        for section in &recipe.sections {
            // Match section to real components
            let recommendations =
                self.matcher
                    .match_section(&section.section_type, &section.variant_id, 1);

            let recommendation = recommendations.first().ok_or_else(|| {
                GenerateError::NoMatchingComponent(format!(
                    "{} (variant: {})",
                    section.section_type, section.variant_id
                ))
            })?;

            // Find the full component record to get UUID and category path
            let reader = crate::components::ComponentReader::new();
            let component = reader
                .find_by_id(framework, &recommendation.id)
                .ok_or_else(|| GenerateError::ComponentNotFound(recommendation.id.clone()))?;

            // Fetch the real component code
            let code = fetcher
                .fetch_component(
                    &component.uuid,
                    &component.category,
                    &component.subcategory,
                    &component.sub_subcategory,
                    framework,
                    options.mode,
                )
                .await?;

            // Parse and transform the fetched code
            let (section_imports, section_body) = self.parse_component_code(&code, framework);

            // Collect imports (deduplicated)
            all_imports.extend(section_imports);

            // Add section comment and body
            let section_code = format!(
                "      {{/* {} - {} */}}\n{}",
                recommendation.name, section.section_type, section_body
            );
            sections_code.push(section_code);

            // Extract dependencies from component metadata
            if let Some(meta) = &component.meta {
                for dep in &meta.dependencies.packages {
                    all_dependencies.insert(dep.clone());
                }
            }
        }

        // Assemble final page
        let imports_vec: Vec<String> = all_imports.into_iter().collect();
        let content = self.assemble_react_page_with_imports(&imports_vec, &sections_code);

        // Collect dependencies
        let dependencies: Vec<String> = all_dependencies.into_iter().collect();

        // Determine output path
        let path = options
            .output_path
            .clone()
            .unwrap_or_else(|| config.path.join(config.framework.main_source_path()));

        Ok(GeneratedPage {
            name: "index".to_string(),
            path,
            content,
            dependencies,
            dev_dependencies: vec![],
        })
    }

    /// Get component recommendations for a recipe without fetching code.
    ///
    /// Useful for previewing what components would be used before generation.
    #[must_use]
    pub fn get_component_recommendations(
        &self,
        recipe: &Recipe,
    ) -> Vec<crate::intelligence::ComponentRecommendation> {
        recipe
            .sections
            .iter()
            .filter_map(|section| {
                self.matcher
                    .match_section(&section.section_type, &section.variant_id, 1)
                    .into_iter()
                    .next()
            })
            .collect()
    }

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

    /// Parse component code to extract imports and body.
    ///
    /// Returns (imports, body) where imports are React import statements
    /// and body is the JSX content to embed in the page.
    fn parse_component_code(&self, code: &str, _framework: Framework) -> (Vec<String>, String) {
        let mut imports = Vec::new();
        let mut body_lines = Vec::new();
        let mut in_component = false;
        let mut brace_depth: u32 = 0;

        for line in code.lines() {
            let trimmed = line.trim();

            // Collect import statements
            if trimmed.starts_with("import ") {
                // Skip React import (will be at page level)
                if !trimmed.contains("from 'react'") && !trimmed.contains("from \"react\"") {
                    imports.push(line.to_string());
                }
                continue;
            }

            // Track when we enter the component function body
            if trimmed.starts_with("export default function")
                || trimmed.starts_with("function ")
                || trimmed.starts_with("export function")
            {
                in_component = true;
                continue;
            }

            // Track braces to know when we're in the return statement
            if in_component {
                for ch in line.chars() {
                    match ch {
                        '{' => brace_depth += 1,
                        '}' => brace_depth = brace_depth.saturating_sub(1),
                        _ => {}
                    }
                }

                // Capture lines that are part of the JSX return
                // Skip the return statement itself
                if !trimmed.starts_with("return") && brace_depth > 0 {
                    body_lines.push(format!("      {}", line.trim_start()));
                }
            }
        }

        // If we didn't find a proper component structure, just use the whole code
        let body = if body_lines.is_empty() {
            format!("      {}", code.replace('\n', "\n      "))
        } else {
            body_lines.join("\n")
        };

        (imports, body)
    }

    /// Assemble a React page with proper imports.
    fn assemble_react_page_with_imports(&self, imports: &[String], sections: &[String]) -> String {
        let imports_str = if imports.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", imports.join("\n"))
        };

        let sections_jsx = sections.join("\n\n");

        format!(
            r#"{imports_str}export default function App() {{
  return (
    <div className="min-h-screen bg-white">
{sections_jsx}
    </div>
  )
}}
"#
        )
    }

    const fn framework_from_target(&self, target: FrameworkTarget) -> Framework {
        match target {
            FrameworkTarget::Html => Framework::Html,
            FrameworkTarget::ViteReact | FrameworkTarget::NextJs => Framework::React,
        }
    }

    fn section_to_component_name(&self, section_type: &str) -> String {
        // Convert section type to PascalCase component name
        section_type
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                chars.next().map_or_else(String::new, |c| {
                    c.to_uppercase().collect::<String>() + chars.as_str()
                })
            })
            .collect()
    }

    fn generate_section_placeholder(
        &self,
        section: &RecipeSection,
        component_name: &str,
        _framework: Framework,
        options: &GenerateOptions,
    ) -> String {
        // Fill slots from options or section defaults
        let headline = options
            .slots
            .get("headline")
            .and_then(SlotValue::as_text)
            .or_else(|| section.slots.get("headline").map(String::as_str))
            .unwrap_or("Welcome");

        let subheadline = options
            .slots
            .get("subheadline")
            .and_then(SlotValue::as_text)
            .or_else(|| section.slots.get("subheadline").map(String::as_str))
            .unwrap_or("Build something amazing.");

        // Generate placeholder JSX for this section
        format!(
            r#"      {{/* {component_name} Section - {variant_id} */}}
      <section className="py-16 px-4">
        <div className="max-w-7xl mx-auto">
          <h2 className="text-3xl font-bold text-gray-900">{headline}</h2>
          <p className="mt-4 text-lg text-gray-600">{subheadline}</p>
        </div>
      </section>"#,
            variant_id = section.variant_id
        )
    }

    fn assemble_react_page(
        &self,
        _project_name: &str,
        imports: &[String],
        sections: &[String],
    ) -> String {
        let imports_comment = if imports.is_empty() {
            String::new()
        } else {
            format!("{}\n\n", imports.join("\n"))
        };

        let sections_jsx = sections.join("\n\n");

        format!(
            r#"{imports_comment}export default function App() {{
  return (
    <div className="min-h-screen bg-white">
{sections_jsx}
    </div>
  )
}}
"#
        )
    }

    fn react_placeholder(&self, project_name: &str) -> String {
        format!(
            r#"export default function App() {{
  return (
    <div className="min-h-screen bg-white">
      <div className="mx-auto max-w-7xl px-4 py-24 sm:px-6 lg:px-8">
        <div className="text-center">
          <h1 className="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
            Welcome to {project_name}
          </h1>
          <p className="mt-6 text-lg text-gray-600">
            Your site is ready. Run <code className="rounded bg-gray-100 px-2 py-1 font-mono text-sm">draftkit generate</code> to add components.
          </p>
        </div>
      </div>
    </div>
  )
}}
"#
        )
    }

    fn html_placeholder(&self, project_name: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{project_name}</title>
  <link href="./output.css" rel="stylesheet">
</head>
<body class="min-h-screen bg-white">
  <div class="mx-auto max-w-7xl px-4 py-24 sm:px-6 lg:px-8">
    <div class="text-center">
      <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
        Welcome to {project_name}
      </h1>
      <p class="mt-6 text-lg text-gray-600">
        Your site is ready. Run <code class="rounded bg-gray-100 px-2 py-1 font-mono text-sm">draftkit generate</code> to add components.
      </p>
    </div>
  </div>
</body>
</html>
"#
        )
    }

    fn nextjs_placeholder(&self, project_name: &str) -> String {
        format!(
            r#"export default function Page() {{
  return (
    <div className="min-h-screen bg-white">
      <div className="mx-auto max-w-7xl px-4 py-24 sm:px-6 lg:px-8">
        <div className="text-center">
          <h1 className="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
            Welcome to {project_name}
          </h1>
          <p className="mt-6 text-lg text-gray-600">
            Your site is ready. Run <code className="rounded bg-gray-100 px-2 py-1 font-mono text-sm">draftkit generate</code> to add components.
          </p>
        </div>
      </div>
    </div>
  )
}}
"#
        )
    }
}

impl Default for PageGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8Path;
    use tempfile::TempDir;

    #[test]
    fn slot_value_text() {
        let slot = SlotValue::from("hello");
        assert_eq!(slot.as_text(), Some("hello"));
    }

    #[test]
    fn slot_value_image() {
        let slot = SlotValue::Image {
            src: "/img.png".to_string(),
            alt: "test".to_string(),
        };
        assert!(slot.as_text().is_none());
    }

    #[test]
    fn generate_options_builder() {
        let opts = GenerateOptions::with_pattern("saas-landing")
            .with_preset("Minimalist")
            .with_mode(Mode::Dark)
            .with_slot("headline", "Hello World");

        assert_eq!(opts.pattern, Some("saas-landing".to_string()));
        assert_eq!(opts.preset, Some("Minimalist".to_string()));
        assert_eq!(opts.mode, Mode::Dark);
        assert!(opts.slots.contains_key("headline"));
    }

    #[test]
    fn section_to_component_name() {
        let generator = PageGenerator::new();

        assert_eq!(generator.section_to_component_name("hero"), "Hero");
        assert_eq!(
            generator.section_to_component_name("social-proof"),
            "SocialProof"
        );
        assert_eq!(generator.section_to_component_name("cta"), "Cta");
    }

    #[test]
    fn generate_placeholder_react() {
        let dir = TempDir::new().unwrap();
        let base_path = Utf8Path::from_path(dir.path()).unwrap();

        let config =
            ProjectConfig::new("test-app", base_path).with_framework(FrameworkTarget::ViteReact);

        let generator = PageGenerator::new();
        let page = generator.generate_placeholder(&config);

        assert!(page.content.contains("test-app"));
        assert!(page.content.contains("export default function App"));
        assert!(page.path.ends_with("src/App.tsx"));
    }

    #[test]
    fn generate_placeholder_html() {
        let dir = TempDir::new().unwrap();
        let base_path = Utf8Path::from_path(dir.path()).unwrap();

        let config =
            ProjectConfig::new("test-html", base_path).with_framework(FrameworkTarget::Html);

        let generator = PageGenerator::new();
        let page = generator.generate_placeholder(&config);

        assert!(page.content.contains("test-html"));
        assert!(page.content.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn write_page_creates_file() {
        let dir = TempDir::new().unwrap();
        let path = Utf8Path::from_path(dir.path()).unwrap();
        let file_path = path.join("src/App.tsx");

        let page = GeneratedPage {
            name: "test".to_string(),
            path: file_path.clone(),
            content: "// test content".to_string(),
            dependencies: vec![],
            dev_dependencies: vec![],
        };

        let generator = PageGenerator::new();
        generator.write_page(&page).unwrap();

        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "// test content");
    }
}
