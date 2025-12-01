use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod commands;
mod config;
mod error;
mod external_tools;

use crate::error::RuntimeError;
use config::Config;

#[derive(Parser)]
#[command(name = "llama-mgr", version, author, about)]
struct Cli {
    /// List of available commands
    #[command(subcommand)]
    command: Commands,

    /// Path to the configuration file
    #[arg(short, long, global = true, default_value = "~/.llama-mgr/config.toml")]
    config: String,

    /// Profile to use
    #[arg(short, long, global = true)]
    profile: Option<String>,
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
fn create_default_config_file(config_path: &Path) -> std::result::Result<(), RuntimeError> {
    if config_path.exists() {
        return Err(RuntimeError::new(
            format!("Config file already exists at: {:?}", config_path),
            exitcode::CANTCREAT as u8,
        ));
    }

    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                RuntimeError::new(
                    format!("Failed to create config directory: {}", e),
                    exitcode::CANTCREAT as u8,
                )
            })?;
        }
    }

    let default_config = Config::default();
    let toml_content = toml::to_string(&default_config).map_err(|e| {
        RuntimeError::new(
            format!("Failed to serialize default config: {}", e),
            exitcode::SOFTWARE as u8,
        )
    })?;

    std::fs::write(config_path, toml_content).map_err(|e| {
        RuntimeError::new(
            format!("Failed to write config file: {}", e),
            exitcode::IOERR as u8,
        )
    })?;

    Ok(())
}

/// Load configuration from file
fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let expanded_path = shellexpand::tilde(config_path);
    let config_path = PathBuf::from(expanded_path.as_ref());

    if !config_path.exists() {
        create_default_config_file(&config_path)?;
    }

    let config_str = std::fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}

fn main() -> ExitCode {
    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .parse_default_env()
        .init();

    let cli = Cli::parse();

    // Load configuration
    let config = match load_config(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            return ExitCode::from(exitcode::NOINPUT as u8);
        }
    };

    // Apply profile override from command line if provided
    let profile_name: &str = if let Some(profile) = &cli.profile {
        profile
    } else {
        &config.config.default_profile
    };

    // Get the selected profile
    let profile = match config.get_profile(Some(profile_name)) {
        Some(profile) => profile,
        None => {
            log::error!("Profile '{}' not found in configuration", profile_name);
            return ExitCode::from(exitcode::UNAVAILABLE as u8);
        }
    };

    log::info!("Using profile: {}", profile_name);

    let command_name: &str = (&cli.command).into();

    let result = match cli.command {
        Commands::Install(args) => commands::install::run(args, &config, profile),
        Commands::Uninstall(args) => commands::uninstall::run(args, &config, profile),
        Commands::Quantize(args) => commands::quantize::run(args, &config, profile),
        Commands::Convert(args) => commands::convert::run(args, &config, profile),
        Commands::Server(args) => commands::server::run(args, &config, profile),
        Commands::Daemon(args) => commands::daemon::run(args, &config, profile),
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
