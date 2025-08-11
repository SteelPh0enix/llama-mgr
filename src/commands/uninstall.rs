use clap::Parser;

#[derive(Parser)]
pub struct UninstallCommand {
    #[arg(help = "Name of the instance to uninstall (default: current active instance)")]
    pub instance_name: Option<String>,
    
    #[arg(long, help = "Force uninstall without confirmation")]
    pub force: bool,
}

pub fn run(args: &[&str]) {
    let _args: UninstallCommand = UninstallCommand::parse_from(args);
    println!("Running uninstall command");
}