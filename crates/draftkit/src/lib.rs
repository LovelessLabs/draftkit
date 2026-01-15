//! Library interface for draftkit CLI - used for documentation generation

pub mod cli;
pub mod commands;
pub mod server;

// Re-export domain types from draftkit-core for convenience
pub use draftkit_core::{
    Catalog, CatalystComponent, CatalystLanguage, ComponentReader, Framework, Mode,
    TailwindVersion, catalyst, components, docs, elements,
};

use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "draftkit")]
#[command(about = "MCP server for Tailwind Plus members.", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Run as if started in DIR
    #[arg(short = 'C', long, global = true)]
    pub chdir: Option<PathBuf>,

    /// Only print errors (suppresses warnings/info)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// More detail (repeatable; e.g. -vv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Colorize output: auto|always|never
    #[arg(long, global = true, default_value = "auto")]
    pub color: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with TailwindPlus
    Auth(commands::auth::AuthArgs),
    /// Manage the local component cache
    Cache(commands::cache::CacheArgs),
    /// Show package information
    Info(commands::info::InfoArgs),
    /// Run the MCP server
    Serve(commands::serve::ServeArgs),
}

/// Returns the clap command for documentation generation
pub fn command() -> clap::Command {
    Cli::command()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_command_has_expected_name() {
        let cmd = Cli::command();
        assert_eq!(cmd.get_name(), "draftkit");
    }

    #[test]
    fn cli_has_info_subcommand() {
        let cmd = Cli::command();
        assert!(cmd.get_subcommands().any(|c| c.get_name() == "info"));
    }

    #[test]
    fn cli_has_serve_subcommand() {
        let cmd = Cli::command();
        assert!(cmd.get_subcommands().any(|c| c.get_name() == "serve"));
    }

    #[test]
    fn cli_has_auth_subcommand() {
        let cmd = Cli::command();
        assert!(cmd.get_subcommands().any(|c| c.get_name() == "auth"));
    }

    #[test]
    fn command_function_returns_valid_command() {
        let cmd = command();
        assert_eq!(cmd.get_name(), "draftkit");
    }

    #[test]
    fn cli_parse_info_command() {
        let result = Cli::try_parse_from(["draftkit", "info"]);
        assert!(result.is_ok());
        if let Ok(cli) = result {
            assert!(matches!(cli.command, Commands::Info(_)));
        }
    }

    #[test]
    fn cli_parse_serve_command() {
        let result = Cli::try_parse_from(["draftkit", "serve"]);
        assert!(result.is_ok());
        if let Ok(cli) = result {
            assert!(matches!(cli.command, Commands::Serve(_)));
        }
    }

    #[test]
    fn cli_parse_auth_command() {
        let result = Cli::try_parse_from(["draftkit", "auth"]);
        assert!(result.is_ok());
        if let Ok(cli) = result {
            assert!(matches!(cli.command, Commands::Auth(_)));
        }
    }

    #[test]
    fn cli_parse_auth_status() {
        let result = Cli::try_parse_from(["draftkit", "auth", "--status"]);
        assert!(result.is_ok());
    }

    #[test]
    fn cli_parse_auth_logout() {
        let result = Cli::try_parse_from(["draftkit", "auth", "--logout"]);
        assert!(result.is_ok());
    }

    #[test]
    fn cli_parse_auth_refresh() {
        let result = Cli::try_parse_from(["draftkit", "auth", "--refresh"]);
        assert!(result.is_ok());
    }

    #[test]
    fn cli_global_flags_parsed() {
        let result = Cli::try_parse_from(["draftkit", "-q", "-vv", "--color", "never", "info"]);
        assert!(result.is_ok());
        if let Ok(cli) = result {
            assert!(cli.quiet);
            assert_eq!(cli.verbose, 2);
            assert_eq!(cli.color, "never");
        }
    }

    #[test]
    fn cli_chdir_flag_parsed() {
        let result = Cli::try_parse_from(["draftkit", "-C", "/tmp", "info"]);
        assert!(result.is_ok());
        if let Ok(cli) = result {
            assert_eq!(cli.chdir, Some(PathBuf::from("/tmp")));
        }
    }

    #[test]
    fn cli_has_cache_subcommand() {
        let cmd = Cli::command();
        assert!(cmd.get_subcommands().any(|c| c.get_name() == "cache"));
    }

    #[test]
    fn cli_parse_cache_command() {
        let result = Cli::try_parse_from(["draftkit", "cache"]);
        assert!(result.is_ok());
        if let Ok(cli) = result {
            assert!(matches!(cli.command, Commands::Cache(_)));
        }
    }

    #[test]
    fn cli_parse_cache_with_stats() {
        let result = Cli::try_parse_from(["draftkit", "cache", "--stats"]);
        assert!(result.is_ok());
    }

    #[test]
    fn cli_parse_cache_with_clear() {
        let result = Cli::try_parse_from(["draftkit", "cache", "--clear"]);
        assert!(result.is_ok());
    }

    #[test]
    fn cli_parse_cache_with_path() {
        let result = Cli::try_parse_from(["draftkit", "cache", "--path"]);
        assert!(result.is_ok());
    }
}
