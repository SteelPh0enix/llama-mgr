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
    /// Skip Python installation and setup
    pub ignore_python: bool,

    #[arg(long, short = 'j')]
    /// Specify the amount of threads to use for building.
    pub parallel: Option<usize>,
}

pub fn run(_args: InstallCommand, _config: &Config, _profile: &Profile) -> Result<()> {
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

fn get_cmake() -> Result<CMake> {
    log::info!("Verifying CMake presence...");

    match CMake::global() {
        Err(_) => Err(CommandError::new(
            "CMake is not installed. Please install it using your system's package manager."
                .to_string(),
            exitcode::UNAVAILABLE as u8,
        )),
        Ok(prog) => {
            log::info!("CMake is installed.");
            Ok(prog)
        }
    }
}

fn get_ninja() -> Result<Ninja> {
    log::info!("Verifying Ninja presence...");

    match Ninja::global() {
        Err(_) => Err(CommandError::new(
            "Ninja is not installed. Please install it using your system's package manager."
                .to_string(),
            exitcode::UNAVAILABLE as u8,
        )),
        Ok(prog) => {
            log::info!("Ninja is installed.");
            Ok(prog)
        }
    }
}

fn get_uv() -> Result<Uv> {
    log::info!("Verifying uv presence...");

    let uv = match Uv::global() {
        Err(_) => {
            return Err(CommandError::new(
                "uv is not installed. Please install it using `pip install uv`.".to_string(),
                exitcode::UNAVAILABLE as u8,
            ));
        }
        Ok(prog) => {
            log::info!("uv is installed.");
            prog
        }
    };

    Ok(uv)
}

fn install_python_with_uv(uv: &Uv) -> Result<()> {
    uv.install_python_version(RECOMMENDED_PYTHON_VERSION)
        .map_err(|e| CommandError {
            message: format!("Could not install Python - {}", e),
            exit_code: ExitCode::from(exitcode::SOFTWARE as u8),
        })
}

fn get_prerequisites(args: &InstallCommand) -> Result<(CMake, Ninja, Option<Uv>)> {
    log::info!("Verifying prerequisites presence...");

    let cmake = get_cmake()?;
    let ninja = get_ninja()?;

    let uv = if args.ignore_python {
        log::info!("Skipping Python setup as requested.");
        None
    } else {
        let uv = get_uv()?;
        install_python_with_uv(&uv)?;
        Some(uv)
    };

    log::info!("All build prerequisites are installed.");
    Ok((cmake, ninja, uv))
}

fn pull_or_update_source_code(
    git: &Git,
    repo_url: &str,
    branch: Option<&str>,
    target_path: impl AsRef<Path>,
) -> Result<()> {
    if target_path.as_ref().exists() {
        log::info!(
            "Repository already exists at {}. Updating...",
            target_path.as_ref().display()
        );
        git.pull(&target_path).map_err(|e| CommandError {
            message: format!("Failed to pull repository: {}", e),
            exit_code: ExitCode::from(exitcode::IOERR as u8),
        })?;
    } else {
        log::info!(
            "Cloning repository from {} to {}",
            repo_url,
            target_path.as_ref().display()
        );
        git.clone(
            &target_path,
            repo_url,
            branch.map(|b| b.as_ref() as &std::ffi::OsStr),
        )
        .map_err(|e| CommandError {
            message: format!("Failed to clone repository: {}", e),
            exit_code: ExitCode::from(exitcode::CANTCREAT as u8),
        })?;
    }

    // Initialize and update submodules
    log::info!("Updating submodules...");
    git.update_submodules(&target_path)
        .map_err(|e| CommandError {
            message: format!("Failed to update submodules: {}", e),
            exit_code: ExitCode::from(exitcode::IOERR as u8),
        })?;

    Ok(())
}

fn generate_cmake_build_files(
    _args: &InstallCommand,
    _instance_path: impl AsRef<Path>,
) -> Result<()> {
    todo!()
}

fn build_and_install_llama_cpp(
    _args: &InstallCommand,
    _instance_path: impl AsRef<Path>,
) -> Result<()> {
    todo!()
}

fn setup_python_environment(
    _args: &InstallCommand,
    _instance_path: impl AsRef<Path>,
) -> Result<()> {
    todo!()
}
