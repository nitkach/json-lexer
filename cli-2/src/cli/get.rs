use super::{CommandContext, RunCommand};
use crate::error::Error;

#[derive(Debug, clap::Args)]
pub(crate) struct GetCommand {
    id: String,
}

impl RunCommand for GetCommand {
    fn run(self: Box<Self>, context: CommandContext) -> Result<(), Error> {
        match context.repo.get(&self.id) {
            Some(record) => {
                println!(
                    "Found record name: {}, breed: {}",
                    record.name, record.breed
                );
            }
            None => {
                return Err(Error::fatal(format!("No record found with id {}", self.id)));
            }
        }

        Ok(())
    }
}
