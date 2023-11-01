use log::info;

use super::{CommandContext, RunCommand};
use crate::error::Error;

#[derive(Debug, clap::Args)]
pub(crate) struct GetCommand {
    id: String,
}

impl RunCommand for GetCommand {
    fn run(self: Box<Self>, context: CommandContext) -> Result<(), Error> {
        let record = context
            .repo
            .get(&self.id)
            .ok_or_else(|| Error::fatal(format!("No record found with id {}", self.id)))?;

        info!(
            "Found record name: {}, breed: {}",
            record.name, record.breed
        );

        Ok(())
    }
}
