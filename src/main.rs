use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod commands;
mod external_tools;

#[derive(Parser)]
#[command(name = "llama-mgr", version, author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download and install llama.cpp
    Install(commands::install::InstallCommand),
    /// Uninstall llama.cpp
    Uninstall(commands::uninstall::UninstallCommand),
    /// Run llama-quantize
    Quantize(commands::quantize::QuantizeCommand),
    /// Convert a raw huggingface model to GGUF
    Convert(commands::convert::ConvertCommand),
    /// Run llama-server instances
    Server(commands::server::ServerCommand),
    /// Start the llama-mgr in daemon mode
    Daemon(commands::daemon::DaemonCommand),
}

impl From<&Commands> for &str {
    fn from(value: &Commands) -> &'static str {
        match value {
            Commands::Install(_) => "install",
            Commands::Uninstall(_) => "uninstall",
            Commands::Quantize(_) => "quantize",
            Commands::Convert(_) => "convert",
            Commands::Server(_) => "server",
            Commands::Daemon(_) => "daemon",
        }
    }
}

fn main() -> ExitCode {
    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .parse_default_env()
        .init();

    let cli = Cli::parse();
    let command_name: &str = (&cli.command).into();

    let result = match cli.command {
        Commands::Install(args) => commands::install::run(args),
        Commands::Uninstall(args) => commands::uninstall::run(args),
        Commands::Quantize(args) => commands::quantize::run(args),
        Commands::Convert(args) => commands::convert::run(args),
        Commands::Server(args) => commands::server::run(args),
        Commands::Daemon(args) => commands::daemon::run(args),
    };

    if result.is_err() {
        let error = result.expect_err("Couldn't unwrap application's error code!");

        log::error!(
            "Error occurred while executing command '{}' - {}",
            String::from(command_name),
            error.message,
        );

        error.exit_code
    } else {
        ExitCode::SUCCESS
    }
}
