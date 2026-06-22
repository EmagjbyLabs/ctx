mod cli;
mod command;
mod filter;
mod render;
mod repo;
mod walk;

use anyhow::Result;
use clap::Parser;

use crate::{cli::Cli, command::run};

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}
