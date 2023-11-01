mod cli;
mod error;
mod repository;

use clap::Parser;
use error::CliError;

pub fn run() -> Result<(), CliError> {
    // TODO --output json/yaml/text
    let args = cli::Args::parse();

    args.run()?;

    Ok(())
}
