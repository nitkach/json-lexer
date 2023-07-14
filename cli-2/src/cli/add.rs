use super::{RunCommand, CommandContext};
use crate::error::Error;
use crate::repository::Record;

#[derive(Debug, clap::Args)]
pub(crate) struct AddCommand {
    #[clap(long)]
    name: String,

    #[arg(long, value_enum)]
    breed: String,
}

impl RunCommand for AddCommand {
    fn run(self: Box<Self>, mut context: CommandContext) -> Result<(), Error> {
        let id = context.repo.add(Record {
            name: self.name,
            breed: self.breed,
        })?;

        context.repo.commit()?;

        println!("Added new record with id {id}");

        Ok(())
    }
}
