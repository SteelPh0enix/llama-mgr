use clap::Parser;

use crate::{
    commands::Result,
    config::{Config, Profile},
};

#[derive(Debug, Parser)]
pub struct QuantizeCommand {
    #[arg(long, short)]
    /// Path to the input GGUF file
    pub input: String,

    #[arg(long, short)]
    /// Path to the output GGUF file
    pub output: String,

    #[arg(long, short)]
    /// Quantization type.
    pub quant: String,
}

pub fn run(args: QuantizeCommand, config: &Config, profile: &Profile) -> Result<()> {
    log::info!(
        "Quantize command called with args: {:?}, config: {:?}, profile: {:?}",
        args,
        config,
        profile
    );
    Ok(())
}
