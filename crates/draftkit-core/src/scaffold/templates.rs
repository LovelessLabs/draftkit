//! Template loading and rendering for project scaffolding.
//!
//! Templates are embedded at compile time using `include_str!` and rendered
//! with simple mustache-style substitution.

use std::collections::HashMap;
use std::fs;
use std::io;

use camino::Utf8PathBuf;
use thiserror::Error;

use super::{FrameworkTarget, ProjectConfig};

/// Template rendering error.
#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Unknown framework: {0}")]
    UnknownFramework(String),

    #[error("Template not found: {0}")]
    NotFound(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid template variable: {0}")]
    InvalidVariable(String),
}

/// A template file with its target path and content.
#[derive(Debug, Clone)]
pub struct TemplateFile {
    /// Relative path within the project
    pub path: Utf8PathBuf,
    /// Raw template content (before substitution)
    pub content: &'static str,
}

/// Template engine for rendering project scaffolds.
#[derive(Debug, Default)]
pub struct TemplateEngine {
    variables: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine from project config.
    #[must_use]
    pub fn from_config(config: &ProjectConfig) -> Self {
        let mut variables = HashMap::new();
        variables.insert("project_name".to_string(), config.name.clone());
        variables.insert(
            "tailwind_version".to_string(),
            config.tailwind_version.as_str().to_string(),
        );
        variables.insert("has_content".to_string(), "false".to_string());

        Self { variables }
    }

    /// Create a new template engine with custom variables.
    #[must_use]
    pub const fn with_variables(variables: HashMap<String, String>) -> Self {
        Self { variables }
    }

    /// Set a template variable.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    /// Get all template files for a framework.
    #[must_use]
    pub fn get_templates(framework: FrameworkTarget) -> Vec<TemplateFile> {
        match framework {
            FrameworkTarget::ViteReact => Self::vite_react_templates(),
            FrameworkTarget::Html => Self::html_templates(),
            FrameworkTarget::NextJs => Self::nextjs_templates(),
        }
    }

    /// Render a template string with variable substitution.
    #[must_use]
    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();

        // Handle simple conditionals: {{#if var}}...{{else}}...{{/if}} or {{#if var}}...{{/if}}
        result = self.process_conditionals(&result);

