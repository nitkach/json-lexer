mod cli;
pub mod database;
mod error;

use clap::Parser;
use error::CliError;

pub async fn run() -> Result<(), CliError> {
    let args = cli::Args::parse();

    args.run().await?;

    Ok(())
}
