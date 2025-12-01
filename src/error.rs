use std::fmt::Display;
use std::process::ExitCode;
use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct RuntimeError {
    pub message: String,
    pub exit_code: ExitCode,
}

impl RuntimeError {
    pub fn new(message: String, exit_code: u8) -> Self {
        Self {
            message,
            exit_code: ExitCode::from(exit_code),
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for RuntimeError {
    fn from(err: io::Error) -> Self {
        RuntimeError::new(err.to_string(), exitcode::IOERR.try_into().unwrap())
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;