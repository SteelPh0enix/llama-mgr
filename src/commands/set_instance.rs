use clap::Parser;

#[derive(Parser)]
pub struct SetInstanceCommand {
    #[arg(help = "Name of the instance to set as active")]
    pub instance_name: String,
}

pub fn run(args: &[&str]) {
    let args: SetInstanceCommand = SetInstanceCommand::parse_from(args);
    println!("Setting instance to: {}", args.instance_name);
}