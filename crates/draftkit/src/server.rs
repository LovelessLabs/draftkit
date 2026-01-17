//! MCP server with tool router for TailwindPlus components.

use rmcp::ErrorData as McpError;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, GetPromptRequestParam, GetPromptResult, Implementation,
    ListPromptsResult, ListResourcesResult, Prompt, PromptArgument, PromptMessage,
    PromptMessageContent, PromptMessageRole, ReadResourceRequestParam, ReadResourceResult,
    ServerCapabilities, ServerInfo,
};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use draftkit_core::catalyst::{self, CatalystLanguage};
use draftkit_core::components::TailwindVersion;
use draftkit_core::fetch::{ComponentFetcher, FetchError};
use draftkit_core::intelligence::{
    ComponentMatcher, PatternMatcher, RecipeOptions, StylePreference,
};
use draftkit_core::patterns::PatternLoader;
use draftkit_core::{ComponentReader, Framework, Mode, cache, docs, elements};

use crate::cli::stderr_spinner;
use crate::commands::auth;

/// MCP Server for TailwindPlus components and Tailwind CSS documentation
#[derive(Clone)]
pub struct DraftkitServer {
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
    component_reader: ComponentReader,
    /// Lazily initialized fetcher for on-demand component fetching
    fetcher: Arc<Mutex<Option<ComponentFetcher>>>,
}

// Tool parameter structs

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchParams {
    /// Search query (matches component names, categories, subcategories)
    pub query: String,
    /// Optional category filter (e.g., "Application UI", "Marketing", "Ecommerce")
    #[serde(default)]
    pub category: Option<String>,
    /// Maximum number of results (default: 20, max: 100)
    #[serde(default = "default_limit")]
    pub limit: Option<usize>,
}

