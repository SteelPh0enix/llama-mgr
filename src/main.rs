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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install(args) => commands::install::run(args),
        Commands::Uninstall(args) => commands::uninstall::run(args),
        Commands::Quantize(args) => commands::quantize::run(args),
        Commands::Convert(args) => commands::convert::run(args),
        Commands::Server(args) => commands::server::run(args),
        Commands::Daemon(args) => commands::daemon::run(args),
    }
}
