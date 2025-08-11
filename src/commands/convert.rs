use clap::Parser;

#[derive(Parser)]
pub struct ConvertCommand {
    #[arg(long, help = "Path to the directory with HuggingFace model")]
    pub input: String,
    #[arg(long, help = "Path to the output GGUF file")]
    pub output: String,
}

pub fn run() {
    let _args: ConvertCommand = ConvertCommand::parse();
    // Implementation here using args
}