const fn default_limit() -> Option<usize> {
    Some(20)
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetComponentParams {
    /// Component ID from search results
    pub id: String,
    /// Target framework: html, react, or vue
    pub framework: Framework,
    /// Theme mode: light, dark, or system
    pub mode: Mode,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DocsParams {
    /// Documentation topic (e.g., "flexbox", "grid", "spacing", "typography")
    pub topic: String,
    /// Tailwind CSS version: "v3" or "v4" (default: "v4")
    #[serde(default = "default_tailwind_version")]
    pub version: Option<String>,
}

fn default_tailwind_version() -> Option<String> {
    Some("v4".to_string())
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct TemplateParams {
    /// Template name (optional - omit to list all templates)
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CatalystParams {
    /// Component name (e.g., "button", "dialog", "table")
    pub name: String,
    /// Language: "typescript" (default) or "javascript"
    #[serde(default = "default_catalyst_language")]
    pub language: Option<String>,
}

fn default_catalyst_language() -> Option<String> {
    Some("typescript".to_string())
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ElementsParams {
    /// Element name (e.g., "dialog", "dropdown-menu", "tabs")
    /// Leave empty to get the overview documentation
    #[serde(default)]
    pub component: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ComponentMetaParams {
    /// Component ID from search results
    pub id: String,
    /// Target framework: html, react, or vue
    pub framework: Framework,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ComponentTokensParams {
    /// Component ID from search results
    pub id: String,
    /// Target framework: html, react, or vue (default: react)
    #[serde(default = "default_framework")]
    pub framework: Option<Framework>,
}

const fn default_framework() -> Option<Framework> {
    Some(Framework::React)
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CompatibilityParams {
    /// Component ID, or "summary" for overall compatibility stats
    pub id: String,
    /// Target framework: html, react, or vue (default: react)
    #[serde(default = "default_framework")]
    pub framework: Option<Framework>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RecipeParams {
    /// Pattern name: "saas-landing", "marketing", "portfolio", or custom pattern ID
    pub pattern: String,
    /// Section to emphasize (use recommended variant for this section)
    #[serde(default)]
    pub emphasis: Option<String>,
    /// Style preference: "minimal", "balanced", or "bold"
    #[serde(default)]
    pub style: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SuggestSectionParams {
    /// Pattern name to use for suggestions
    pub pattern: String,
    /// Current sections already added to the page (by type, e.g., ["header", "hero"])
    #[serde(default)]
    pub current_sections: Vec<String>,
}

// Response types

#[derive(Debug, Serialize)]
struct SearchResultItem {
    id: String,
    name: String,
    category: String,
    subcategory: String,
    sub_subcategory: String,
}

#[derive(Debug, Serialize)]
struct ComponentCode {
    id: String,
    name: String,
    category: String,
    subcategory: String,
    sub_subcategory: String,
    framework: String,
    mode: String,
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<String>,
}

// Tool implementations
#[tool_router]
impl DraftkitServer {
    #[tool(
        description = "Search TailwindPlus components by keyword. Returns matching component IDs, names, and category paths. Use get_component to retrieve the actual code."
    )]
    async fn search_components(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let limit = params.limit.unwrap_or(20).min(100);

        // Use keyword search on embedded components
        let results = self
            .component_reader
            .search(Framework::React, &params.query);

        // Apply category filter if specified
        let filtered: Vec<_> = results
            .into_iter()
            .filter(|c| {
                params
                    .category
                    .as_ref()
                    .is_none_or(|cat| c.category.eq_ignore_ascii_case(cat))
            })
            .take(limit)
            .collect();

        let items: Vec<SearchResultItem> = filtered
            .iter()
            .map(|c| SearchResultItem {
                id: c.id.clone(),
                name: c.name.clone(),
                category: c.category.clone(),
                subcategory: c.subcategory.clone(),
                sub_subcategory: c.sub_subcategory.clone(),
            })
            .collect();

        if items.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "No components found matching your query.",
            )]));
        }

        let json = serde_json::to_string_pretty(&items)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Get component code by ID. Specify framework (react/vue/html) and mode (light/dark/system). Returns the component code ready to use."
    )]
    async fn get_component(
        &self,
        Parameters(params): Parameters<GetComponentParams>,
    ) -> Result<CallToolResult, McpError> {
        let record = self
            .component_reader
            .find_by_id(params.framework, &params.id)
            .ok_or_else(|| {
                McpError::resource_not_found(format!("Component not found: {}", params.id), None)
            })?;

        // Check if this mode variant exists
        if !record.has_mode(params.mode) {
            return Err(McpError::resource_not_found(
                format!(
                    "Component '{}' does not have {} mode variant",
                    params.id, params.mode
                ),
                None,
            ));
        }

        // Fetch code on-demand (embedded data contains metadata only, not source code)
        let code = self
            .fetch_component_on_demand(
                &record.uuid,
                &record.category,
                &record.subcategory,
                &record.sub_subcategory,
                params.framework,
                params.mode,
            )
            .await
            .map_err(|e| match e {
                FetchError::NotAuthenticated => McpError::internal_error(
                    "Not authenticated.\n\n\
                     To fetch TailwindPlus components, run:\n\n\
                     ```\n\
                     draftkit auth\n\
                     ```"
                    .to_string(),
                    None,
                ),
                FetchError::SessionExpired => McpError::internal_error(
                    "Session expired. Run `draftkit auth --refresh` to renew.".to_string(),
                    None,
                ),
                FetchError::ComponentNotFound(msg) => {
                    McpError::resource_not_found(format!("Component not found: {msg}"), None)
                }
                other => McpError::internal_error(format!("Fetch failed: {other}"), None),
            })?;

        // Preview URL comes from embedded metadata
        let preview = record.preview_url(params.mode).map(ToString::to_string);

        let response = ComponentCode {
            id: record.id.clone(),
            name: record.name.clone(),
            category: record.category.clone(),
            subcategory: record.subcategory.clone(),
            sub_subcategory: record.sub_subcategory.clone(),
            framework: params.framework.as_str().to_string(),
            mode: params.mode.as_str().to_string(),
            code,
            preview,
        };

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "List all component categories with counts. Returns the category hierarchy for browsing."
    )]
    async fn list_categories(&self) -> Result<CallToolResult, McpError> {
        // Build category counts from embedded data
        let all = self.component_reader.all(Framework::React);

        let mut counts: HashMap<&str, usize> = HashMap::new();
        for component in all {
            *counts.entry(component.category.as_str()).or_insert(0) += 1;
        }

        let categories: Vec<serde_json::Value> = counts
            .into_iter()
            .map(|(name, count)| {
                serde_json::json!({
                    "name": name,
                    "count": count
                })
            })
            .collect();

        let json = serde_json::to_string_pretty(&categories)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Get Tailwind CSS documentation for a utility or concept. Supports v3 and v4 (default). Topics include: flexbox, grid, spacing, sizing, typography, colors, backgrounds, borders, effects, filters, transforms, transitions, interactivity, states, responsive, dark-mode, accessibility, svg. v4 adds: forms, v4-changes. Use 'index' for the full topic list."
    )]
    async fn get_tailwind_docs(
        &self,
        Parameters(params): Parameters<DocsParams>,
    ) -> Result<CallToolResult, McpError> {
        // Parse version (default to v4)
        let version_str = params.version.as_deref().unwrap_or("v4");
        let version = TailwindVersion::parse(version_str).ok_or_else(|| {
            McpError::invalid_params(
                format!("Invalid version '{version_str}'. Use 'v3' or 'v4'."),
                None,
            )
        })?;

        // First try to get the exact topic
        if let Some(content) = docs::get_docs(&params.topic, version) {
            return Ok(CallToolResult::success(vec![Content::text(content)]));
        }

        // If not found, check if query matches any topic names/descriptions
        let matches = docs::search_topics(&params.topic, version);
        if !matches.is_empty() {
            let suggestions: Vec<String> = matches
                .iter()
                .map(|(name, desc)| format!("  - {name}: {desc}"))
                .collect();

            return Err(McpError::resource_not_found(
                format!(
                    "Topic '{}' not found for {}. Did you mean one of these?\n{}",
                    params.topic,
                    version,
                    suggestions.join("\n")
                ),
                None,
            ));
        }

        // No matches at all - list all available topics for this version
        let all_topics: Vec<String> = docs::list_topics(version)
            .iter()
            .map(|(name, desc)| format!("  - {name}: {desc}"))
            .collect();

        Err(McpError::resource_not_found(
            format!(
                "Topic '{}' not found for {}. Available topics:\n{}",
                params.topic,
                version,
                all_topics.join("\n")
            ),
            None,
        ))
    }

    #[tool(
        description = "Get TailwindPlus template information. Returns metadata about official templates."
    )]
    async fn get_template_info(
        &self,
        Parameters(params): Parameters<TemplateParams>,
    ) -> Result<CallToolResult, McpError> {
        let templates = serde_json::json!([
            {"name": "Oatmeal", "category": "SaaS Marketing", "tech_stack": ["React", "Next.js", "Tailwind CSS"]},
            {"name": "Spotlight", "category": "Personal Website", "tech_stack": ["React", "Next.js", "Tailwind CSS"]},
            {"name": "Radiant", "category": "SaaS Marketing", "tech_stack": ["React", "Next.js", "Tailwind CSS"]},
            {"name": "Compass", "category": "Course Platform", "tech_stack": ["React", "Next.js", "Tailwind CSS"]},
            {"name": "Protocol", "category": "API Reference", "tech_stack": ["React", "Next.js", "Tailwind CSS"]},
            {"name": "Syntax", "category": "Documentation", "tech_stack": ["React", "Next.js", "Tailwind CSS"]}
        ]);

        if let Some(name) = params.name {
            if let Some(template) = templates
                .as_array()
                .and_then(|arr| arr.iter().find(|t| t["name"].as_str() == Some(&name)))
            {
                return Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(template).unwrap(),
                )]));
            }
            return Err(McpError::resource_not_found(
                format!("Template not found: {name}"),
                None,
            ));
        }

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&templates).unwrap(),
        )]))
    }

    #[tool(
        description = "List all available Catalyst UI Kit components. Catalyst provides atomic React components for building production UIs with Tailwind CSS."
    )]
    async fn list_catalyst_components(&self) -> Result<CallToolResult, McpError> {
        let components = catalyst::get_component_metadata();

        let json: Vec<serde_json::Value> = components
            .into_iter()
            .map(|c| {
                serde_json::json!({
                    "name": c.name,
                    "description": c.description
                })
            })
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap(),
        )]))
    }

    #[tool(
        description = "Get Catalyst UI Kit component source code. Returns the full component implementation in TypeScript (.tsx) or JavaScript (.jsx)."
    )]
    async fn get_catalyst_component(
        &self,
        Parameters(params): Parameters<CatalystParams>,
    ) -> Result<CallToolResult, McpError> {
        let language_str = params.language.as_deref().unwrap_or("typescript");
        let language = CatalystLanguage::parse(language_str).ok_or_else(|| {
            McpError::invalid_params(
                format!("Invalid language '{language_str}'. Use 'typescript' or 'javascript'."),
                None,
            )
        })?;

        let code = catalyst::get_component(&params.name, language).ok_or_else(|| {
            McpError::resource_not_found(
                format!(
                    "Catalyst component '{}' not found. Use list_catalyst_components to see available components.",
                    params.name
                ),
                None,
            )
        })?;

        let response = serde_json::json!({
            "name": params.name,
            "language": language.as_str(),
            "extension": language.extension(),
            "code": code
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(
        description = "List all TailwindPlus Elements interactive Web Components. Elements provide JavaScript-powered interactivity (dialogs, dropdowns, tabs, etc.) for HTML snippets. Works with any framework."
    )]
    async fn list_elements(&self) -> Result<CallToolResult, McpError> {
        let element_list = elements::list_elements();

        let json: Vec<serde_json::Value> = element_list
            .into_iter()
            .map(|e| {
                serde_json::json!({
                    "name": e.name,
                    "tag": e.tag,
                    "description": e.description,
                    "use_cases": e.use_cases
                })
            })
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap(),
        )]))
    }

    #[tool(
        description = "Get TailwindPlus Elements documentation. Specify a component name (dialog, dropdown-menu, tabs, etc.) for its API reference, or omit to get the overview with installation instructions."
    )]
    async fn get_elements_docs(
        &self,
        Parameters(params): Parameters<ElementsParams>,
    ) -> Result<CallToolResult, McpError> {
        match params.component {
            Some(name) => {
                let docs = elements::get_element_docs(&name).ok_or_else(|| {
                    let available: Vec<&str> =
                        elements::list_elements().iter().map(|e| e.name).collect();
                    McpError::resource_not_found(
                        format!(
                            "Element '{name}' not found. Available elements: {}",
                            available.join(", ")
                        ),
                        None,
                    )
                })?;
                Ok(CallToolResult::success(vec![Content::text(docs)]))
            }
            None => Ok(CallToolResult::success(vec![Content::text(
                elements::get_overview(),
            )])),
        }
    }

    #[tool(
        description = "Get component metadata including npm dependencies, icons used, and JS behaviors. Use this to understand what packages need to be installed."
    )]
    async fn get_component_meta(
        &self,
        Parameters(params): Parameters<ComponentMetaParams>,
    ) -> Result<CallToolResult, McpError> {
        let component = self
            .component_reader
            .find_by_id(params.framework, &params.id)
            .ok_or_else(|| {
                McpError::invalid_params(format!("Component not found: {}", params.id), None)
            })?;

        let meta = component.meta.as_ref();

        #[allow(clippy::option_if_let_else)] // if-let clearer for complex format strings
        let response = if let Some(m) = meta {
            // Build install command for packages
            let install_cmd = if m.dependencies.packages.is_empty() {
                "No external packages required".to_string()
            } else {
                format!("npm install {}", m.dependencies.packages.join(" "))
            };

            format!(
                r#"# Component Metadata: {}

## Dependencies

### npm Packages
{}

### Install Command
```bash
{}
```

### Icons Used ({})
{}

## Tailwind Compatibility
- v3 Compatible: {}
- v4-only Features: {}"#,
                component.name,
                if m.dependencies.packages.is_empty() {
                    "None".to_string()
                } else {
                    m.dependencies
                        .packages
                        .iter()
                        .map(|p| format!("- {p}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                install_cmd,
                m.dependencies.icons.len(),
                if m.dependencies.icons.is_empty() {
                    "None".to_string()
                } else {
                    m.dependencies.icons.join(", ")
                },
                if m.tailwind.v3_compatible {
                    "Yes"
                } else {
                    "No"
                },
                if m.tailwind.v4_only.is_empty() {
                    "None".to_string()
                } else {
                    m.tailwind.v4_only.join(", ")
                }
            )
        } else {
            format!(
                "# Component: {}\n\nNo metadata available. Run `scripts/metadata.sh` to extract metadata.",
                component.name
            )
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(
        description = "Get Tailwind CSS tokens used by a component (colors, spacing, typography). Useful for design system integration."
    )]
    async fn get_component_tokens(
        &self,
        Parameters(params): Parameters<ComponentTokensParams>,
    ) -> Result<CallToolResult, McpError> {
        let framework = params.framework.unwrap_or(Framework::React);
        let component = self
            .component_reader
            .find_by_id(framework, &params.id)
            .ok_or_else(|| {
                McpError::invalid_params(format!("Component not found: {}", params.id), None)
            })?;

        let meta = component.meta.as_ref();

        #[allow(clippy::option_if_let_else)] // if-let clearer for complex format strings
        let response = if let Some(m) = meta {
            format!(
                r#"# Tailwind Tokens: {}

## Colors ({})
{}

## Spacing ({})
{}

## Typography ({})
{}"#,
                component.name,
                m.tokens.colors.len(),
                if m.tokens.colors.is_empty() {
                    "None detected".to_string()
                } else {
                    m.tokens.colors.join(", ")
                },
                m.tokens.spacing.len(),
                if m.tokens.spacing.is_empty() {
                    "None detected".to_string()
                } else {
                    m.tokens.spacing.join(", ")
                },
                m.tokens.typography.len(),
                if m.tokens.typography.is_empty() {
                    "None detected".to_string()
                } else {
                    m.tokens.typography.join(", ")
                }
            )
        } else {
            format!(
                "# Component: {}\n\nNo token metadata available. Run `scripts/metadata.sh` to extract metadata.",
                component.name
            )
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(
        description = "Check Tailwind v3/v4 compatibility for a component. Pass 'summary' as id for overall stats."
    )]
    async fn get_compatibility_info(
        &self,
        Parameters(params): Parameters<CompatibilityParams>,
    ) -> Result<CallToolResult, McpError> {
        let framework = params.framework.unwrap_or(Framework::React);

        if params.id == "summary" {
            // Overall compatibility summary
            let all = self.component_reader.all(framework);
            let total = all.len();
            let mut v3_compatible = 0;
            let mut v4_only = 0;
            let mut no_meta = 0;
            let mut v4_features: HashMap<String, usize> = HashMap::new();

            for component in all {
                if let Some(meta) = &component.meta {
                    if meta.tailwind.v3_compatible {
                        v3_compatible += 1;
                    } else {
                        v4_only += 1;
                    }
                    for feature in &meta.tailwind.v4_only {
                        *v4_features.entry(feature.clone()).or_insert(0) += 1;
                    }
                } else {
                    no_meta += 1;
                }
            }

            let mut feature_list: Vec<_> = v4_features.into_iter().collect();
            feature_list.sort_by(|a, b| b.1.cmp(&a.1));

            let response = format!(
                r#"# Tailwind Compatibility Summary

## Overall Stats
- Total components: {}
- v3 compatible: {} ({:.1}%)
- v4 only: {} ({:.1}%)
- No metadata: {}

## Most Common v4-only Features
{}"#,
                total,
                v3_compatible,
                (v3_compatible as f64 / total as f64) * 100.0,
                v4_only,
                (v4_only as f64 / total as f64) * 100.0,
                no_meta,
                feature_list
                    .iter()
                    .take(10)
                    .map(|(f, count)| format!("- {f}: {count} components"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            Ok(CallToolResult::success(vec![Content::text(response)]))
        } else {
            // Single component compatibility
            let component = self
                .component_reader
                .find_by_id(framework, &params.id)
                .ok_or_else(|| {
                    McpError::invalid_params(format!("Component not found: {}", params.id), None)
                })?;

            #[allow(clippy::option_if_let_else)] // if-let clearer for complex format strings
            let response = if let Some(meta) = &component.meta {
                format!(
                    r#"# Compatibility: {}

## Tailwind v3
- Compatible: {}
{}

## Tailwind v4
- Compatible: Yes (all components work with v4)
- v4-only features used: {}"#,
                    component.name,
                    if meta.tailwind.v3_compatible {
                        "Yes"
                    } else {
                        "No"
                    },
                    if meta.tailwind.v3_compatible {
                        String::new()
                    } else {
                        format!(
                            "- Reason: Uses v4-only features: {}",
                            meta.tailwind.v4_only.join(", ")
                        )
                    },
                    if meta.tailwind.v4_only.is_empty() {
                        "None".to_string()
                    } else {
                        meta.tailwind.v4_only.join(", ")
                    }
                )
            } else {
                format!(
                    "# Component: {}\n\nNo compatibility metadata available.",
                    component.name
                )
            };

            Ok(CallToolResult::success(vec![Content::text(response)]))
        }
    }

    #[tool(
        description = "Get a summary of everything this MCP server provides, including component counts, available tools, and when the data was last refreshed."
    )]
    async fn get_summary(&self) -> Result<CallToolResult, McpError> {
        // Get component counts
        let component_count = self.component_reader.component_count(Framework::React);

        // Build category counts
        let all = self.component_reader.all(Framework::React);
        let mut category_counts: HashMap<&str, usize> = HashMap::new();
        for c in all {
            *category_counts.entry(c.category.as_str()).or_insert(0) += 1;
        }

        let catalyst_components = catalyst::get_component_metadata();
        let elements_list = elements::list_elements();
        let doc_topics_v3 = docs::list_topics(TailwindVersion::V3);
        let doc_topics_v4 = docs::list_topics(TailwindVersion::V4);

        let category_summary: Vec<String> = category_counts
            .iter()
            .map(|(name, count)| format!("  - {name}: {count} components"))
            .collect();

        let summary = format!(
            r#"# Draftkit MCP Server Summary

## Server Info
- Version: {}
- Build date: {}

## UI Blocks
- Total components: {}
- Frameworks: React, Vue, HTML
- Modes: light, dark, system (v4 only)

### Categories
{}

## Catalyst UI Kit
- Components: {} atomic React components
- Languages: TypeScript (.tsx), JavaScript (.jsx)

## Elements (Web Components)
- Components: {} interactive components
- Examples: {}

## Tailwind CSS Documentation
- v3 topics: {} documentation pages
- v4 topics: {} documentation pages

## Available Tools
1. **search_components** - Search UI Blocks by keyword
2. **get_component** - Get component code by ID
3. **list_categories** - Browse component categories
4. **list_catalyst_components** - List Catalyst components
5. **get_catalyst_component** - Get Catalyst source code
6. **list_elements** - List Elements components
7. **get_elements_docs** - Get Elements documentation
8. **get_tailwind_docs** - Get Tailwind CSS documentation (v3/v4)
9. **get_template_info** - Get template metadata
10. **get_component_meta** - Get component dependencies and icons
11. **get_component_tokens** - Get Tailwind tokens used by a component
12. **get_compatibility_info** - Check v3/v4 compatibility
13. **get_summary** - This summary

## Page Intelligence Tools
14. **list_patterns** - List available page patterns (saas-landing, marketing, etc.)
15. **get_recipe** - Generate a complete page recipe from a pattern
16. **suggest_section** - Get suggestions for the next section to add
17. **preview_recipe** - Get preview URLs for a recipe's sections"#,
            env!("CARGO_PKG_VERSION"),
            compile_time_date(),
            component_count,
            category_summary.join("\n"),
            catalyst_components.len(),
            elements_list.len(),
            elements_list
                .iter()
                .take(5)
                .map(|e| e.name)
                .collect::<Vec<_>>()
                .join(", "),
            doc_topics_v3.len(),
            doc_topics_v4.len(),
        );

        Ok(CallToolResult::success(vec![Content::text(summary)]))
    }

    #[tool(
        description = "Generate a complete page recipe from a pattern. Returns ordered sections with component variants, coherence validation, and slot defaults. Available patterns: saas-landing, marketing, portfolio."
    )]
    async fn get_recipe(
        &self,
        Parameters(params): Parameters<RecipeParams>,
    ) -> Result<CallToolResult, McpError> {
        let loader = PatternLoader::builtin_only();
        let matcher = PatternMatcher::new();

        // Get the pattern
        let pattern_entry = loader.get(&params.pattern).ok_or_else(|| {
            let available: Vec<_> = loader
                .list_all()
                .iter()
                .map(|p| p.pattern.id.as_str())
                .collect();
            McpError::invalid_params(
                format!(
                    "Pattern '{}' not found. Available patterns: {}",
                    params.pattern,
                    available.join(", ")
                ),
                None,
            )
        })?;

        // Build recipe options
        let style_pref = params.style.as_deref().and_then(|s| match s {
            "minimal" => Some(StylePreference::Minimal),
            "balanced" => Some(StylePreference::Balanced),
            "bold" => Some(StylePreference::Bold),
            _ => None,
        });

        let opts = RecipeOptions {
            emphasis: params.emphasis,
            style_preference: style_pref,
            component_profiles: HashMap::new(), // Would be populated from component metadata
        };

        // Generate the recipe
        let recipe = matcher.generate_recipe(&pattern_entry.pattern, &opts);

        // Create component matcher for recommendations
        let component_matcher = ComponentMatcher::react();

        // Build response with component recommendations
        let sections: Vec<serde_json::Value> = recipe
            .sections
            .iter()
            .map(|s| {
                // Get recommended components for this section
                let recommendations = component_matcher.match_section(
                    &s.section_type,
                    &s.variant_id,
                    3, // Top 3 recommendations
                );

                let recommended: Vec<serde_json::Value> = recommendations
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "id": r.id,
                            "name": r.name,
                            "confidence": r.confidence,
                            "preview_url": r.preview_url
                        })
                    })
                    .collect();

                serde_json::json!({
                    "section_type": s.section_type,
                    "variant_id": s.variant_id,
                    "position": s.position,
                    "slots": s.slots,
                    "recommended_components": recommended
                })
            })
            .collect();

        let response = serde_json::json!({
            "pattern_id": recipe.pattern_id,
            "sections": sections,
            "coherence": {
                "score": recipe.coherence.score,
                "valid": recipe.coherence.valid,
                "issues": recipe.coherence.issues.iter().map(|i| {
                    serde_json::json!({
                        "category": i.category.as_str(),
                        "message": i.message,
                        "severity": i.severity
                    })
                }).collect::<Vec<_>>()
            },
            "dependencies": recipe.dependencies
        });

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "List available page patterns. Returns pattern IDs, descriptions, and section counts."
    )]
    async fn list_patterns(&self) -> Result<CallToolResult, McpError> {
        let loader = PatternLoader::builtin_only();
        let patterns = loader.list_all();

        let json: Vec<serde_json::Value> = patterns
            .iter()
            .map(|p| {
                serde_json::json!({
                    "id": p.pattern.id,
                    "name": p.pattern.name,
                    "description": p.pattern.description,
                    "section_count": p.pattern.sections.len(),
                    "sections": p.pattern.sections.iter().map(|s| &s.section_type).collect::<Vec<_>>()
                })
            })
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap(),
        )]))
    }

    #[tool(
        description = "Suggest the next section to add based on pattern and current page state. Helps iteratively build pages section by section."
    )]
    async fn suggest_section(
        &self,
        Parameters(params): Parameters<SuggestSectionParams>,
    ) -> Result<CallToolResult, McpError> {
        let loader = PatternLoader::builtin_only();
        let matcher = PatternMatcher::new();

        let pattern_entry = loader.get(&params.pattern).ok_or_else(|| {
            McpError::invalid_params(format!("Pattern '{}' not found", params.pattern), None)
        })?;

        let suggestions =
            matcher.suggest_next_section(&pattern_entry.pattern, &params.current_sections);

        let json: Vec<serde_json::Value> = suggestions
            .iter()
            .map(|s| {
                serde_json::json!({
                    "section_type": s.section_type,
                    "reason": s.reason,
                    "priority": s.priority,
                    "required": s.required,
                    "variants": s.variants.iter().map(|v| {
                        serde_json::json!({
                            "id": v.id,
                            "weight": v.weight,
                            "recommended": v.recommended
                        })
                    }).collect::<Vec<_>>()
                })
            })
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap(),
        )]))
    }

    #[tool(
        description = "Get visual preview URLs for a recipe's sections. Searches for components matching each section type and returns their preview image URLs."
    )]
    async fn preview_recipe(
        &self,
        Parameters(params): Parameters<RecipeParams>,
    ) -> Result<CallToolResult, McpError> {
        let loader = PatternLoader::builtin_only();
        let matcher = PatternMatcher::new();

        let pattern_entry = loader.get(&params.pattern).ok_or_else(|| {
            McpError::invalid_params(format!("Pattern '{}' not found", params.pattern), None)
        })?;

        let style_pref = params.style.as_deref().and_then(|s| match s {
            "minimal" => Some(StylePreference::Minimal),
            "balanced" => Some(StylePreference::Balanced),
            "bold" => Some(StylePreference::Bold),
            _ => None,
        });

        let opts = RecipeOptions {
            emphasis: params.emphasis,
            style_preference: style_pref,
            component_profiles: HashMap::new(),
        };

        let recipe = matcher.generate_recipe(&pattern_entry.pattern, &opts);

        // For each section, search for matching components and get preview URLs
        let mut section_previews: Vec<serde_json::Value> = Vec::new();

        for section in &recipe.sections {
            // Search for components matching this section type
            let search_results = self
                .component_reader
                .search(Framework::React, &section.section_type);

            // Take the first few matching components' previews
            let previews: Vec<serde_json::Value> = search_results
                .iter()
                .take(3)
                .filter_map(|c| {
                    c.preview_url(Mode::Light).map(|url| {
                        serde_json::json!({
                            "component_id": c.id,
                            "name": c.name,
                            "preview_url": url
                        })
                    })
                })
                .collect();

            section_previews.push(serde_json::json!({
                "section_type": section.section_type,
                "position": section.position,
                "selected_variant": section.variant_id,
                "available_previews": previews
            }));
        }

        let response = serde_json::json!({
            "pattern_id": recipe.pattern_id,
            "section_previews": section_previews
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }
}

fn compile_time_date() -> &'static str {
    option_env!("BUILD_DATE").unwrap_or(env!("CARGO_PKG_VERSION"))
}

