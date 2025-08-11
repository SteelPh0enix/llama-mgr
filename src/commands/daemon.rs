use clap::Parser;

#[derive(Parser)]
pub struct DaemonCommand {
    #[arg(short = 'p', long = "port", help = "Port to listen on", default_value = "8080")]
    pub port: u16,
    
    #[arg(short = 'b', long = "bind", help = "Address to bind to", default_value = "127.0.0.1")]
    pub bind: String,
    
    #[arg(long, help = "Run in background")]
    pub background: bool,
}

pub fn run(args: &[&str]) {
    let args: DaemonCommand = DaemonCommand::parse_from(args);
    println!("Running daemon on {}:{}", args.bind, args.port);
}