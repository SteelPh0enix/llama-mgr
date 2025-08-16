use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::commands::{CommonArguments, Result};

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
    let instance_path = get_instance_path(&args)?;

    pull_or_update_source_code(&args, &instance_path)?;

    if args.pull_only {
        return Ok(());
    }

    verify_prerequisites()?;

    generate_cmake_build_files(&args, &instance_path)?;
    build_and_install_llama_cpp(&args, &instance_path)?;

    if !args.ignore_python {
        setup_python_environment(&args, &instance_path)?;
    }

    Ok(())
}

fn get_instance_path(args: &InstallCommand) -> Result<PathBuf> {
    todo!()
}

fn verify_prerequisites() -> Result<()> {
    todo!()
}

fn pull_or_update_source_code(args: &InstallCommand, instance_path: &PathBuf) -> Result<()> {
    todo!()
}

fn generate_cmake_build_files(args: &InstallCommand, instance_path: &PathBuf) -> Result<()> {
    todo!()
}

fn build_and_install_llama_cpp(args: &InstallCommand, instance_path: &PathBuf) -> Result<()> {
    todo!()
}

fn setup_python_environment(args: &InstallCommand, instance_path: &PathBuf) -> Result<()> {
    todo!()
}
