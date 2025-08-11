use clap::{Parser, ValueEnum};

use crate::commands::CommonArguments;

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

pub fn run(args: InstallCommand) {
    
}
