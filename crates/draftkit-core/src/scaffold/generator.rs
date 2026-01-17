//! Page generation from patterns and recipes.
//!
//! Pipeline:
//! ```text
//! Recipe
//!   ↓
//! For each section:
//!   → Fetch component code
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

use std::collections::HashMap;
use std::fs;
use std::io;

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{FrameworkTarget, ProjectConfig};
use crate::components::{Framework, Mode};
use crate::fetch::ComponentFetcher;
use crate::intelligence::{Recipe, RecipeSection};

/// Page generation error.
#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Failed to fetch component: {0}")]
    FetchError(String),

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
    #[allow(dead_code)]
    fetcher: Option<ComponentFetcher>,
}

impl PageGenerator {
    /// Create a new page generator.
    #[must_use]
    pub const fn new() -> Self {
        Self { fetcher: None }
    }

    /// Create a generator with a component fetcher for authenticated access.
    #[must_use]
    pub const fn with_fetcher(fetcher: ComponentFetcher) -> Self {
        Self {
            fetcher: Some(fetcher),
        }
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

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

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
