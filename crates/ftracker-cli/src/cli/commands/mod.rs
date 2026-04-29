use clap::Subcommand;

pub mod add;
pub mod parse;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Record a new trade (buy or sell)
    Add(add::AddArgs),
}
