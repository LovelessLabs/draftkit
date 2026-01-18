//! Page-centric template analysis for cross-template intelligence.
//!
//! Analyzes TailwindPlus templates from a page-first perspective to learn:
//! - What components make up different page types (home, blog, docs, pricing)
//! - How to bridge styles across templates
//! - Which components work well together
//!
//! This enables combining templates: commit's blog style with oatmeal's pricing page.

use crate::components::StyleProfile;
use crate::intelligence::StyleExtractor;
use camino::{Utf8Path, Utf8PathBuf};
use regex_lite::Regex;
use std::collections::{HashMap, HashSet};

/// Page type classification based on route and content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageType {
    /// Home/landing page (index route)
    Home,
    /// About page (team, company info)
    About,
    /// Pricing page
    Pricing,
    /// Blog listing or article page
    Blog,
    /// Documentation page
    Docs,
    /// Contact page
    Contact,
    /// Legal pages (privacy, terms)
    Legal,
    /// Changelog/releases
    Changelog,
    /// API reference
    ApiReference,
    /// Portfolio/projects
    Portfolio,
    /// Authentication pages (login, register, signin)
    Auth,
    /// Dashboard/app pages (behind auth)
    Dashboard,
    /// Media pages (podcast episodes, videos)
    Media,
    /// Error pages (404, 500)
    Error,
    /// Resources/downloads page
    Resources,
    /// Generic content page
    Content,
    /// Unknown page type
    Unknown,
}

impl PageType {
    /// Infer page type from route path.
    #[must_use]
    pub fn from_route(route: &str) -> Self {
        let lower = route.to_lowercase();

        // Strip route groups like /(main), /(auth), /(centered) for better matching
        let normalized = Self::normalize_route(&lower);

        // Handle index routes (including bare "page" which represents the index)
        if normalized == "/" || normalized.is_empty() || normalized == "page" {
            return Self::Home;
        }

        // Match specific patterns first (more specific wins)
        // Error pages
        if normalized.contains("404") || normalized.contains("500") || normalized.contains("error")
        {
            return Self::Error;
        }

        // Auth pages
        if normalized.contains("login")
            || normalized.contains("signin")
            || normalized.contains("sign-in")
            || normalized.contains("register")
            || normalized.contains("signup")
            || normalized.contains("sign-up")
            || normalized.contains("forgot")
            || normalized.contains("reset-password")
        {
            return Self::Auth;
        }

        // Dashboard/app pages
        if normalized.contains("dashboard")
            || normalized.contains("settings")
            || normalized.contains("profile")
            || normalized.contains("account")
        {
            return Self::Dashboard;
        }

        // Media pages (podcast, video)
        if normalized.contains("episode")
            || normalized.contains("podcast")
            || normalized.contains("video")
            || normalized.contains("watch")
            || normalized.contains("listen")
        {
            return Self::Media;
        }

        // Resources
        if normalized.contains("resource") || normalized.contains("download") {
            return Self::Resources;
        }

        // Pricing
        if normalized.contains("pricing") || normalized.contains("plans") {
            return Self::Pricing;
        }

        // About
        if normalized.contains("about") || normalized.contains("team") {
            return Self::About;
        }

        // Blog
        if normalized.contains("blog")
            || normalized.contains("article")
            || normalized.contains("post")
            || normalized.contains("news")
        {
            return Self::Blog;
        }

        // Interview content (often blog-like)
        if normalized.contains("interview") {
            return Self::Blog;
        }

        // Docs
        if normalized.contains("doc") || normalized.contains("guide") || normalized.contains("help")
        {
            return Self::Docs;
        }

        // Contact
        if normalized.contains("contact") || normalized.contains("support") {
            return Self::Contact;
        }

        // Legal
        if normalized.contains("privacy")
            || normalized.contains("terms")
            || normalized.contains("legal")
            || normalized.contains("cookie")
        {
            return Self::Legal;
        }

        // Changelog
        if normalized.contains("changelog")
            || normalized.contains("release")
            || normalized.contains("what-new")
            || normalized.contains("whats-new")
        {
            return Self::Changelog;
        }

        // API reference
        if normalized.contains("api") || normalized.contains("reference") {
            return Self::ApiReference;
        }

        // Portfolio
        if normalized.contains("project")
            || normalized.contains("portfolio")
            || normalized.contains("work")
            || normalized.contains("case-stud")
        {
            return Self::Portfolio;
        }

        // Thank you / confirmation pages
        if normalized.contains("thank") || normalized.contains("success") {
            return Self::Content;
        }

        Self::Unknown
    }

