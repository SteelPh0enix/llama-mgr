use clap::Parser;

use crate::commands::{CommonArguments, Result};

#[derive(Debug, Parser)]
pub struct QuantizeCommand {
    #[command(flatten)]
    common: CommonArguments,

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

pub fn run(args: QuantizeCommand) -> Result<()> {
    println!("Quantize command called with args: {:?}", args);
    Ok(())
}
