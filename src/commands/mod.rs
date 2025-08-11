pub mod convert;
pub mod daemon;
pub mod install;
pub mod quantize;
pub mod server;
pub mod uninstall;

use clap::Args;

#[derive(Args, Debug)]
struct CommonArguments {
    #[arg(long, default_value = "~/.llama-mgr")]
    /// Data directory for installation files and configuration.
    pub data_dir: String,

    #[arg(long, default_value = "global")]
    /// Name of installation profile to use.
    pub instance: String,
}
