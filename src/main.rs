use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[clap(name = "llama-mgr", version = "0.1.0", author = "SteelPh0enix <wojciech_olech@hotmail.com>", about = "Utility for managing llama.cpp tools")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download and install llama.cpp
    Install(commands::install::InstallCommand),
    /// Uninstall llama.cpp
    Uninstall(commands::uninstall::UninstallCommand),
    /// Set active instance of llama.cpp
    SetInstance(commands::set_instance::SetInstanceCommand),
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
        Commands::Install(_args) => {
            // Implementation here using args
            println!("Running install command");
        }
        Commands::Uninstall(args) => {
            // Implementation here using args
            println!("Running uninstall command for instance: {:?}", args.instance_name);
        }
        Commands::SetInstance(args) => {
            // Implementation here using args
            println!("Setting instance to: {}", args.instance_name);
        }
        Commands::Quantize(args) => {
            // Implementation here using args
            println!("Running quantize command with input: {}, output: {}, quant: {}", args.input, args.output, args.quant);
        }
        Commands::Convert(args) => {
            // Implementation here using args
            println!("Running convert command with input: {}, output: {}", args.input, args.output);
        }
        Commands::Server(args) => {
            // Implementation here using args
            println!("Running server command with model: {:?}", args.model);
        }
        Commands::Daemon(args) => {
            // Implementation here using args
            println!("Running daemon on {}:{}", args.bind, args.port);
        }
    }
}
