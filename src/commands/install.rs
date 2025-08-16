use std::{path::Path, process::ExitCode};

use clap::{Parser, ValueEnum};
use git2::{Repository, build::RepoBuilder};

use crate::{
    commands::{CommandError, CommonArguments, Result},
    external_tools::{ExternalTool, cmake::CMake, ninja::Ninja, uv::Uv, version::Version},
};

const RECOMMENDED_PYTHON_VERSION: Version = Version {
    major: 3,
    minor: Some(13),
    patch: None,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InstallationArchitecture {
    Cpu,
    Rocm,
}

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

    #[arg(long, short, default_value = "https://github.com/ggml-org/llama.cpp")]
    /// Set llama.cpp repository URL.
    pub repo_url: String,

    #[arg(long, short, default_value = "master")]
    /// Set llama.cpp repository branch to build from
    pub branch: String,

    #[arg(long, short)]
    /// Add custom arguments to cmake for building llama.cpp
    pub cmake_args: Option<String>,

    #[arg(long, short, value_enum, default_value_t = InstallationArchitecture::Cpu)]
    /// Specify the architecture to build for
    pub arch: InstallationArchitecture,

    #[arg(long, short = 'j')]
    /// Specify the amount of threads to use for building.
    pub parallel: Option<usize>,
}

pub fn run(args: InstallCommand) -> Result<()> {
    if args.update_only && !args.common.instance_dir_exists() {
        return Err(CommandError::new(
            format!(
                "Instance {} is not installed, nothing to update.",
                args.common.instance
            ),
            exitcode::UNAVAILABLE as u8,
        ));
    }

    args.common.create_data_dir()?;
    args.common.create_instance_dir()?;
    let instance_path = args.common.get_instance_dir();

    pull_or_update_source_code(&args, &instance_path)?;

    if args.pull_only {
        return Ok(());
    }

    verify_prerequisites(&args)?;

    generate_cmake_build_files(&args, &instance_path)?;
    build_and_install_llama_cpp(&args, &instance_path)?;

    if !args.ignore_python {
        setup_python_environment(&args, &instance_path)?;
    }

    Ok(())
}

fn verify_prerequisites(args: &InstallCommand) -> Result<(CMake, Ninja, Option<Uv>)> {
    log::info!("Verifying prerequisites...");

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

    log::info!("All core prerequisites are installed.");
    Ok((cmake, ninja, uv))
}

fn pull_or_update_source_code(
    args: &InstallCommand,
    instance_path: impl AsRef<Path>,
) -> Result<Repository> {
    let source_path = instance_path.as_ref().join("source");

    let repo = if source_path.exists() {
        log::info!(
            "Updating llama.cpp source code in {}...",
            source_path.display()
        );

        let repo = Repository::open(&source_path).map_err(|e| {
            CommandError::new(
                format!(
                    "Failed to open existing repository at {} - {}\n",
                    source_path.display(),
                    e
                ),
                exitcode::IOERR as u8,
            )
        })?;

        {
            let mut remote = repo.find_remote("origin").map_err(|e| {
                CommandError::new(
                    format!("Failed to find remote 'origin' in repository - {}", e),
                    exitcode::IOERR as u8,
                )
            })?;

            remote.fetch(&[&args.branch], None, None).map_err(|e| {
                CommandError::new(
                    format!("Failed to fetch from remote - {}", e),
                    exitcode::IOERR as u8,
                )
            })?;

            let (object, reference) = repo.revparse_ext(&args.branch).map_err(|e| {
                CommandError::new(
                    format!("Failed to revparse branch {} - {}", args.branch, e),
                    exitcode::DATAERR as u8,
                )
            })?;

            repo.checkout_tree(&object, None).map_err(|e| {
                CommandError::new(
                    format!("Failed to checkout tree - {}", e),
                    exitcode::IOERR as u8,
                )
            })?;

            match reference {
                // gref is an actual reference like branches or tags
                Some(gref) => repo.set_head(gref.name().unwrap()).map_err(|e| {
                    CommandError::new(
                        format!("Failed to set head to {} - {}", gref.name().unwrap(), e),
                        exitcode::IOERR as u8,
                    )
                })?,
                // this is a commit, not a reference
                None => repo.set_head_detached(object.id()).map_err(|e| {
                    CommandError::new(
                        format!(
                            "Failed to set head to detached commit {} - {}",
                            object.id(),
                            e
                        ),
                        exitcode::IOERR as u8,
                    )
                })?,
            };
        }

        repo
    } else {
        log::info!(
            "Cloning llama.cpp repository to {}...",
            source_path.display()
        );

        RepoBuilder::new()
            .branch(&args.branch)
            .clone(&args.repo_url, &source_path)
            .map_err(|e| {
                CommandError::new(
                    format!("Failed to clone repository {} - {}", args.repo_url, e),
                    exitcode::CANTCREAT as u8,
                )
            })?
    };

    log::info!("Updating submodules...");

    {
        let mut submodules = repo.submodules().map_err(|e| {
            CommandError::new(
                format!("Failed to get submodules - {}", e),
                exitcode::IOERR as u8,
            )
        })?;

        for submodule in submodules.iter_mut() {
            submodule.update(true, None).map_err(|e| {
                CommandError::new(
                    format!(
                        "Failed to update submodule {} - {}",
                        submodule.path().display(),
                        e
                    ),
                    exitcode::IOERR as u8,
                )
            })?;
        }
    }

    log::info!("Source code is ready.");
    Ok(repo)
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
