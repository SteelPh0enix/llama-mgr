use clap::Parser;

use crate::commands::{CommonArguments, Result};

#[derive(Debug, Parser)]
pub struct UninstallCommand {
    #[command(flatten)]
    common: CommonArguments,

    #[arg(long, short)]
    /// Force uninstall without confirmation
    pub force: bool,
}

pub fn run(args: UninstallCommand) -> Result<()> {
    log::info!("Uninstall command called with args: {:?}", args);
    Ok(())
}
