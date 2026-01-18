//! Authentication command implementation.
//!
//! Uses Playwright to open a browser for TailwindPlus login, then captures
//! the session cookie for use when fetching components. This approach means
//! we never handle user credentials directly - they enter them in the browser.

use std::fs;
use std::io::{self, Write as _};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use clap::Args;
use draftkit_core::data_dir::data_dir;
use playwright_rs::Playwright;
use serde::{Deserialize, Serialize};

use crate::cli::Styler;

/// Session file name within data directory
const SESSION_FILE: &str = "session.json";

/// TailwindPlus login URL
const LOGIN_URL: &str = "https://tailwindcss.com/plus/login";

/// URL patterns that indicate successful authentication
const AUTH_SUCCESS_PATTERNS: &[&str] = &[
    "tailwindcss.com/plus/ui",
    "tailwindcss.com/plus/templates",
    "tailwindcss.com/plus/documentation",
];

/// Laravel session cookie name
const SESSION_COOKIE_NAME: &str = "laravel_session";

/// Session data stored in keyring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// The Laravel session cookie value
    pub cookie: String,
    /// Unix timestamp when session expires (if known)
    pub expires_at: Option<i64>,
    /// Domain the cookie is valid for
    pub domain: String,
}

impl Session {
    /// Check if the session appears expired.
    #[allow(clippy::option_if_let_else)]
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            expires <= now
        } else {
            // No expiry info - assume valid (will fail on use if not)
            false
        }
    }

    /// Check if session is expiring soon (within 24 hours).
    #[allow(clippy::option_if_let_else)]
    pub fn is_expiring_soon(&self) -> bool {
        if let Some(expires) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let one_day = 24 * 60 * 60;
            expires <= now + one_day
        } else {
            false
        }
    }
}

#[derive(Args)]
pub struct AuthArgs {
    /// Remove stored session
    #[arg(long)]
    pub logout: bool,

    /// Show session status
    #[arg(long)]
    pub status: bool,

    /// Force re-authentication even if session exists
    #[arg(long)]
    pub refresh: bool,
}

/// Get the path to the session file
fn session_path() -> Result<camino::Utf8PathBuf> {
    data_dir()
        .map(|dir| dir.join(SESSION_FILE))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}

/// Get stored session if it exists
pub fn get_session() -> Result<Option<Session>> {
    let path = session_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(&path).context("Failed to read session file")?;
    let session: Session = serde_json::from_str(&json).context("Failed to parse session file")?;
    Ok(Some(session))
}

/// Store session in data directory
fn store_session(session: &Session) -> Result<()> {
    let path = session_path()?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create data directory")?;
    }

    let json = serde_json::to_string_pretty(session).context("Failed to serialize session")?;
    fs::write(&path, json).context("Failed to write session file")?;
    Ok(())
}

/// Remove stored session
fn remove_session() -> Result<()> {
    let path = session_path()?;

    if path.exists() {
        fs::remove_file(&path).context("Failed to remove session file")?;
    }
    Ok(())
}

/// Prompt for manual cookie input (fallback when browser unavailable)
fn prompt_manual_cookie() -> Result<String> {
    println!();
    println!("Manual authentication fallback");
    println!("────────────────────────────────────────────────────────");
    println!();
    println!("1. Open your browser and go to: {LOGIN_URL}");
    println!("2. Log in with your TailwindPlus account");
    println!("3. Open browser DevTools (F12 or Cmd+Option+I)");
    println!("4. Go to: Application → Cookies → tailwindcss.com");
    println!("5. Copy the value of '{SESSION_COOKIE_NAME}'");
    println!();

    print!("Paste session cookie: ");
    io::stdout().flush()?;

    let cookie = rpassword::read_password()?;
    let cookie = cookie.trim().to_string();

    if cookie.is_empty() {
        bail!("Cookie cannot be empty");
    }

    if cookie.len() < 50 {
        bail!(
            "Cookie value seems too short. Make sure you copied the full '{SESSION_COOKIE_NAME}' value."
        );
    }

    Ok(cookie)
}

