use log::info;

use super::{CommandContext, RunCommand};
use crate::error::Error;

#[derive(Debug, clap::Args)]
pub(crate) struct RemoveCommand {
    #[clap(long)]
    id: String,
}

impl RunCommand for RemoveCommand {
    fn run(self: Box<Self>, mut context: CommandContext) -> Result<(), Error> {
        let name = context.repo.remove(&self.id)?;

        context.repo.commit()?;

        info!("Removed record with name {name}");

        Ok(())
    }
}
