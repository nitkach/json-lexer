mod cli;
mod repository;
mod error;

use clap::Parser;
use error::CliError;

pub fn run() -> Result<(), CliError> {
    let args = cli::Args::parse();

    args.run()?;

    Ok(())
}
