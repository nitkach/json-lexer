use super::CommandContext;

#[derive(Debug, clap::Args)]
pub(crate) struct ListCommand {}

impl ListCommand {
    pub(crate) async fn run(self, context: &CommandContext) -> Result<(), crate::error::Error> {
        let records = context.database.list().await?;

        for record in records {
            println!("{record}")
        }

        Ok(())
    }
}