    /// Normalize route by stripping Next.js route groups like /(main), /(auth).
    fn normalize_route(route: &str) -> String {
        // Remove route groups: /(...) patterns
        let mut result = route.to_string();

        // Strip patterns like "/(main)", "/(auth)", "/(centered)", "/(sidebar)"
        // Keep the leading slash but remove the group itself
        while let Some(start) = result.find("/(") {
            if let Some(end) = result[start..].find(')') {
                // Keep what's before the /( and add what's after the )
                let before = &result[..start];
                let after = &result[start + end + 1..];
                result = format!("{before}{after}");
            } else {
                break;
            }
        }

        // Clean up double slashes
        while result.contains("//") {
            result = result.replace("//", "/");
        }

        // Ensure we return at least "/" for empty results
        if result.is_empty() {
            "/".to_string()
        } else {
            result
        }
    }

    /// Get string representation.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::About => "about",
            Self::Pricing => "pricing",
            Self::Blog => "blog",
            Self::Docs => "docs",
            Self::Contact => "contact",
            Self::Legal => "legal",
            Self::Changelog => "changelog",
            Self::ApiReference => "api-reference",
            Self::Portfolio => "portfolio",
            Self::Auth => "auth",
            Self::Dashboard => "dashboard",
            Self::Media => "media",
            Self::Error => "error",
            Self::Resources => "resources",
            Self::Content => "content",
            Self::Unknown => "unknown",
        }
    }
}

/// A component used in a page.
#[derive(Debug, Clone)]
pub struct ComponentUsage {
    /// Component identifier (derived from import path or inline name)
    pub id: String,
    /// Import path (e.g., "@/components/sections/hero")
    pub import_path: Option<String>,
    /// Whether this is an inline component defined in the page file
    pub is_inline: bool,
    /// Extracted style profile (if component source is available)
    pub style: Option<StyleProfile>,
}

/// Analysis of a single page.
#[derive(Debug, Clone)]
pub struct PageAnalysis {
    /// Route path (e.g., "/pricing", "/blog/[slug]")
    pub route: String,
    /// Classified page type
    pub page_type: PageType,
    /// Components used in this page
    pub components: Vec<ComponentUsage>,
    /// Source file path
    pub source_path: Utf8PathBuf,
    /// Template this page belongs to
    pub template_name: String,
}

/// Analysis of a component across templates.
#[derive(Debug, Clone)]
pub struct ComponentAnalysis {
    /// Component identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Which templates contain this component
    pub templates: Vec<String>,
    /// Which page types use this component
    pub page_types: HashSet<PageType>,
    /// Extracted style profile
    pub style: StyleProfile,
    /// Source file path
    pub source_path: Utf8PathBuf,
}

/// Analysis of a template's layout structure.
#[derive(Debug, Clone)]
pub struct LayoutAnalysis {
    /// Layout identifier
    pub id: String,
    /// Components shared in this layout (header, footer, nav)
    pub shared_components: Vec<String>,
    /// Routes that use this layout
    pub routes: Vec<String>,
    /// Source file path
    pub source_path: Utf8PathBuf,
}

/// Complete analysis of a template.
#[derive(Debug, Clone)]
pub struct TemplatePageAnalysis {
    /// Template name (e.g., "oatmeal", "commit")
    pub name: String,
    /// Template root path
    pub path: Utf8PathBuf,
    /// All pages found in this template
    pub pages: Vec<PageAnalysis>,
    /// All components found
    pub components: HashMap<String, ComponentAnalysis>,
    /// Layout structures
    pub layouts: Vec<LayoutAnalysis>,
    /// Template's primary strengths (what page types it excels at)
    pub strengths: Vec<PageType>,
}

/// Page-centric template analyzer.
#[derive(Debug, Default)]
pub struct PageAnalyzer {
    /// Cached template analyses
    analyses: HashMap<String, TemplatePageAnalysis>,
    /// Regex for parsing imports
    import_regex: Option<Regex>,
    /// Regex for finding JSX component usage
    component_regex: Option<Regex>,
    /// Regex for MDX layout export pattern
    mdx_layout_regex: Option<Regex>,
    /// Regex for Markdoc component syntax: {% component-name %}
    markdoc_regex: Option<Regex>,
}