/// Authenticate using Playwright browser automation
async fn browser_auth(styler: &Styler) -> Result<Session> {
    use playwright_rs::api::LaunchOptions;

    let spinner = styler.spinner("Preparing browser...");

    let playwright = Playwright::launch()
        .await
        .context("Failed to initialize Playwright. Run: npx playwright install")?;

    spinner.finish_and_clear();

    println!();
    println!("Opening browser for TailwindPlus login...");
    println!();
    println!("A browser window will open. Please log in with your");
    println!("TailwindPlus account credentials.");
    println!();

    // Launch visible browser (not headless - user needs to interact)
    let launch_options = LaunchOptions {
        headless: Some(false),
        ..Default::default()
    };

    let browser = playwright
        .chromium()
        .launch_with_options(launch_options)
        .await
        .context("Failed to launch browser")?;

    let context = browser
        .new_context()
        .await
        .context("Failed to create browser context")?;

    let page = context.new_page().await.context("Failed to create page")?;

    // Navigate to login page
    page.goto(LOGIN_URL, None)
        .await
        .context("Failed to navigate to login page")?;

    let spinner = styler.spinner("Waiting for login...");

    // Wait for successful authentication (URL change)
    let mut authenticated = false;
    let timeout = Duration::from_secs(300); // 5 minute timeout
    let start = std::time::Instant::now();

    while !authenticated && start.elapsed() < timeout {
        tokio::time::sleep(Duration::from_millis(500)).await;

        let url = page.url();
        for pattern in AUTH_SUCCESS_PATTERNS {
            if url.contains(pattern) && !url.contains("login") {
                authenticated = true;
                break;
            }
        }
    }

    spinner.finish_and_clear();

    if !authenticated {
        browser.close().await.ok();
        bail!("Authentication timed out. Please try again.");
    }

    // Extract cookies using storage_state()
    let state = context
        .storage_state()
        .await
        .context("Failed to get storage state from browser")?;

    // Find the Laravel session cookie
    let session_cookie = state
        .cookies
        .iter()
        .find(|c| c.name == SESSION_COOKIE_NAME)
        .ok_or_else(|| anyhow::anyhow!("Session cookie not found after login"))?;

    let session = Session {
        cookie: session_cookie.value.clone(),
        expires_at: if session_cookie.expires < 0.0 {
            None // Session cookie
        } else {
            Some(session_cookie.expires as i64)
        },
        domain: session_cookie.domain.clone(),
    };

    // Close browser
    browser.close().await.ok();

    Ok(session)
}

