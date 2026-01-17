//! Framework targets and project configuration.
//!
//! Framework tiers:
//! | Framework    | Build Step       | Complexity   | Target User       |
//! |--------------|------------------|--------------|-------------------|
//! | `html`       | Tailwind CLI     | Simplest     | Total beginners   |
//! | `vite-react` | Vite             | Modern       | Most users        |
//! | `nextjs`     | Next.js          | Full-featured| Apps with routing |

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use super::PackageManager;
use crate::components::TailwindVersion;

/// Framework target for project scaffolding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FrameworkTarget {
    /// Plain HTML with Tailwind CLI
    Html,
    /// Vite + React (TypeScript)
    #[default]
    ViteReact,
    /// Next.js App Router
    NextJs,
}

impl FrameworkTarget {
    /// Get the string representation.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::ViteReact => "vite-react",
            Self::NextJs => "nextjs",
        }
    }

    /// Parse from string (case-insensitive).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "html" => Some(Self::Html),
            "vite-react" | "vitereact" | "vite" | "react" => Some(Self::ViteReact),
            "nextjs" | "next" => Some(Self::NextJs),
            _ => None,
        }
    }

    /// Whether this framework uses TypeScript.
    #[must_use]
    pub const fn uses_typescript(&self) -> bool {
        match self {
            Self::Html => false,
            Self::ViteReact | Self::NextJs => true,
        }
    }

    /// Whether this framework requires a build step.
    #[must_use]
    pub const fn requires_build(&self) -> bool {
        match self {
            Self::Html => false, // Tailwind CLI watches
            Self::ViteReact | Self::NextJs => true,
        }
    }

    /// Default port for dev server.
    #[must_use]
    pub const fn default_port(&self) -> u16 {
        match self {
            Self::Html => 3000,      // Simple HTTP server
            Self::ViteReact => 5173, // Vite default
            Self::NextJs => 3000,    // Next.js default
        }
    }

    /// Get the main source file path for this framework.
    #[must_use]
    pub const fn main_source_path(&self) -> &'static str {
        match self {
            Self::Html => "index.html",
            Self::ViteReact => "src/App.tsx",
            Self::NextJs => "app/page.tsx",
        }
    }

    /// Get the main page/route path for generated content.
    #[must_use]
    pub const fn page_path(&self, _page_name: &str) -> &'static str {
        // For MVP, we only generate the index page
        match self {
            Self::Html => "index.html",
            Self::ViteReact => "src/App.tsx",
            Self::NextJs => "app/page.tsx",
        }
    }
}

impl std::fmt::Display for FrameworkTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for FrameworkTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| format!("Unknown framework: {s}"))
    }
}

/// Configuration for a scaffolded project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name (directory name)
    pub name: String,
    /// Project directory path
    pub path: Utf8PathBuf,
    /// Selected framework
    pub framework: FrameworkTarget,
    /// Selected package manager
    pub package_manager: PackageManager,
    /// Tailwind CSS version
    pub tailwind_version: TailwindVersion,
    /// Initial pattern to apply
    pub pattern: Option<String>,
    /// Initial preset to apply
    pub preset: Option<String>,
    /// Skip running package install
    pub skip_install: bool,
}

impl ProjectConfig {
    /// Create a new project configuration.
    pub fn new(name: impl Into<String>, base_dir: &Utf8Path) -> Self {
        let name = name.into();
        let path = base_dir.join(&name);

        Self {
            name,
            path,
            framework: FrameworkTarget::default(),
            package_manager: PackageManager::default(),
            tailwind_version: TailwindVersion::default(),
            pattern: None,
            preset: None,
            skip_install: false,
        }
    }

    /// Set the framework target.
    #[must_use]
    pub const fn with_framework(mut self, framework: FrameworkTarget) -> Self {
        self.framework = framework;
        self
    }

    /// Set the package manager.
    #[must_use]
    pub const fn with_package_manager(mut self, pm: PackageManager) -> Self {
        self.package_manager = pm;
        self
    }

    /// Set the Tailwind version.
    #[must_use]
    pub const fn with_tailwind_version(mut self, version: TailwindVersion) -> Self {
        self.tailwind_version = version;
        self
    }

    /// Set the initial pattern.
    #[must_use]
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Set the initial preset.
    #[must_use]
    pub fn with_preset(mut self, preset: impl Into<String>) -> Self {
        self.preset = Some(preset.into());
        self
    }

    /// Skip package installation.
    #[must_use]
    pub const fn skip_install(mut self) -> Self {
        self.skip_install = true;
        self
    }

    /// Get the path to package.json.
    #[must_use]
    pub fn package_json_path(&self) -> Utf8PathBuf {
        self.path.join("package.json")
    }

    /// Get the path to the main source file.
    #[must_use]
    pub fn main_source_path(&self) -> Utf8PathBuf {
        self.path.join(self.framework.main_source_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framework_parse() {
        assert_eq!(
            FrameworkTarget::parse("vite-react"),
            Some(FrameworkTarget::ViteReact)
        );
        assert_eq!(
            FrameworkTarget::parse("VITE-REACT"),
            Some(FrameworkTarget::ViteReact)
        );
        assert_eq!(
            FrameworkTarget::parse("react"),
            Some(FrameworkTarget::ViteReact)
        );
        assert_eq!(FrameworkTarget::parse("html"), Some(FrameworkTarget::Html));
        assert_eq!(
            FrameworkTarget::parse("nextjs"),
            Some(FrameworkTarget::NextJs)
        );
        assert_eq!(
            FrameworkTarget::parse("next"),
            Some(FrameworkTarget::NextJs)
        );
        assert_eq!(FrameworkTarget::parse("invalid"), None);
    }

    #[test]
    fn framework_properties() {
        assert!(!FrameworkTarget::Html.uses_typescript());
        assert!(FrameworkTarget::ViteReact.uses_typescript());
        assert!(FrameworkTarget::NextJs.uses_typescript());

        assert!(!FrameworkTarget::Html.requires_build());
        assert!(FrameworkTarget::ViteReact.requires_build());
        assert!(FrameworkTarget::NextJs.requires_build());
    }

    #[test]
    fn project_config_builder() {
        let config = ProjectConfig::new("my-site", Utf8Path::new("/tmp"))
            .with_framework(FrameworkTarget::ViteReact)
            .with_package_manager(PackageManager::Pnpm)
            .with_pattern("saas-landing");

        assert_eq!(config.name, "my-site");
        assert_eq!(config.path.as_str(), "/tmp/my-site");
        assert_eq!(config.framework, FrameworkTarget::ViteReact);
        assert_eq!(config.package_manager, PackageManager::Pnpm);
        assert_eq!(config.pattern, Some("saas-landing".to_string()));
    }

    #[test]
    fn serde_roundtrip() {
        let json = serde_json::to_string(&FrameworkTarget::ViteReact).unwrap();
        assert_eq!(json, "\"vite-react\"");

        let parsed: FrameworkTarget = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, FrameworkTarget::ViteReact);
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", FrameworkTarget::Html), "html");
        assert_eq!(format!("{}", FrameworkTarget::ViteReact), "vite-react");
        assert_eq!(format!("{}", FrameworkTarget::NextJs), "nextjs");
    }
}
