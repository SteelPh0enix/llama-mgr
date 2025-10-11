use clap::Parser;

use crate::{
    commands::{CommonArguments, Result},
    config::{Config, Profile},
};

#[derive(Debug, Parser)]
pub struct UninstallCommand {
    #[command(flatten)]
    common: CommonArguments,

    #[arg(long, short)]
    /// Force uninstall without confirmation
    pub force: bool,
}

pub fn run(args: UninstallCommand, config: &Config, profile: &Profile) -> Result<()> {
    log::info!(
        "Uninstall command called with args: {:?}, config: {:?}, profile: {:?}",
        args,
        config,
        profile
    );
    Ok(())
}
