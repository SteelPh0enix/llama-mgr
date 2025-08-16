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
    #[arg(long, default_value = "~/.llama-mgr")]
    /// Data directory for installation files and configuration.
    pub data_dir: String,

    #[arg(long, default_value = "global")]
    /// Name of installation profile to use.
    pub instance: String,
}

impl CommonArguments {
    pub fn get_data_dir(&self) -> PathBuf {
        PathBuf::from(shellexpand::tilde(&self.data_dir).to_string())
    }

    pub fn data_dir_exists(&self) -> bool {
        self.get_data_dir().exists()
    }

    pub fn create_data_dir(&self) -> Result<()> {
        fs::create_dir_all(self.get_data_dir()).map_err(|e| {
            CommandError::new(
                format!("Failed to create data directory: {}", e),
                exitcode::CANTCREAT as u8,
            )
        })
    }

    pub fn get_instance_dir(&self) -> PathBuf {
        let mut instance_path = self.get_data_dir();
        instance_path.push("instances");
        instance_path.push(&self.instance);
        instance_path
    }

    pub fn instance_dir_exists(&self) -> bool {
        self.get_instance_dir().exists()
    }

    pub fn create_instance_dir(&self) -> Result<()> {
        fs::create_dir_all(self.get_instance_dir()).map_err(|e| {
            CommandError::new(
                format!("Failed to create instance directory: {}", e),
                exitcode::CANTCREAT as u8,
            )
        })
    }
}
