use clap::Parser;

use crate::cli::args::GlobalArgs;
use crate::cli::commands::Commands;

pub mod args;
pub mod commands;

#[derive(Debug, Parser)]
#[command(
    name = "ft",
    version,
    about = "Track your financial trades from the command line",
    propagate_version = true
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}
