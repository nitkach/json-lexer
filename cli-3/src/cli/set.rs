use crate::database::Breed;

use super::CommandContext;

#[derive(Debug, clap::Args)]
pub(crate) struct SetCommand {
    #[clap(long)]
    id: i64,

    #[clap(long)]
    name: Option<String>,

    #[clap(long)]
    breed: Option<Breed>
}

impl SetCommand {
    pub(crate) async fn run(self, context: &CommandContext) -> Result<(), crate::error::Error> {
        let record = context.database.set(self.id, self.name, self.breed).await?;

        if let Some(record) = record {
            println!("{}", record)
        } else {
            println!("No record found by {} id.", self.id);
        }

        Ok(())
    }
}
