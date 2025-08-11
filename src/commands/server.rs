use clap::Parser;

#[derive(Parser)]
pub struct ServerCommand {
    #[arg(short = 'm', long = "model", help = "Model to use")]
    pub model: Option<String>,
    
    #[arg(short = 'p', long = "port", help = "Port to listen on", default_value = "8080")]
    pub port: u16,
    
    #[arg(short = 'b', long = "bind", help = "Address to bind to", default_value = "127.0.0.1")]
    pub bind: String,
    
    #[arg(long, help = "Context size", default_value = "2048")]
    pub ctx_size: u32,
    
    #[arg(long, help = "Number of layers to offload to GPU")]
    pub gpu_layers: Option<u32>,
}

pub fn run(args: &[&str]) {
    let args: ServerCommand = ServerCommand::parse_from(args);
    println!("Running server command with model: {:?}", args.model);
}