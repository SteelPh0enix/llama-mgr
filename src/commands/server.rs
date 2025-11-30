use clap::Parser;

use crate::{
    commands::Result,
    config::{Config, Profile},
};

#[derive(Debug, Parser)]
pub struct ServerCommand {
    #[arg(long, short)]
    /// Model to use
    pub model: Option<String>,

    #[arg(long, default_value_t = 8080)]
    /// Port to listen on
    pub port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    /// Address to bind to
    pub address: String,

    #[arg(long)]
    /// Context size
    pub ctx_size: Option<u32>,

    #[arg(long, short)]
    /// Number of layers to offload to GPU
    pub gpu_layers: Option<u32>,
}

pub fn run(args: ServerCommand, config: &Config, profile: &Profile) -> Result<()> {
    log::info!(
        "Server command called with args: {:?}, config: {:?}, profile: {:?}",
        args,
        config,
        profile
    );
    Ok(())
}
