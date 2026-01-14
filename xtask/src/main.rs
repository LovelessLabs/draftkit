mod commands;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "xtask")]
#[command(about = "Project maintenance tasks")]
struct Xtask {
    #[command(subcommand)]
    command: Task,
}

#[derive(Subcommand, Debug)]
enum Task {
    /// Generate benchmark harnesses from benches/benchmarks.kdl.
    GenBenchmarks,

    /// Run benchmarks (divan, gungraun, hyperfine).
    Bench(commands::bench::BenchArgs),

    /// Generate shell completions for the draftkit CLI.
    Completions(commands::completions::CompletionsArgs),

    /// Generate manpages for the draftkit CLI.
    Man(commands::man::ManArgs),

    /// Build and install the draftkit CLI into ~/.bin for local testing.
    Install(commands::install::InstallArgs),

    /// Generate sqlite_vec embeddings from NDJSON component files.
    GenEmbeddings(commands::gen_embeddings::GenEmbeddingsArgs),
}

fn main() -> Result<(), String> {
    let task = Xtask::parse();
    match task.command {
        Task::GenBenchmarks => commands::gen_benchmarks::cmd_gen_benchmarks(),
        Task::Bench(args) => commands::bench::cmd_bench(args),
        Task::Completions(args) => commands::completions::cmd_completions(args),
        Task::Man(args) => commands::man::cmd_man(args),
        Task::Install(args) => commands::install::cmd_install(args),
        Task::GenEmbeddings(args) => {
            // Initialize tracing for the embeddings generator
            tracing_subscriber::registry()
                .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
                .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
                .init();

            // Build a tokio runtime and run the async command
            tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create tokio runtime: {e}"))?
                .block_on(commands::gen_embeddings::cmd_gen_embeddings(args))
                .map_err(|e| format!("{e:?}"))
        }
    }
}

pub fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf()
}
