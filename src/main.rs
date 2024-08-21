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
    /// Analyze the source code of a project
    Code { path: String },
    /// Analyze a web application
    Web { url: String },
    /// Analyze an API endpoints
    Api { url: String },
    /// Analyze a mobile app binary, such as an .apk
    Mobile { path: String },
    /// Analyze a compiled binary, such as an .exe
    Binary { path: String },
}

fn main() {
    let cli = Cli::parse();

    // Initialized the logger accordingly to the verbosity level defined by clap arguments
    // TODO: use log::{debug, error, log_enabled, info, Level};
    // as mentioned here: https://docs.rs/env_logger/latest/env_logger/#example
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Some(Commands::Code { path }) => {
            subcommands::code::analyze::analyze(path);
        }
        Some(Commands::Web { url }) => {
            subcommands::web::analyze::analyze(url);
        }
        Some(Commands::Api { url }) => {
            subcommands::api::analyze::analyze(url);
        }
        Some(Commands::Mobile { path }) => {
            subcommands::mobile::analyze::analyze(path);
        }
        Some(Commands::Binary { path }) => {
            subcommands::binary::analyze::analyze(path);
        }
        None => {} // Will automatically show the help message from Clap
    }
}
