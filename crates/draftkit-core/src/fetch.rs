//! On-demand component fetching from TailwindPlus.
//!
//! Fetches component code using authenticated session cookies.
//! TailwindPlus uses Inertia.js, so we need specific headers to get JSON responses.

use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

use crate::cache;
use crate::components::{Framework, Mode};

/// Base URL for TailwindPlus
const BASE_URL: &str = "https://tailwindcss.com/plus";

/// User agent to use for requests
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36";

/// Error type for fetch operations
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Not authenticated. Run `draftkit auth` to log in.")]
    NotAuthenticated,

    #[error("Session expired. Run `draftkit auth` to refresh.")]
    SessionExpired,

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    Parse(String),

    #[error("Cache error: {0}")]
    Cache(#[from] std::io::Error),
}

/// Inertia.js response structure (simplified)
#[derive(Debug, Deserialize)]
struct InertiaResponse {
    props: InertiaProps,
}

#[derive(Debug, Deserialize)]
struct InertiaProps {
    subcategory: Option<SubcategoryData>,
}

#[derive(Debug, Deserialize)]
struct SubcategoryData {
    components: Vec<ComponentData>,
}

#[derive(Debug, Deserialize)]
struct ComponentData {
    uuid: String,
    #[allow(dead_code)]
    name: String,
    snippet: SnippetData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SnippetData {
    code: String,
    #[allow(dead_code)]
    language: String,
    #[allow(dead_code)]
    version: String,
    mode: String,
    #[allow(dead_code)]
    supports_dark_mode: bool,
    #[allow(dead_code)]
    preview: Option<String>,
}

/// Build headers for Inertia.js requests
fn inertia_headers(xsrf_token: &str, inertia_version: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        "accept",
        HeaderValue::from_static("text/html, application/xhtml+xml, application/json"),
    );
    headers.insert("x-inertia", HeaderValue::from_static("true"));
    headers.insert(
        "x-requested-with",
        HeaderValue::from_static("XMLHttpRequest"),
    );
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"macOS\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));

    if let Ok(v) = HeaderValue::from_str(inertia_version) {
        headers.insert("x-inertia-version", v);
    }
    if let Ok(v) = HeaderValue::from_str(xsrf_token) {
        headers.insert("x-xsrf-token", v);
    }

    headers
}

/// Convert category path to URL slug
/// e.g., "Application UI" -> "application-ui"
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

/// Build subcategory URL from category path
/// e.g., ["Application UI", "Forms", "Input Groups"] -> "/plus/ui-blocks/application-ui/forms/input-groups"
pub fn subcategory_url(category: &str, subcategory: &str, sub_subcategory: &str) -> String {
    let cat_slug = slugify(category);
    let sub_slug = slugify(subcategory);
    let subsub_slug = slugify(sub_subcategory);

    format!("{BASE_URL}/ui-blocks/{cat_slug}/{sub_slug}/{subsub_slug}")
}

/// Component fetcher with session management
#[derive(Clone)]
pub struct ComponentFetcher {
    client: reqwest::Client,
    session_cookie: String,
    xsrf_token: Option<String>,
    inertia_version: Option<String>,
}

impl ComponentFetcher {
    /// Create a new fetcher with the given session cookie
    pub fn new(session_cookie: String) -> Self {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            session_cookie,
            xsrf_token: None,
            inertia_version: None,
        }
    }

    /// Initialize the fetcher by getting XSRF token and Inertia version
    pub async fn init(&mut self) -> Result<(), FetchError> {
        // Make initial request to get cookies and version
        let resp = self
            .client
            .get(BASE_URL)
            .header("cookie", format!("laravel_session={}", self.session_cookie))
            .send()
            .await?;

        // Extract XSRF token from cookies
        if let Some(cookies) = resp.headers().get("set-cookie")
            && let Ok(cookie_str) = cookies.to_str()
            && let Some(start) = cookie_str.find("XSRF-TOKEN=")
        {
            let token_start = start + 11;
            if let Some(end) = cookie_str[token_start..].find(';') {
                let token = &cookie_str[token_start..token_start + end];
                // URL decode the token
                self.xsrf_token = Some(urlencoding::decode(token).unwrap_or_default().into_owned());
            }
        }

        // Extract Inertia version from HTML
        let html = resp.text().await?;
        if let Some(start) = html.find("data-page=\"") {
            let json_start = start + 11;
            if let Some(end) = html[json_start..].find('"') {
                let escaped = &html[json_start..json_start + end];
                let unescaped = escaped.replace("&quot;", "\"");
                if let Ok(page) = serde_json::from_str::<serde_json::Value>(&unescaped)
                    && let Some(version) = page.get("version").and_then(|v| v.as_str())
                {
                    self.inertia_version = Some(version.to_string());
                }
            }
        }

        Ok(())
    }

    /// Fetch a subcategory page and return all components
    async fn fetch_subcategory(
        &self,
        url: &str,
    ) -> Result<HashMap<String, ComponentData>, FetchError> {
        let xsrf = self.xsrf_token.as_deref().unwrap_or("");
        let version = self.inertia_version.as_deref().unwrap_or("");

        let resp = self
            .client
            .get(url)
            .header("cookie", format!("laravel_session={}", self.session_cookie))
            .headers(inertia_headers(xsrf, version))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(FetchError::Http(resp.error_for_status().unwrap_err()));
        }

        let data: InertiaResponse = resp
            .json()
            .await
            .map_err(|e| FetchError::Parse(e.to_string()))?;

        let components = data
            .props
            .subcategory
            .ok_or_else(|| FetchError::Parse("No subcategory data in response".into()))?
            .components;

        let mut map = HashMap::new();
        for comp in components {
            map.insert(comp.uuid.clone(), comp);
        }

        Ok(map)
    }

    /// Fetch a specific component by UUID from a subcategory
    pub async fn fetch_component(
        &self,
        uuid: &str,
        category: &str,
        subcategory: &str,
        sub_subcategory: &str,
        framework: Framework,
        mode: Mode,
    ) -> Result<String, FetchError> {
        // Check cache first
        if let Some(code) = cache::get_cached(uuid, framework, mode) {
            return Ok(code);
        }

        // Build URL and fetch
        let url = subcategory_url(category, subcategory, sub_subcategory);
        let components = self.fetch_subcategory(&url).await?;

        // Find the component
        let comp = components
            .get(uuid)
            .ok_or_else(|| FetchError::ComponentNotFound(uuid.to_string()))?;

        // Check mode matches
        let mode_str = mode.as_str();
        if comp.snippet.mode != mode_str {
            return Err(FetchError::ComponentNotFound(format!(
                "{uuid} mode {mode_str} (found {})",
                comp.snippet.mode
            )));
        }

        let code = comp.snippet.code.clone();

        // Cache the result
        cache::store_cached(uuid, framework, mode, &code)?;

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Application UI"), "application-ui");
        assert_eq!(slugify("Forms"), "forms");
        assert_eq!(slugify("Input Groups"), "input-groups");
        assert_eq!(slugify("E-commerce"), "e-commerce");
    }

    #[test]
    fn test_subcategory_url() {
        let url = subcategory_url("Application UI", "Forms", "Input Groups");
        assert_eq!(
            url,
            "https://tailwindcss.com/plus/ui-blocks/application-ui/forms/input-groups"
        );
    }

    #[test]
    fn test_subcategory_url_ecommerce() {
        let url = subcategory_url("Ecommerce", "Components", "Product Lists");
        assert_eq!(
            url,
            "https://tailwindcss.com/plus/ui-blocks/ecommerce/components/product-lists"
        );
    }
}
