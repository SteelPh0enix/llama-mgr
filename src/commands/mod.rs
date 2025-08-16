pub mod convert;
pub mod daemon;
pub mod install;
pub mod quantize;
pub mod server;
pub mod uninstall;

use std::fmt::Display;
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
struct CommonArguments {
    #[arg(long, default_value = "~/.llama-mgr")]
    /// Data directory for installation files and configuration.
    pub data_dir: String,

    #[arg(long, default_value = "global")]
    /// Name of installation profile to use.
    pub instance: String,
}
