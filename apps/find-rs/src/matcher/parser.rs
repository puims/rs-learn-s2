use clap::Parser;

use crate::types::Cli;

pub fn parse_cli() -> Cli {
    Cli::parse()
}