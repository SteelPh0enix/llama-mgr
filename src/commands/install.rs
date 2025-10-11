use std::{path::Path, process::ExitCode};

use clap::Parser;

use crate::{
    commands::{CommandError, CommonArguments, Result},
    config::{Config, Profile},
    external_tools::{
        ExternalTool, cmake::CMake, git::Git, ninja::Ninja, uv::Uv, version::Version,
    },
};

const RECOMMENDED_PYTHON_VERSION: Version = Version {
    major: 3,
    minor: Some(13),
    patch: None,
};

#[derive(Debug, Parser)]
pub struct InstallCommand {
    #[command(flatten)]
    common: CommonArguments,

    #[arg(long)]
    /// Do not trigger installation, only download the repo
    pub pull_only: bool,

    #[arg(long)]
    /// Do not install llama.cpp, only update existing profiles
    pub update_only: bool,

    #[arg(long)]
    /// Skip Python installation and setup
    pub ignore_python: bool,

    #[arg(long, short = 'j')]
    /// Specify the amount of threads to use for building.
    pub parallel: Option<usize>,
}

pub fn run(args: InstallCommand, config: &Config, profile: &Profile) -> Result<()> {
    todo!()
}

fn get_git() -> Result<Git> {
    log::info!("Verifying git presence...");

    match Git::global() {
        Err(_) => Err(CommandError::new(
            "Git is not installed. Please install it using your system's package manager."
                .to_string(),
            exitcode::UNAVAILABLE as u8,
        )),
        Ok(prog) => {
            log::info!("Git is installed.");
            Ok(prog)
        }
    }
}

fn get_prerequisites(args: &InstallCommand) -> Result<(CMake, Ninja, Option<Uv>)> {
    log::info!("Verifying prerequisites presence...");

    let cmake =
        match CMake::global() {
            Err(_) => return Err(CommandError::new(
                "CMake is not installed. Please install it using your system's package manager."
                    .to_string(),
                exitcode::UNAVAILABLE as u8,
            )),
            Ok(prog) => prog,
        };

    let ninja =
        match Ninja::global() {
            Err(_) => return Err(CommandError::new(
                "Ninja is not installed. Please install it using your system's package manager."
                    .to_string(),
                exitcode::UNAVAILABLE as u8,
            )),
            Ok(prog) => prog,
        };

    let mut uv: Option<Uv> = None;
    if !args.ignore_python {
        uv = match Uv::global() {
            Err(_) => {
                return Err(CommandError::new(
                    "uv is not installed. Please install it using `pip install uv`.".to_string(),
                    exitcode::UNAVAILABLE as u8,
                ));
            }
            Ok(prog) => Some(prog),
        };

        uv.as_ref()
            .unwrap()
            .install_python_version(RECOMMENDED_PYTHON_VERSION)
            .map_err(|e| CommandError {
                message: format!("Could not install Python - {}", e),
                exit_code: ExitCode::from(exitcode::SOFTWARE as u8),
            })?;
    }

    log::info!("All build prerequisites are installed.");
    Ok((cmake, ninja, uv))
}

fn pull_or_update_source_code(
    git: &Git,
    args: &InstallCommand,
    instance_path: impl AsRef<Path>,
) -> Result<()> {
    todo!()
}

fn generate_cmake_build_files(
    args: &InstallCommand,
    instance_path: impl AsRef<Path>,
) -> Result<()> {
    todo!()
}

fn build_and_install_llama_cpp(
    args: &InstallCommand,
    instance_path: impl AsRef<Path>,
) -> Result<()> {
    todo!()
}

fn setup_python_environment(args: &InstallCommand, instance_path: impl AsRef<Path>) -> Result<()> {
    todo!()
}
