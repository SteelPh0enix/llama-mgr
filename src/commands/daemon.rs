use clap::Parser;

use crate::{
    commands::Result,
    config::{Config, Profile},
};

#[derive(Debug, Parser)]
pub struct DaemonCommand {
    #[arg(long, short, default_value_t = 51536)]
    /// Port to listen on
    pub port: u16,

    #[arg(long, short, default_value = "127.0.0.1")]
    /// Address to bind to
    pub address: String,
}

pub fn run(args: DaemonCommand, config: &Config, profile: &Profile) -> Result<()> {
    log::info!(
        "Daemon command called with args: {:?}, config: {:?}, profile: {:?}",
        args,
        config,
        profile
    );
    Ok(())
}