/// Run the auth command
pub async fn cmd_auth(args: AuthArgs, styler: &Styler) -> Result<()> {
    // Handle --status flag
    if args.status {
        return show_status(styler);
    }

    // Handle --logout flag
    if args.logout {
        remove_session()?;
        styler.print_success("Session removed.");
        return Ok(());
    }

    // Check for existing session (unless --refresh)
    if !args.refresh
        && let Some(session) = get_session()?
    {
        if !session.is_expired() {
            styler.print_success("Already authenticated!");
            println!();
            if session.is_expiring_soon() {
                styler
                    .print_warning("Session expires soon. Run `draftkit auth --refresh` to renew.");
            } else {
                println!("Run `draftkit auth --status` to see session details.");
                println!("Run `draftkit auth --refresh` to get a new session.");
            }
            return Ok(());
        }
        // Session expired, continue to re-auth
        styler.print_warning("Session expired. Starting re-authentication...");
    }

    // Try browser auth first, fall back to manual
    let session = match browser_auth(styler).await {
        Ok(s) => s,
        Err(e) => {
            styler.print_warning(&format!("Browser automation failed: {e}"));
            println!();

            // Fall back to manual cookie entry
            let cookie = prompt_manual_cookie()?;
            Session {
                cookie,
                expires_at: None,
                domain: "tailwindcss.com".into(),
            }
        }
    };

    // Store session
    store_session(&session)?;

    println!();
    styler.print_success("Authentication successful!");

    if let Some(expires) = session.expires_at {
        let expires_dt = chrono::DateTime::from_timestamp(expires, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
            .unwrap_or_else(|| "unknown".into());
        println!("  Session valid until: {expires_dt}");
    }

    println!();
    println!("Tip: Run `draftkit auth --status` to check session validity.");

    Ok(())
}

/// Show authentication status
fn show_status(styler: &Styler) -> Result<()> {
    styler.print_header("Session Status");
    println!();

    match get_session()? {
        Some(session) => {
            if session.is_expired() {
                styler.print_error("Session expired");
                println!();
                println!("Run `draftkit auth` to log in again.");
            } else if session.is_expiring_soon() {
                styler.print_warning("Session expiring soon");
                println!();
                if let Some(expires) = session.expires_at {
                    let expires_dt = chrono::DateTime::from_timestamp(expires, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
                        .unwrap_or_else(|| "unknown".into());
                    styler.print_kv("Expires", &expires_dt, 12);
                }
                println!();
                println!("Run `draftkit auth --refresh` to renew.");
            } else {
                styler.print_success("Session valid");
                println!();
                if let Some(expires) = session.expires_at {
                    let expires_dt = chrono::DateTime::from_timestamp(expires, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
                        .unwrap_or_else(|| "unknown".into());
                    styler.print_kv("Expires", &expires_dt, 12);
                } else {
                    styler.print_kv("Expires", "session cookie", 12);
                }
                styler.print_kv("Domain", &session.domain, 12);
            }
        }
        None => {
            styler.print_info("Not authenticated");
            println!();
            println!("Run `draftkit auth` to log in to your TailwindPlus account.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_args_default() {
        let _args = AuthArgs {
            logout: false,
            status: false,
            refresh: false,
        };
    }

    #[test]
    fn test_session_file_name() {
        assert_eq!(SESSION_FILE, "session.json");
    }

    #[test]
    fn test_session_serialization() {
        let session = Session {
            cookie: "test_cookie_value_that_is_long_enough".into(),
            expires_at: Some(1_234_567_890),
            domain: "example.com".into(),
        };

        let json = serde_json::to_string(&session).unwrap();
        let parsed: Session = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.cookie, "test_cookie_value_that_is_long_enough");
        assert_eq!(parsed.expires_at, Some(1_234_567_890));
        assert_eq!(parsed.domain, "example.com");
    }

    #[test]
    fn test_session_is_expired() {
        let expired = Session {
            cookie: "x".into(),
            expires_at: Some(1),
            domain: "x".into(),
        };
        assert!(expired.is_expired());

        let future = Session {
            cookie: "x".into(),
            expires_at: Some(i64::MAX / 2),
            domain: "x".into(),
        };
        assert!(!future.is_expired());

        let unknown = Session {
            cookie: "x".into(),
            expires_at: None,
            domain: "x".into(),
        };
        assert!(!unknown.is_expired());
    }

    #[test]
    fn test_session_is_expiring_soon() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let expiring_soon = Session {
            cookie: "x".into(),
            expires_at: Some(now + 12 * 60 * 60),
            domain: "x".into(),
        };
        assert!(expiring_soon.is_expiring_soon());

        let not_expiring_soon = Session {
            cookie: "x".into(),
            expires_at: Some(now + 48 * 60 * 60),
            domain: "x".into(),
        };
        assert!(!not_expiring_soon.is_expiring_soon());

        let unknown = Session {
            cookie: "x".into(),
            expires_at: None,
            domain: "x".into(),
        };
        assert!(!unknown.is_expiring_soon());
    }
}