#[tool_handler]
impl ServerHandler for DraftkitServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
            server_info: Implementation {
                name: env!("CARGO_PKG_NAME").to_string(),
                title: Some("Draftkit MCP Server".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                website_url: Some("https://github.com/lovelesslabs/draftkit".to_string()),
            },
            instructions: Some(
                r#"TailwindPlus component browser and Tailwind CSS documentation server.

## UI Blocks (657 complete page components)
- search_components: Find components by keyword (returns IDs)
- get_component: Get component code by ID, framework, and mode
- list_categories: Browse component categories with counts

Frameworks: react, vue, html
Modes: light, dark, system

## Page Intelligence (build complete pages)
- list_patterns: See available page patterns (saas-landing, marketing, portfolio)
- get_recipe: Generate a complete page with ordered sections and component variants
- suggest_section: Get suggestions for what section to add next
- preview_recipe: Get preview URLs for a recipe's sections

## Catalyst UI Kit (27 atomic React components)
- list_catalyst_components: List all available Catalyst components
- get_catalyst_component: Get component source code (TypeScript or JavaScript)

## Elements (9 interactive Web Components)
- list_elements: List all available Element components
- get_elements_docs: Get Element documentation (overview or specific component)

## Other Tools
- get_tailwind_docs: Get Tailwind CSS utility documentation
- get_template_info: Get TailwindPlus template metadata
"#
                .to_string(),
            ),
        }
    }

    fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
        std::future::ready(Ok(ListResourcesResult {
            resources: vec![],
            next_cursor: None,
            meta: None,
        }))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        std::future::ready(Err(McpError::resource_not_found(
            format!("Resource not found: {}", request.uri),
            None,
        )))
    }

    fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListPromptsResult, McpError>> + Send + '_ {
        let prompts = vec![
            Prompt {
                name: "implement-ui".to_string(),
                title: Some("Implement UI Feature".to_string()),
                description: Some(
                    "Implement a UI feature using TailwindPlus components".to_string(),
                ),
                arguments: Some(vec![
                    PromptArgument {
                        name: "description".to_string(),
                        title: None,
                        description: Some(
                            "What UI do you need? (e.g., 'login form', 'pricing table')"
                                .to_string(),
                        ),
                        required: Some(true),
                    },
                    PromptArgument {
                        name: "framework".to_string(),
                        title: None,
                        description: Some("Target framework: react, vue, or html".to_string()),
                        required: Some(true),
                    },
                ]),
                icons: None,
                meta: None,
            },
            Prompt {
                name: "explain-utility".to_string(),
                title: Some("Explain Tailwind Utility".to_string()),
                description: Some("Explain a Tailwind CSS utility class".to_string()),
                arguments: Some(vec![PromptArgument {
                    name: "utility".to_string(),
                    title: None,
                    description: Some(
                        "The utility class to explain (e.g., 'flex', 'grid-cols-3')".to_string(),
                    ),
                    required: Some(true),
                }]),
                icons: None,
                meta: None,
            },
        ];

        std::future::ready(Ok(ListPromptsResult {
            prompts,
            next_cursor: None,
            meta: None,
        }))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl std::future::Future<Output = Result<GetPromptResult, McpError>> + Send + '_ {
        let result = match request.name.as_str() {
            "implement-ui" => {
                let args = request.arguments.unwrap_or_default();
                let description = args
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(not provided)");
                let framework = args
                    .get("framework")
                    .and_then(|v| v.as_str())
                    .unwrap_or("react");

                let prompt = format!(
                    r#"Help me implement: {description}

Use TailwindPlus components in {framework} with Tailwind CSS v4.

Steps:
1. Search for relevant components using search_components
2. Get the component code with get_component
3. Adapt it to the specific requirements
4. Explain any customizations needed"#
                );

                Ok(GetPromptResult {
                    description: Some(
                        "Implement a UI feature using TailwindPlus components".to_string(),
                    ),
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            "explain-utility" => {
                let args = request.arguments.unwrap_or_default();
                let utility = args
                    .get("utility")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(not provided)");

                let prompt = format!(
                    r#"Explain the Tailwind CSS utility: {utility}

Include:
- What it does
- CSS properties it sets
- Common use cases
- Related utilities

Use get_tailwind_docs to get detailed documentation."#
                );

                Ok(GetPromptResult {
                    description: Some("Explain a Tailwind CSS utility class".to_string()),
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            _ => Err(McpError::invalid_params(
                format!("Unknown prompt: {}", request.name),
                None,
            )),
        };

        std::future::ready(result)
    }
}

impl DraftkitServer {
    /// Create a new server instance (all data is embedded at compile time)
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            component_reader: ComponentReader::new(),
            fetcher: Arc::new(Mutex::new(None)),
        }
    }

    /// Get or initialize the component fetcher for on-demand fetching.
    ///
    /// Returns an error if not authenticated or session is expired.
    async fn get_fetcher(&self) -> Result<ComponentFetcher, FetchError> {
        let mut guard = self.fetcher.lock().await;

        if let Some(fetcher) = guard.as_ref() {
            return Ok(fetcher.clone());
        }

        // Get session cookie
        let session = auth::get_session()
            .map_err(|_| FetchError::NotAuthenticated)?
            .ok_or(FetchError::NotAuthenticated)?;

        if session.is_expired() {
            return Err(FetchError::SessionExpired);
        }

        // Initialize new fetcher
        let mut fetcher = ComponentFetcher::new(session.cookie);
        fetcher.init().await?;

        // Cache for future use
        *guard = Some(fetcher.clone());
        drop(guard);
        Ok(fetcher)
    }

    /// Fetch a component on-demand from TailwindPlus.
    ///
    /// Checks local cache first, then fetches from remote if needed.
    /// Shows a progress spinner on stderr during network requests.
    async fn fetch_component_on_demand(
        &self,
        uuid: &str,
        category: &str,
        subcategory: &str,
        sub_subcategory: &str,
        framework: Framework,
        mode: Mode,
    ) -> Result<String, FetchError> {
        // Check local cache first
        if let Some(code) = cache::get_cached(uuid, framework, mode) {
            return Ok(code);
        }

        // Show progress spinner on stderr (MCP uses stdout for JSON-RPC)
        let spinner = stderr_spinner(&format!("Fetching {sub_subcategory}..."));

        // Get or initialize fetcher
        let fetcher = match self.get_fetcher().await {
            Ok(f) => f,
            Err(e) => {
                spinner.finish_and_clear();
                return Err(e);
            }
        };

        // Fetch from TailwindPlus
        let result = fetcher
            .fetch_component(
                uuid,
                category,
                subcategory,
                sub_subcategory,
                framework,
                mode,
            )
            .await;

        // Clear spinner before returning
        match &result {
            Ok(_) => spinner.finish_with_message("✓ Fetched and cached"),
            Err(_) => spinner.finish_and_clear(),
        }

        result
    }
}

impl Default for DraftkitServer {
    fn default() -> Self {
        Self::new()
    }
}