        // Simple mustache-style substitution: {{variable}}
        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{key}}}}}");
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// Process conditional blocks in templates.
    fn process_conditionals(&self, template: &str) -> String {
        let mut result = template.to_string();

        // Pattern: {{#if var}}true_content{{else}}false_content{{/if}}
        // or: {{#if var}}true_content{{/if}}
        loop {
            let Some(if_start) = result.find("{{#if ") else {
                break;
            };

            let Some(if_var_end) = result[if_start..].find("}}") else {
                break;
            };
            let if_var_end = if_start + if_var_end;

            let var_name = &result[if_start + 6..if_var_end];

            let Some(endif_pos) = result[if_var_end..].find("{{/if}}") else {
                break;
            };
            let endif_pos = if_var_end + endif_pos;

            let block_content = &result[if_var_end + 2..endif_pos];

            // Check for {{else}}
            let (true_content, false_content) = block_content
                .find("{{else}}")
                .map_or((block_content, ""), |else_pos| {
                    (&block_content[..else_pos], &block_content[else_pos + 8..])
                });

            // Evaluate the condition
            let var_value = self
                .variables
                .get(var_name)
                .map(String::as_str)
                .unwrap_or("false");
            let is_truthy = !var_value.is_empty() && var_value != "false" && var_value != "0";

            let replacement = if is_truthy {
                true_content
            } else {
                false_content
            };

            result = format!(
                "{}{}{}",
                &result[..if_start],
                replacement,
                &result[endif_pos + 7..]
            );
        }

        result
    }

    /// Scaffold a project to disk.
    pub fn scaffold(&self, config: &ProjectConfig) -> Result<Vec<Utf8PathBuf>, TemplateError> {
        let templates = Self::get_templates(config.framework);
        let mut created_files = Vec::new();

        // Create project directory
        fs::create_dir_all(&config.path)?;

        for template in templates {
            let target_path = config.path.join(&template.path);

            // Ensure parent directory exists
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Render and write
            let content = self.render(template.content);
            fs::write(&target_path, content)?;

            created_files.push(target_path);
        }

        Ok(created_files)
    }

    // -------------------------------------------------------------------------
    // Embedded templates per framework
    // -------------------------------------------------------------------------

    fn vite_react_templates() -> Vec<TemplateFile> {
        vec![
            TemplateFile {
                path: "package.json".into(),
                content: include_str!("templates/vite-react/package.json.tmpl"),
            },
            TemplateFile {
                path: "vite.config.ts".into(),
                content: include_str!("templates/vite-react/vite.config.ts.tmpl"),
            },
            TemplateFile {
                path: "tsconfig.json".into(),
                content: include_str!("templates/vite-react/tsconfig.json.tmpl"),
            },
            TemplateFile {
                path: "tsconfig.app.json".into(),
                content: include_str!("templates/vite-react/tsconfig.app.json.tmpl"),
            },
            TemplateFile {
                path: "tsconfig.node.json".into(),
                content: include_str!("templates/vite-react/tsconfig.node.json.tmpl"),
            },
            TemplateFile {
                path: "index.html".into(),
                content: include_str!("templates/vite-react/index.html.tmpl"),
            },
            TemplateFile {
                path: "eslint.config.js".into(),
                content: include_str!("templates/vite-react/eslint.config.js.tmpl"),
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: include_str!("templates/vite-react/.gitignore.tmpl"),
            },
            TemplateFile {
                path: "src/main.tsx".into(),
                content: include_str!("templates/vite-react/src/main.tsx.tmpl"),
            },
            TemplateFile {
                path: "src/App.tsx".into(),
                content: include_str!("templates/vite-react/src/App.tsx.tmpl"),
            },
            TemplateFile {
                path: "src/index.css".into(),
                content: include_str!("templates/vite-react/src/index.css.tmpl"),
            },
            TemplateFile {
                path: "src/vite-env.d.ts".into(),
                content: include_str!("templates/vite-react/src/vite-env.d.ts.tmpl"),
            },
        ]
    }

    const fn html_templates() -> Vec<TemplateFile> {
        // HTML templates will be added post-MVP
        vec![]
    }

    const fn nextjs_templates() -> Vec<TemplateFile> {
        // Next.js templates will be added post-MVP
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8Path;
    use tempfile::TempDir;

    #[test]
    fn simple_substitution() {
        let mut engine = TemplateEngine::default();
        engine.set("name", "my-project");

        let result = engine.render("Hello, {{name}}!");
        assert_eq!(result, "Hello, my-project!");
    }

    #[test]
    fn multiple_substitutions() {
        let mut engine = TemplateEngine::default();
        engine.set("name", "test");
        engine.set("version", "1.0.0");

        let result = engine.render("{{name}} v{{version}} - {{name}}");
        assert_eq!(result, "test v1.0.0 - test");
    }

    #[test]
    fn conditional_truthy() {
        let mut engine = TemplateEngine::default();
        engine.set("has_content", "true");

        let result = engine.render("{{#if has_content}}YES{{else}}NO{{/if}}");
        assert_eq!(result, "YES");
    }

    #[test]
    fn conditional_falsy() {
        let mut engine = TemplateEngine::default();
        engine.set("has_content", "false");

        let result = engine.render("{{#if has_content}}YES{{else}}NO{{/if}}");
        assert_eq!(result, "NO");
    }

    #[test]
    fn conditional_missing() {
        let engine = TemplateEngine::default();

        let result = engine.render("{{#if missing}}YES{{else}}NO{{/if}}");
        assert_eq!(result, "NO");
    }

    #[test]
    fn conditional_no_else() {
        let mut engine = TemplateEngine::default();
        engine.set("show", "true");

        let result = engine.render("prefix{{#if show}}CONTENT{{/if}}suffix");
        assert_eq!(result, "prefixCONTENTsuffix");
    }

    #[test]
    fn vite_react_templates_exist() {
        let templates = TemplateEngine::get_templates(FrameworkTarget::ViteReact);
        assert!(!templates.is_empty());

        let paths: Vec<_> = templates.iter().map(|t| t.path.as_str()).collect();
        assert!(paths.contains(&"package.json"));
        assert!(paths.contains(&"src/App.tsx"));
        assert!(paths.contains(&"src/main.tsx"));
    }

    #[test]
    fn scaffold_creates_files() {
        let dir = TempDir::new().unwrap();
        let base_path = Utf8Path::from_path(dir.path()).unwrap();

        let config = ProjectConfig::new("test-project", base_path)
            .with_framework(FrameworkTarget::ViteReact);

        let engine = TemplateEngine::from_config(&config);
        let created = engine.scaffold(&config).unwrap();

        assert!(!created.is_empty());

        // Check package.json exists and contains project name
        let package_json = config.path.join("package.json");
        assert!(package_json.exists());

        let content = fs::read_to_string(&package_json).unwrap();
        assert!(content.contains("\"name\": \"test-project\""));
    }

    #[test]
    fn scaffold_creates_nested_dirs() {
        let dir = TempDir::new().unwrap();
        let base_path = Utf8Path::from_path(dir.path()).unwrap();

        let config =
            ProjectConfig::new("nested-test", base_path).with_framework(FrameworkTarget::ViteReact);

        let engine = TemplateEngine::from_config(&config);
        engine.scaffold(&config).unwrap();

        // Check src/ directory and files exist
        let src_dir = config.path.join("src");
        assert!(src_dir.exists());
        assert!(src_dir.join("App.tsx").exists());
        assert!(src_dir.join("main.tsx").exists());
    }
}
