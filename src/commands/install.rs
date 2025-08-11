use clap::Parser;

#[derive(Parser)]
pub struct InstallCommand {
    #[arg(long, short = 'p', help = "Do not trigger installation, only download the repo.")]
    pub pull_only: bool,
    #[arg(long, short = 'u', help = "Prevents installing llama.cpp if it's not installed already, only updates existing installation.")]
    pub update_only: bool,
    #[arg(long, help = "Skips the Python environment configuration.")]
    pub ignore_python: bool,
    #[arg(long, help = "Use custom repository, instead of official one.")]
    pub repo_url: Option<String>,
    #[arg(long, help = "Use custom branch, instead of master.")]
    pub branch: Option<String>,
    #[arg(long, help = "Add custom arguments to cmake for building llama.cpp.")]
    pub cmake_args: Option<String>,
    #[arg(long, help = "Specify the architecture to build for.")]
    pub arch: Option<String>,
    #[arg(long, help = "Specify the amount of threads to use for building.")]
    pub parallel: Option<usize>,
}

pub fn run() {
    let _args: InstallCommand = InstallCommand::parse();
    // Implementation here using args
}
