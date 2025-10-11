pub mod convert;
pub mod daemon;
pub mod install;
pub mod quantize;
pub mod server;
pub mod uninstall;

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Args;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct CommandError {
    pub message: String,
    pub exit_code: ExitCode,
}

impl CommandError {
    pub fn new(message: String, exit_code: u8) -> Self {
        Self {
            message,
            exit_code: ExitCode::from(exit_code),
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Args, Debug)]
pub struct CommonArguments {
    #[arg(long, default_value = "~/.llama-mgr/config.toml")]
    /// Data directory for installation files and configuration.
    pub config: PathBuf,

    #[arg(long, default_value = "vulkan")]
    /// Name of installation profile to use.
    pub profile: String,
}

impl CommonArguments {
    pub fn create_default_config_file(&self) -> Result<()> {
        if self.config.exists() {
            return Err(CommandError::new(
                format!("Config file already exists at: {:?}", self.config),
                exitcode::CANTCREAT as u8,
            ));
        }

        if let Some(parent) = self.config.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    CommandError::new(
                        format!("Failed to create config directory: {}", e),
                        exitcode::CANTCREAT as u8,
                    )
                })?;
            }
        }

        let default_config = crate::config::Config::default();
        let toml_content = toml::to_string(&default_config).map_err(|e| {
            CommandError::new(
                format!("Failed to serialize default config: {}", e),
                exitcode::SOFTWARE as u8,
            )
        })?;

        fs::write(&self.config, toml_content).map_err(|e| {
            CommandError::new(
                format!("Failed to write config file: {}", e),
                exitcode::IOERR as u8,
            )
        })?;

        Ok(())
    }
}
