use clap::Parser;

use crate::commands::{CommonArguments, Result};

#[derive(Debug, Parser)]
pub struct ConvertCommand {
    #[command(flatten)]
    common: CommonArguments,

    #[arg(long, short)]
    /// Path to the directory with HuggingFace model
    pub input: String,

    #[arg(long, short)]
    /// Path to the output GGUF file
    pub output: String,
}

pub fn run(args: ConvertCommand) -> Result<()> {
    log::info!("Convert command called with args: {:?}", args);
    Ok(())
}
