use super::CommandContext;

#[derive(Debug, clap::Args)]
pub(crate) struct GetCommand {
    #[clap(long)]
    id: i64,
}

impl GetCommand {
    pub(crate) async fn run(self, context: &CommandContext) -> Result<(), crate::error::Error> {
        let record = context.database.get(self.id).await?;

        if let Some(record) = record {
            println!("{}", record)
        } else {
            println!("No record found by {} id.", self.id);
        }

        Ok(())
    }
}
