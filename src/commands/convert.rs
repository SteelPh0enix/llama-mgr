use clap::Parser;

use crate::{
    commands::Result,
    config::{Config, Profile},
};

#[derive(Debug, Parser)]
pub struct ConvertCommand {
    #[arg(long, short)]
    /// Path to the directory with HuggingFace model
    pub input: String,

    #[arg(long, short)]
    /// Path to the output GGUF file
    pub output: String,
}

pub fn run(args: ConvertCommand, config: &Config, profile: &Profile) -> Result<()> {
    log::info!(
        "Convert command called with args: {:?}, config: {:?}, profile: {:?}",
        args,
        config,
        profile
    );
    Ok(())
}
