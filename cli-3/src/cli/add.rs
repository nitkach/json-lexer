use crate::database::{Breed, RecordData};

use super::CommandContext;

#[derive(Debug, clap::Args)]
pub(crate) struct AddCommand {
    #[clap(long)]
    name: String,

    #[clap(long, value_enum)]
    breed: Breed,
}

impl AddCommand {
    pub(crate) async fn run(self, context: &mut CommandContext) -> Result<(), crate::error::Error> {
        let id = context.database.add(RecordData { name: self.name, breed: self.breed }).await?;

        println!("Added record with {} id.", id);

        Ok(())
    }
}
