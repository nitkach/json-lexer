use log::info;

use super::{CommandContext, RunCommand};
use crate::error::Error;
use crate::repository::{Breed, RecordData};

#[derive(Debug, clap::Args)]
pub(crate) struct AddCommand {
    #[clap(long)]
    name: String,

    #[clap(long, value_enum)]
    breed: Breed,
}

impl RunCommand for AddCommand {
    fn run(self: Box<Self>, mut context: CommandContext) -> Result<(), Error> {
        let id = context.repo.add(RecordData {
            name: self.name,
            breed: self.breed,
        })?;

        context.repo.commit()?;

        info!("Added new record with id {id}");

        Ok(())
    }
}
