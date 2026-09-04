mod commands;
mod log;

use clap::{Parser, Subcommand};
use log::setup_logging;

#[derive(Parser)]
#[command(name = "local-ai")]
#[command(about = "Local autonomous AI agent platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List available and cached models
    ListModels {
        #[arg(long)]
        remote: bool,
    },

    /// Select and download a model
    SelectModel { model_id: String },

    /// Run diagnostic checks
    Doctor,

    /// Test offline operation
    OfflineTest,

    /// Run inference with a prompt
    Infer { prompt: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    setup_logging(cli.verbose);

    match cli.command {
        Commands::ListModels { remote } => commands::list_models(remote).await?,
        Commands::SelectModel { model_id } => commands::select_model(&model_id).await?,
        Commands::Doctor => commands::doctor().await?,
        Commands::OfflineTest => commands::offline_test().await?,
        Commands::Infer { prompt } => commands::infer(&prompt).await?,
    }

    Ok(())
}