impl PageAnalyzer {
    /// Create a new page analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyses: HashMap::new(),
            import_regex: Regex::new(
                r#"import\s+\{?\s*([^}]+?)\s*\}?\s+from\s+['"]([^'"]+)['"]"#,
            )
            .ok(),
            component_regex: Regex::new(r"<([A-Z][a-zA-Z0-9]*)").ok(),
            // Match: export { Layout as default } from '@/components/Layout'
            mdx_layout_regex: Regex::new(
                r#"export\s+\{\s*(\w+)(?:\s+as\s+default)?\s*\}\s+from\s+['"]([^'"]+)['"]"#,
            )
            .ok(),
            // Match Markdoc syntax: {% component-name ... %}
            markdoc_regex: Regex::new(r"\{%\s*([a-zA-Z][a-zA-Z0-9-]*)\s").ok(),
        }
    }

    /// Analyze a template directory.
    ///
    /// Handles both direct template structure and nested `-ts`/`-js` variants.
    pub fn analyze_template(
        &mut self,
        path: &Utf8Path,
    ) -> Result<&TemplatePageAnalysis, PageAnalysisError> {
        let name = path
            .file_name()
            .ok_or(PageAnalysisError::InvalidPath)?
            .to_string();

        if self.analyses.contains_key(&name) {
            return Ok(self.analyses.get(&name).expect("just checked"));
        }

        // Find the actual source directory (handle -ts/-js variants)
        let source_root = self.find_source_root(path)?;

        let analysis = self.do_analyze(&name, path, &source_root)?;
        self.analyses.insert(name.clone(), analysis);

        Ok(self.analyses.get(&name).expect("just inserted"))
    }

    /// Find the source root directory, handling variant subdirectories.
    fn find_source_root(&self, path: &Utf8Path) -> Result<Utf8PathBuf, PageAnalysisError> {
        // Check for direct src directory
        let direct_src = path.join("src");
        if direct_src.exists() {
            return Ok(path.to_owned());
        }

        // Check for -ts variant (prefer TypeScript)
        let name = path.file_name().unwrap_or("template");
        let ts_variant = path.join(format!("{name}-ts"));
        if ts_variant.exists() {
            return Ok(ts_variant);
        }

        // Check for -js variant
        let js_variant = path.join(format!("{name}-js"));
        if js_variant.exists() {
            return Ok(js_variant);
        }

        // Check for any subdirectory with src
        if let Ok(entries) = std::fs::read_dir(path.as_std_path()) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let src_check = entry_path.join("src");
                    if src_check.exists() {
                        return Utf8PathBuf::try_from(entry_path)
                            .map_err(|_| PageAnalysisError::InvalidPath);
                    }
                }
            }
        }

        Err(PageAnalysisError::NoSourceDir)
    }

    /// Perform the actual analysis.
    fn do_analyze(
        &self,
        name: &str,
        template_path: &Utf8Path,
        source_root: &Utf8Path,
    ) -> Result<TemplatePageAnalysis, PageAnalysisError> {
        let mut pages = Vec::new();
        let mut components: HashMap<String, ComponentAnalysis> = HashMap::new();
        let mut layouts = Vec::new();

        // Find app directory (Next.js App Router)
        let app_dir = self.find_app_dir(source_root);

        // Analyze pages
        if let Some(ref app) = app_dir {
            self.find_pages(app, app, name, &mut pages, &mut components)?;

            // Analyze layouts
            self.find_layouts(app, &mut layouts)?;
        }

        // Analyze component library
        let components_dir = source_root.join("src/components");
        if components_dir.exists() {
            self.analyze_component_dir(&components_dir, name, &mut components)?;
        }

        // Determine template strengths based on page types
        let mut page_type_counts: HashMap<PageType, usize> = HashMap::new();
        for page in &pages {
            *page_type_counts.entry(page.page_type).or_insert(0) += 1;
        }

        // Sort by count to find strengths
        let mut strengths: Vec<_> = page_type_counts.into_iter().collect();
        strengths.sort_by(|a, b| b.1.cmp(&a.1));
        let strengths: Vec<PageType> = strengths
            .into_iter()
            .filter(|(pt, _)| *pt != PageType::Unknown)
            .map(|(pt, _)| pt)
            .take(3)
            .collect();

        Ok(TemplatePageAnalysis {
            name: name.to_string(),
            path: template_path.to_owned(),
            pages,
            components,
            layouts,
            strengths,
        })
    }

    /// Find the Next.js app directory.
    fn find_app_dir(&self, source_root: &Utf8Path) -> Option<Utf8PathBuf> {
        let candidates = [
            source_root.join("src/app"),
            source_root.join("app"),
        ];

        candidates.into_iter().find(|p| p.exists())
    }

    /// Recursively find and analyze page files.
    fn find_pages(
        &self,
        dir: &Utf8Path,
        app_root: &Utf8Path,
        template_name: &str,
        pages: &mut Vec<PageAnalysis>,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<(), PageAnalysisError> {
        let entries = std::fs::read_dir(dir.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // Skip special Next.js directories
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if dir_name.starts_with('_') || dir_name.starts_with('.') {
                    continue;
                }

                let utf8_path =
                    Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;
                self.find_pages(&utf8_path, app_root, template_name, pages, components)?;
            } else {
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Check for page files (TSX/JSX/JS)
                if file_name == "page.tsx"
                    || file_name == "page.jsx"
                    || file_name == "page.js"
                {
                    let utf8_path =
                        Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;

                    if let Some(analysis) =
                        self.analyze_page_file(&utf8_path, app_root, template_name, components)?
                    {
                        pages.push(analysis);
                    }
                }
                // Check for MDX page files (used by commit, protocol templates)
                else if file_name == "page.mdx" {
                    let utf8_path =
                        Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;

                    if let Some(analysis) =
                        self.analyze_mdx_page(&utf8_path, app_root, template_name, components)?
                    {
                        pages.push(analysis);
                    }
                }
                // Check for Markdoc page files (used by syntax template)
                else if file_name == "page.md" {
                    let utf8_path =
                        Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;

                    if let Some(analysis) =
                        self.analyze_markdoc_page(&utf8_path, app_root, template_name, components)?
                    {
                        pages.push(analysis);
                    }
                }
            }
        }

        Ok(())
    }

    /// Analyze a single page file.
    fn analyze_page_file(
        &self,
        path: &Utf8Path,
        app_root: &Utf8Path,
        template_name: &str,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<Option<PageAnalysis>, PageAnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        // Derive route from path relative to app root
        let route = path
            .parent()
            .and_then(|p| p.strip_prefix(app_root).ok())
            .map(|p| format!("/{}", p.as_str().replace('\\', "/")))
            .unwrap_or_else(|| "/".to_string());

        // Handle empty route or "app" route as home
        let route = if route == "/" || route == "/app" || route.is_empty() {
            "/".to_string()
        } else {
            route
        };

        let page_type = PageType::from_route(&route);

        // Parse imports
        let mut component_usages = Vec::new();
        let mut seen_components = HashSet::new();

        if let Some(ref import_re) = self.import_regex {
            for cap in import_re.captures_iter(&code) {
                let imports = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let import_path = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                // Skip non-component imports
                if !import_path.starts_with("@/components")
                    && !import_path.starts_with("./")
                    && !import_path.starts_with("../")
                {
                    continue;
                }

                // Parse individual imports from destructured import
                for import in imports.split(',') {
                    let import = import.trim();
                    // Handle renamed imports: "Foo as Bar"
                    let component_name = import.split(" as ").next().unwrap_or(import).trim();

                    if component_name.is_empty()
                        || !component_name
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_uppercase())
                    {
                        continue;
                    }

                    let id = self.derive_component_id(component_name, import_path);

                    if seen_components.insert(id.clone()) {
                        component_usages.push(ComponentUsage {
                            id: id.clone(),
                            import_path: Some(import_path.to_string()),
                            is_inline: false,
                            style: None,
                        });

                        // Track component usage
                        components
                            .entry(id.clone())
                            .or_insert_with(|| ComponentAnalysis {
                                id: id.clone(),
                                name: component_name.to_string(),
                                templates: Vec::new(),
                                page_types: HashSet::new(),
                                style: StyleProfile::default(),
                                source_path: Utf8PathBuf::new(),
                            })
                            .page_types
                            .insert(page_type);
                    }
                }
            }
        }

        // Find inline components (functions defined in the page file)
        if let Some(ref component_re) = self.component_regex {
            for cap in component_re.captures_iter(&code) {
                if let Some(m) = cap.get(1) {
                    let component_name = m.as_str();

                    // Skip already imported components and common React elements
                    let id = component_name.to_lowercase();
                    if seen_components.contains(&id)
                        || ["Fragment", "Suspense", "Image", "Link", "Head", "Script"]
                            .contains(&component_name)
                    {
                        continue;
                    }

                    // Check if it's defined inline (look for function ComponentName)
                    let inline_pattern = format!(r"function\s+{component_name}\s*\(");
                    let is_inline_component = code.contains(&inline_pattern)
                        || code.contains(&format!("const {component_name} ="));

                    if is_inline_component && seen_components.insert(id.clone()) {
                        component_usages.push(ComponentUsage {
                            id,
                            import_path: None,
                            is_inline: true,
                            style: Some(StyleExtractor::extract(&code)),
                        });
                    }
                }
            }
        }

        // Skip pages with no components
        if component_usages.is_empty() {
            return Ok(None);
        }

        Ok(Some(PageAnalysis {
            route,
            page_type,
            components: component_usages,
            source_path: path.to_owned(),
            template_name: template_name.to_string(),
        }))
    }

    /// Analyze an MDX page file.
    ///
    /// MDX files have a different structure from TSX pages:
    /// - Imports at the top (same syntax as TSX)
    /// - `export { Layout as default } from '@/components/Layout'` pattern
    /// - Content below `---` frontmatter separator
    /// - Inline JSX component usage in markdown content
    fn analyze_mdx_page(
        &self,
        path: &Utf8Path,
        app_root: &Utf8Path,
        template_name: &str,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<Option<PageAnalysis>, PageAnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        // Derive route from path relative to app root
        let route = path
            .parent()
            .and_then(|p| p.strip_prefix(app_root).ok())
            .map(|p| format!("/{}", p.as_str().replace('\\', "/")))
            .unwrap_or_else(|| "/".to_string());

        // Handle empty route or "app" route as home
        let route = if route == "/" || route == "/app" || route.is_empty() {
            "/".to_string()
        } else {
            route
        };

        // Try to infer page type from content analysis first (MDX content is more descriptive)
        let page_type = self.infer_mdx_page_type(&code, &route);

        // Parse imports and track component usages
        let mut component_usages = Vec::new();
        let mut seen_components = HashSet::new();

        // Extract the header section (before ---) for import parsing
        let header = code.split("---").next().unwrap_or(&code);

        // Parse standard imports
        if let Some(ref import_re) = self.import_regex {
            for cap in import_re.captures_iter(header) {
                let imports = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let import_path = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                // Skip non-component imports
                if !import_path.starts_with("@/components")
                    && !import_path.starts_with("./")
                    && !import_path.starts_with("../")
                {
                    continue;
                }

                // Parse individual imports from destructured import
                for import in imports.split(',') {
                    let import = import.trim();
                    // Handle renamed imports: "Foo as Bar"
                    let component_name = import.split(" as ").next().unwrap_or(import).trim();

                    if component_name.is_empty()
                        || !component_name
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_uppercase())
                    {
                        continue;
                    }

                    let id = self.derive_component_id(component_name, import_path);

                    if seen_components.insert(id.clone()) {
                        component_usages.push(ComponentUsage {
                            id: id.clone(),
                            import_path: Some(import_path.to_string()),
                            is_inline: false,
                            style: None,
                        });

                        // Track component usage
                        components
                            .entry(id.clone())
                            .or_insert_with(|| ComponentAnalysis {
                                id: id.clone(),
                                name: component_name.to_string(),
                                templates: Vec::new(),
                                page_types: HashSet::new(),
                                style: StyleProfile::default(),
                                source_path: Utf8PathBuf::new(),
                            })
                            .page_types
                            .insert(page_type);
                    }
                }
            }
        }

        // Parse MDX layout export pattern: export { Layout as default } from '...'
        if let Some(ref layout_re) = self.mdx_layout_regex {
            for cap in layout_re.captures_iter(header) {
                let component_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let import_path = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                if component_name.is_empty() {
                    continue;
                }

                let id = self.derive_component_id(component_name, import_path);

                if seen_components.insert(id.clone()) {
                    component_usages.push(ComponentUsage {
                        id: id.clone(),
                        import_path: Some(import_path.to_string()),
                        is_inline: false,
                        style: None,
                    });

                    // Track this as a layout component
                    components
                        .entry(id.clone())
                        .or_insert_with(|| ComponentAnalysis {
                            id: id.clone(),
                            name: component_name.to_string(),
                            templates: Vec::new(),
                            page_types: HashSet::new(),
                            style: StyleProfile::default(),
                            source_path: Utf8PathBuf::new(),
                        })
                        .page_types
                        .insert(page_type);
                }
            }
        }

        // Find inline JSX components used in the MDX content
        if let Some(ref component_re) = self.component_regex {
            for cap in component_re.captures_iter(&code) {
                if let Some(m) = cap.get(1) {
                    let component_name = m.as_str();

                    // Skip already tracked components and common React/Next elements
                    let id = component_name.to_lowercase();
                    if seen_components.contains(&id)
                        || ["Fragment", "Suspense", "Image", "Link", "Head", "Script"]
                            .contains(&component_name)
                    {
                        continue;
                    }

                    // If not imported, it might be used inline via MDX provider
                    if seen_components.insert(id.clone()) {
                        component_usages.push(ComponentUsage {
                            id,
                            import_path: None,
                            is_inline: true, // Provided via MDX context
                            style: None,
                        });
                    }
                }
            }
        }

        // MDX pages with just layout export are still valid pages
        if component_usages.is_empty() {
            return Ok(None);
        }

        Ok(Some(PageAnalysis {
            route,
            page_type,
            components: component_usages,
            source_path: path.to_owned(),
            template_name: template_name.to_string(),
        }))
    }

    /// Infer page type from MDX content (more descriptive than route alone).
    fn infer_mdx_page_type(&self, content: &str, route: &str) -> PageType {
        let lower_content = content.to_lowercase();

        // Check content for strong indicators
        if lower_content.contains("changelog") || lower_content.contains("what's new") {
            return PageType::Changelog;
        }
        if lower_content.contains("api reference")
            || lower_content.contains("api documentation")
            || lower_content.contains("endpoint")
        {
            return PageType::ApiReference;
        }
        if lower_content.contains("getting started")
            || lower_content.contains("installation")
            || lower_content.contains("quickstart")
        {
            return PageType::Docs;
        }
        if lower_content.contains("blog post")
            || lower_content.contains("article")
            || lower_content.contains("published")
            || content.contains("{{ date:")
        {
            return PageType::Blog;
        }
        if lower_content.contains("privacy policy")
            || lower_content.contains("terms of service")
            || lower_content.contains("legal")
        {
            return PageType::Legal;
        }

        // Fall back to route-based inference
        PageType::from_route(route)
    }

    /// Analyze a Markdoc page file (.md with {% component %} syntax).
    ///
    /// Used by the syntax template which uses Markdoc for documentation.
    fn analyze_markdoc_page(
        &self,
        path: &Utf8Path,
        app_root: &Utf8Path,
        template_name: &str,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<Option<PageAnalysis>, PageAnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        // Derive route from path relative to app root
        let route = path
            .parent()
            .and_then(|p| p.strip_prefix(app_root).ok())
            .map(|p| format!("/{}", p.as_str().replace('\\', "/")))
            .unwrap_or_else(|| "/".to_string());

        // Handle empty route as home
        let route = if route == "/" || route == "/app" || route.is_empty() {
            "/".to_string()
        } else {
            route
        };

        // Infer page type from frontmatter and content
        let page_type = self.infer_markdoc_page_type(&code, &route);

        // Parse Markdoc component syntax: {% component-name ... %}
        let mut component_usages = Vec::new();
        let mut seen_components = HashSet::new();

        if let Some(ref markdoc_re) = self.markdoc_regex {
            for cap in markdoc_re.captures_iter(&code) {
                if let Some(m) = cap.get(1) {
                    let component_name = m.as_str();

                    // Skip closing tags and common markdown elements
                    if component_name.starts_with('/')
                        || component_name == ".lead"
                        || component_name.is_empty()
                    {
                        continue;
                    }

                    // Normalize component name (quick-link -> quick-link)
                    let id = component_name.to_lowercase();

                    if seen_components.insert(id.clone()) {
                        component_usages.push(ComponentUsage {
                            id: id.clone(),
                            import_path: None, // Markdoc components are registered globally
                            is_inline: false,
                            style: None,
                        });

                        // Track component usage
                        let display_name = component_name
                            .split('-')
                            .map(|w| {
                                let mut c = w.chars();
                                c.next()
                                    .map_or_else(String::new, |first| {
                                        first.to_uppercase().chain(c).collect()
                                    })
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        components
                            .entry(id.clone())
                            .or_insert_with(|| ComponentAnalysis {
                                id: id.clone(),
                                name: display_name,
                                templates: Vec::new(),
                                page_types: HashSet::new(),
                                style: StyleProfile::default(),
                                source_path: Utf8PathBuf::new(),
                            })
                            .page_types
                            .insert(page_type);
                    }
                }
            }
        }

        // Markdoc pages without explicit components are still valid pages
        // (they use the documentation layout)
        if component_usages.is_empty() {
            // Add a synthetic "docs-layout" component to track the page
            component_usages.push(ComponentUsage {
                id: "docs-layout".to_string(),
                import_path: None,
                is_inline: false,
                style: None,
            });

            components
                .entry("docs-layout".to_string())
                .or_insert_with(|| ComponentAnalysis {
                    id: "docs-layout".to_string(),
                    name: "Docs Layout".to_string(),
                    templates: Vec::new(),
                    page_types: HashSet::new(),
                    style: StyleProfile::default(),
                    source_path: Utf8PathBuf::new(),
                })
                .page_types
                .insert(page_type);
        }

        Ok(Some(PageAnalysis {
            route,
            page_type,
            components: component_usages,
            source_path: path.to_owned(),
            template_name: template_name.to_string(),
        }))
    }

    /// Infer page type from Markdoc content and frontmatter.
    fn infer_markdoc_page_type(&self, content: &str, route: &str) -> PageType {
        let lower_content = content.to_lowercase();

        // Check frontmatter title and content for docs-specific patterns
        if lower_content.contains("installation")
            || lower_content.contains("getting started")
            || lower_content.contains("quickstart")
        {
            return PageType::Docs;
        }
        if lower_content.contains("api reference")
            || lower_content.contains("method:")
            || lower_content.contains("endpoint")
        {
            return PageType::ApiReference;
        }
        if lower_content.contains("architecture")
            || lower_content.contains("guide")
            || lower_content.contains("tutorial")
        {
            return PageType::Docs;
        }
        if lower_content.contains("changelog") || lower_content.contains("release notes") {
            return PageType::Changelog;
        }

        // Check route for docs path
        if route.contains("/docs") {
            return PageType::Docs;
        }

        // Fall back to standard route inference
        PageType::from_route(route)
    }

    /// Derive a component ID from name and import path.
    fn derive_component_id(&self, name: &str, import_path: &str) -> String {
        // For section imports, use the file name
        if import_path.contains("/sections/") {
            import_path
                .rsplit('/')
                .next()
                .unwrap_or(name)
                .to_string()
        } else {
            // Convert PascalCase to kebab-case
            let mut id = String::new();
            for (i, c) in name.chars().enumerate() {
                if c.is_uppercase() && i > 0 {
                    id.push('-');
                }
                id.push(c.to_lowercase().next().unwrap_or(c));
            }
            id
        }
    }

    /// Find and analyze layout files.
    fn find_layouts(
        &self,
        app_dir: &Utf8Path,
        layouts: &mut Vec<LayoutAnalysis>,
    ) -> Result<(), PageAnalysisError> {
        self.find_layouts_recursive(app_dir, app_dir, layouts)
    }

    /// Recursively find layout files.
    fn find_layouts_recursive(
        &self,
        dir: &Utf8Path,
        app_root: &Utf8Path,
        layouts: &mut Vec<LayoutAnalysis>,
    ) -> Result<(), PageAnalysisError> {
        let entries = std::fs::read_dir(dir.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let utf8_path =
                    Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;
                self.find_layouts_recursive(&utf8_path, app_root, layouts)?;
            } else {
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if file_name == "layout.tsx" || file_name == "layout.jsx" {
                    let utf8_path =
                        Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;

                    if let Some(layout) = self.analyze_layout_file(&utf8_path, app_root)? {
                        layouts.push(layout);
                    }
                }
            }
        }

        Ok(())
    }

    /// Analyze a layout file.
    fn analyze_layout_file(
        &self,
        path: &Utf8Path,
        app_root: &Utf8Path,
    ) -> Result<Option<LayoutAnalysis>, PageAnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        // Derive layout ID from path
        let route = path
            .parent()
            .and_then(|p| p.strip_prefix(app_root).ok())
            .map(|p| p.as_str().to_string())
            .unwrap_or_default();

        let id = if route.is_empty() {
            "root".to_string()
        } else {
            route.replace('/', "-")
        };

        // Find shared components (Header, Footer, Nav, etc.)
        let mut shared_components = Vec::new();

        let shared_patterns = [
            "Header", "Footer", "Nav", "Navbar", "Navigation", "Sidebar",
        ];

        if let Some(ref component_re) = self.component_regex {
            for cap in component_re.captures_iter(&code) {
                if let Some(m) = cap.get(1) {
                    let name = m.as_str();
                    if shared_patterns.iter().any(|p| name.contains(p)) {
                        shared_components.push(name.to_string());
                    }
                }
            }
        }

        Ok(Some(LayoutAnalysis {
            id,
            shared_components,
            routes: vec![route],
            source_path: path.to_owned(),
        }))
    }

    /// Analyze a components directory.
    fn analyze_component_dir(
        &self,
        dir: &Utf8Path,
        template_name: &str,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<(), PageAnalysisError> {
        self.analyze_component_dir_recursive(dir, template_name, components)
    }

    /// Recursively analyze component directories.
    fn analyze_component_dir_recursive(
        &self,
        dir: &Utf8Path,
        template_name: &str,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<(), PageAnalysisError> {
        let entries = std::fs::read_dir(dir.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let utf8_path =
                    Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;
                self.analyze_component_dir_recursive(&utf8_path, template_name, components)?;
            } else {
                let ext = path.extension().and_then(|e| e.to_str());
                if ext == Some("tsx") || ext == Some("jsx") {
                    let utf8_path =
                        Utf8PathBuf::try_from(path).map_err(|_| PageAnalysisError::InvalidPath)?;
                    self.analyze_component_file(&utf8_path, template_name, components)?;
                }
            }
        }

        Ok(())
    }

    /// Analyze a single component file.
    fn analyze_component_file(
        &self,
        path: &Utf8Path,
        template_name: &str,
        components: &mut HashMap<String, ComponentAnalysis>,
    ) -> Result<(), PageAnalysisError> {
        let code = std::fs::read_to_string(path.as_std_path())
            .map_err(|e| PageAnalysisError::IoError(e.to_string()))?;

        // Skip very small files (index exports)
        if code.len() < 100 {
            return Ok(());
        }

        // Derive ID from filename
        let id = path
            .file_stem()
            .map(|s| s.to_string())
            .unwrap_or_default()
            .to_lowercase()
            .replace('_', "-");

        // Skip index files
        if id == "index" {
            return Ok(());
        }

        // Extract style profile
        let style = StyleExtractor::extract(&code);

        // Derive human-readable name
        let name = path
            .file_stem()
            .map(|s| {
                s.split('-')
                    .map(|w| {
                        let mut c = w.chars();
                        c.next()
                            .map_or_else(String::new, |first| {
                                first.to_uppercase().chain(c).collect()
                            })
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();

        components
            .entry(id.clone())
            .and_modify(|c| {
                if !c.templates.contains(&template_name.to_string()) {
                    c.templates.push(template_name.to_string());
                }
                c.style = style.clone();
                c.source_path = path.to_owned();
            })
            .or_insert_with(|| ComponentAnalysis {
                id,
                name,
                templates: vec![template_name.to_string()],
                page_types: HashSet::new(),
                style,
                source_path: path.to_owned(),
            });

        Ok(())
    }

    /// Get all cached analyses.
    #[must_use]
    pub const fn analyses(&self) -> &HashMap<String, TemplatePageAnalysis> {
        &self.analyses
    }

    /// Get analysis for a specific template.
    #[must_use]
    pub fn get(&self, template_name: &str) -> Option<&TemplatePageAnalysis> {
        self.analyses.get(template_name)
    }
}

/// Errors during page analysis.
#[derive(Debug, Clone)]
pub enum PageAnalysisError {
    /// Invalid path
    InvalidPath,
    /// No source directory found
    NoSourceDir,
    /// IO error
    IoError(String),
}

impl std::fmt::Display for PageAnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "invalid path"),
            Self::NoSourceDir => write!(f, "no source directory found"),
            Self::IoError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for PageAnalysisError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_type_from_route_home() {
        assert_eq!(PageType::from_route("/"), PageType::Home);
        assert_eq!(PageType::from_route(""), PageType::Home);
        assert_eq!(PageType::from_route("page"), PageType::Home);
    }

    #[test]
    fn page_type_from_route_pricing() {
        assert_eq!(PageType::from_route("/pricing"), PageType::Pricing);
        assert_eq!(PageType::from_route("/plans/pricing"), PageType::Pricing);
    }

    #[test]
    fn page_type_from_route_blog() {
        assert_eq!(PageType::from_route("/blog"), PageType::Blog);
        assert_eq!(PageType::from_route("/blog/[slug]"), PageType::Blog);
        assert_eq!(PageType::from_route("/articles"), PageType::Blog);
    }

    #[test]
    fn page_type_from_route_docs() {
        assert_eq!(PageType::from_route("/docs"), PageType::Docs);
        assert_eq!(PageType::from_route("/docs/[...slug]"), PageType::Docs);
        assert_eq!(PageType::from_route("/documentation"), PageType::Docs);
    }

    #[test]
    fn page_type_from_route_about() {
        assert_eq!(PageType::from_route("/about"), PageType::About);
        assert_eq!(PageType::from_route("/team"), PageType::About);
    }

    #[test]
    fn page_type_from_route_legal() {
        assert_eq!(PageType::from_route("/privacy"), PageType::Legal);
        assert_eq!(PageType::from_route("/terms"), PageType::Legal);
    }

    #[test]
    fn page_type_from_route_auth() {
        assert_eq!(PageType::from_route("/login"), PageType::Auth);
        assert_eq!(PageType::from_route("/signin"), PageType::Auth);
        assert_eq!(PageType::from_route("/register"), PageType::Auth);
        assert_eq!(PageType::from_route("/signup"), PageType::Auth);
        assert_eq!(PageType::from_route("/(auth)/login"), PageType::Auth);
    }

    #[test]
    fn page_type_from_route_media() {
        assert_eq!(PageType::from_route("/episode/1"), PageType::Media);
        assert_eq!(PageType::from_route("/podcast"), PageType::Media);
        assert_eq!(PageType::from_route("/(main)/[episode]"), PageType::Media);
    }

    #[test]
    fn page_type_from_route_error() {
        assert_eq!(PageType::from_route("/404"), PageType::Error);
        assert_eq!(PageType::from_route("/500"), PageType::Error);
    }

    #[test]
    fn page_type_from_route_dashboard() {
        assert_eq!(PageType::from_route("/dashboard"), PageType::Dashboard);
        assert_eq!(PageType::from_route("/settings"), PageType::Dashboard);
        assert_eq!(PageType::from_route("/account"), PageType::Dashboard);
    }

    #[test]
    fn page_type_from_route_resources() {
        assert_eq!(PageType::from_route("/resources"), PageType::Resources);
        assert_eq!(PageType::from_route("/downloads"), PageType::Resources);
    }

    #[test]
    fn page_type_strips_route_groups() {
        // /(main) group should be stripped, resulting in "/" = home
        assert_eq!(PageType::from_route("/(main)"), PageType::Home);
        assert_eq!(
            PageType::from_route("/(centered)/resources"),
            PageType::Resources
        );
        assert_eq!(PageType::from_route("/(sidebar)/blog"), PageType::Blog);
    }

    #[test]
    fn page_type_content_pages() {
        assert_eq!(PageType::from_route("/thank-you"), PageType::Content);
        assert_eq!(PageType::from_route("/success"), PageType::Content);
    }

    #[test]
    fn normalize_route_strips_groups() {
        assert_eq!(PageType::normalize_route("/(main)"), "/");
        assert_eq!(PageType::normalize_route("/(auth)/login"), "/login");
        assert_eq!(
            PageType::normalize_route("/(centered)/resources"),
            "/resources"
        );
        assert_eq!(PageType::normalize_route("/(main)/(nested)/page"), "/page");
        assert_eq!(PageType::normalize_route("/(centered)/resources"), "/resources");
        assert_eq!(
            PageType::normalize_route("/(main)/(nested)/page"),
            "/page"
        );
    }
}
