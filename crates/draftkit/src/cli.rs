//! CLI output helpers for styled terminal output.
//!
//! Provides consistent formatting for status messages, errors, tables,
//! and progress indicators across all draftkit commands.

// Style methods can't be const (Style::new() isn't const)
#![allow(clippy::missing_const_for_fn)]

use console::{Style, Term};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::time::Duration;

/// Terminal styling configuration based on --color flag.
#[derive(Debug, Clone)]
pub struct Styler {
    enabled: bool,
    term: Term,
}

impl Styler {
    /// Create a new styler based on the color mode.
    ///
    /// - "always": Force colors on
    /// - "never": Force colors off
    /// - "auto": Detect if terminal supports colors
    pub fn new(color_mode: &str) -> Self {
        let term = Term::stdout();
        let enabled = match color_mode {
            "always" => true,
            "never" => false,
            _ => term.is_term(), // "auto" or any other value
        };

        Self { enabled, term }
    }

    /// Check if colors are enabled.
    #[inline]
    pub fn colors_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the terminal reference.
    #[inline]
    pub fn term(&self) -> &Term {
        &self.term
    }

    // ─────────────────────────────────────────────────────────────
    // Styles
    // ─────────────────────────────────────────────────────────────

    /// Style for success messages (green).
    pub fn success(&self) -> Style {
        if self.enabled {
            Style::new().green().bold()
        } else {
            Style::new()
        }
    }

    /// Style for error messages (red).
    pub fn error(&self) -> Style {
        if self.enabled {
            Style::new().red().bold()
        } else {
            Style::new()
        }
    }

    /// Style for warning messages (yellow).
    pub fn warning(&self) -> Style {
        if self.enabled {
            Style::new().yellow().bold()
        } else {
            Style::new()
        }
    }

    /// Style for info/emphasis (cyan).
    pub fn info(&self) -> Style {
        if self.enabled {
            Style::new().cyan()
        } else {
            Style::new()
        }
    }

    /// Style for dimmed/secondary text.
    pub fn dim(&self) -> Style {
        if self.enabled {
            Style::new().dim()
        } else {
            Style::new()
        }
    }

    /// Style for bold text.
    pub fn bold(&self) -> Style {
        if self.enabled {
            Style::new().bold()
        } else {
            Style::new()
        }
    }

    /// Style for headers/titles.
    pub fn header(&self) -> Style {
        if self.enabled {
            Style::new().bold().underlined()
        } else {
            Style::new()
        }
    }

    // ─────────────────────────────────────────────────────────────
    // Formatted Output
    // ─────────────────────────────────────────────────────────────

    /// Print a success message with checkmark.
    pub fn print_success(&self, message: &str) {
        let check = self.success().apply_to("\u{2713}"); // ✓
        println!("{check} {message}");
    }

    /// Print an error message with X.
    pub fn print_error(&self, message: &str) {
        let x = self.error().apply_to("\u{2717}"); // ✗
        println!("{x} {message}");
    }

    /// Print a warning message with triangle.
    pub fn print_warning(&self, message: &str) {
        let warn = self.warning().apply_to("\u{26A0}"); // ⚠
        println!("{warn} {message}");
    }

    /// Print an info message with bullet.
    pub fn print_info(&self, message: &str) {
        let bullet = self.info().apply_to("\u{2022}"); // •
        println!("{bullet} {message}");
    }

    /// Print a section header with underline.
    pub fn print_header(&self, title: &str) {
        let styled = self.header().apply_to(title);
        let line = "\u{2500}".repeat(title.len()); // ─
        println!("{styled}");
        println!("{}", self.dim().apply_to(line));
    }

    /// Print a key-value pair with aligned formatting.
    pub fn print_kv(&self, key: &str, value: &str, width: usize) {
        let styled_key = self.dim().apply_to(format!("{key:>width$}:"));
        println!("{styled_key} {value}");
    }

    // ─────────────────────────────────────────────────────────────
    // Progress Indicators
    // ─────────────────────────────────────────────────────────────

    /// Create a spinner for indeterminate progress.
    pub fn spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();

        if self.enabled {
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("\u{25D0}\u{25D3}\u{25D1}\u{25D2} ") // ◐◓◑◒
                    .template("{spinner:.cyan} {msg}")
                    .expect("valid template"),
            );
        } else {
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("| / - \\ ")
                    .template("{spinner} {msg}")
                    .expect("valid template"),
            );
        }

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Create a progress bar for determinate progress.
    pub fn progress_bar(&self, len: u64, message: &str) -> ProgressBar {
        let pb = ProgressBar::new(len);

        if self.enabled {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{msg} {bar:30.cyan/blue} {pos}/{len} ({percent}%) [{elapsed_precise}]",
                    )
                    .expect("valid template")
                    .progress_chars("\u{2588}\u{2592}\u{2591}"), // █▒░
            );
        } else {
            #[allow(clippy::literal_string_with_formatting_args)] // indicatif template syntax
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} [{bar:30}] {pos}/{len} ({percent}%)")
                    .expect("valid template")
                    .progress_chars("#>-"),
            );
        }

        pb.set_message(message.to_string());
        pb
    }
}

impl Default for Styler {
    fn default() -> Self {
        Self::new("auto")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Standalone helpers for MCP server (outputs to stderr to avoid stdout pollution)
// ─────────────────────────────────────────────────────────────────────────────

/// Create a spinner that writes to stderr.
///
/// This is suitable for MCP servers where stdout is reserved for JSON-RPC.
/// The spinner uses a simple style that works in all terminals.
pub fn stderr_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr());

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.cyan} {msg}")
            .expect("valid template"),
    );

    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styler_new_auto() {
        let styler = Styler::new("auto");
        // In test environment, terminal detection should work
        let _ = styler.colors_enabled();
    }

    #[test]
    fn test_styler_new_always() {
        let styler = Styler::new("always");
        assert!(styler.colors_enabled());
    }

    #[test]
    fn test_styler_new_never() {
        let styler = Styler::new("never");
        assert!(!styler.colors_enabled());
    }

    #[test]
    fn test_styler_default() {
        let styler = Styler::default();
        // Should behave like "auto"
        let _ = styler.colors_enabled();
    }

    #[test]
    fn test_styles_return_style() {
        let styler = Styler::new("always");
        let _ = styler.success();
        let _ = styler.error();
        let _ = styler.warning();
        let _ = styler.info();
        let _ = styler.dim();
        let _ = styler.bold();
        let _ = styler.header();
    }

    #[test]
    fn test_spinner_creation() {
        let styler = Styler::new("never");
        let pb = styler.spinner("Loading...");
        pb.finish_and_clear();
    }

    #[test]
    fn test_progress_bar_creation() {
        let styler = Styler::new("never");
        let pb = styler.progress_bar(100, "Processing");
        pb.finish_and_clear();
    }

    #[test]
    fn test_stderr_spinner_creation() {
        let pb = stderr_spinner("Fetching...");
        pb.finish_and_clear();
    }
}
