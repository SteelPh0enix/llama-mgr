use clap::Parser;

use crate::commands::CommonArguments;

#[derive(Debug, Parser)]
pub struct ServerCommand {
    #[command(flatten)]
    common: CommonArguments,

    #[arg(long, short)]
    /// Model to use
    pub model: Option<String>,

    #[arg(long, short)]
    /// Port to listen on
    pub port: u16,

    #[arg(long, short)]
    /// Address to bind to
    pub address: String,

    #[arg(long, short)]
    /// Context size
    pub ctx_size: u32,

    #[arg(long, short)]
    /// Number of layers to offload to GPU
    pub gpu_layers: Option<u32>,
}

pub fn run(args: ServerCommand) {
    println!("Server command called with args: {:?}", args);
}
