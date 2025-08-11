use clap::Parser;

use crate::commands::CommonArguments;

#[derive(Debug, Parser)]
pub struct UninstallCommand {
    #[command(flatten)]
    common: CommonArguments,

    #[arg(long, short)]
    /// Force uninstall without confirmation
    pub force: bool,
}

pub fn run(args: UninstallCommand) {
    println!("Uninstall command called with args: {:?}", args);
}
