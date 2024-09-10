use clap::{Parser, Subcommand};
use colored::Colorize;

mod helpers;
mod subcommands;

#[derive(Parser)]
#[command(version, about, long_about = None)] // It's read from Cargo.toml
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count, help = "Enable debug mode")]
    debug: u8,

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
    let cli: Cli = Cli::parse();

    // Initialized the logger accordingly to the verbosity level defined by clap arguments
    // TODO: use log::{error, warn, info, debug, trace};
    // as mentioned here: https://docs.rs/env_logger/latest/env_logger/#example
    env_logger::Builder::new()
        .filter_level(match cli.debug {
            0 => log::LevelFilter::Warn,  // Default
            1 => log::LevelFilter::Debug, // -d -> Debug
            2 => log::LevelFilter::Trace, // -dd -> Trace
            _ => log::LevelFilter::Trace, // All others go from Trace
        })
        .init();

    println!(
        "\n{} / The all-in-one scanner\n",
        String::from(" -> enygmah ").on_blue().white().bold()
    );

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
