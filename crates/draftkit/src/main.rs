//! draftkit CLI
#![deny(unsafe_code)]

use clap::Parser;
use draftkit::{Cli, Commands, commands};
use draftkit_core::config::ConfigLoader;

mod observability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(ref dir) = cli.chdir {
        std::env::set_current_dir(dir)?;
    }

    let config = ConfigLoader::new()
        .with_project_search(std::env::current_dir()?)
        .load()?;

    let obs_config = observability::ObservabilityConfig::from_env_with_overrides(
        config.otel_endpoint.clone(),
        config
            .log_dir
            .as_ref()
            .map(|dir| dir.as_std_path().to_path_buf()),
    );
    let env_filter = observability::env_filter(cli.quiet, cli.verbose, config.log_level.as_str());
    let _guard = observability::init_observability(&obs_config, env_filter)?;

    let span = observability::correlated_span("cli", &obs_config);
    observability::record_otel_ids(&span);

    let result = match cli.command {
        Commands::Info(args) => span.in_scope(|| commands::info::cmd_info(args)),
        Commands::Serve(args) => {
            // Serve command runs async and needs different observability setup
            // MCP stdio requires stdout to be clean - only stderr for logs
            commands::serve::cmd_serve(args).await
        }
    };

    observability::shutdown_tracing();

    result
}
