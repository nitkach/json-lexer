use super::CommandContext;

#[derive(Debug, clap::Args)]
pub(crate) struct RemoveCommand {
    #[clap(long)]
    id: i64,
}

impl RemoveCommand {
    pub(crate) async fn run(self, context: &mut CommandContext) -> Result<(), crate::error::Error> {
        let record = context.database.remove(self.id).await?;

        if let Some(record) = record {
            println!("Removed record: {record}");
        } else {
            println!("No record found by {} id.", self.id);
        }

        Ok(())
    }
}
