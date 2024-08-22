use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod helpers;
mod subcommands;

#[derive(Parser)]
#[command(version, about, long_about = None)] // It's read from Cargo.toml
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum Commands {
    /// Analyze an asset
    Scan { asset: String },
    /// Download needed tools and Docker Images
    Install {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialized the logger accordingly to the verbosity level defined by clap arguments
    // TODO: use log::{error, warn, info, debug, trace};
    // as mentioned here: https://docs.rs/env_logger/latest/env_logger/#example
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Some(Commands::Scan { asset }) => {
            subcommands::scan::analyze::analyze(asset).await;
        }
        Some(Commands::Install {}) => {
            helpers::install_tools::install_tools().await;
        }
        None => {} // Will automatically show the help message from Clap
    }
}
