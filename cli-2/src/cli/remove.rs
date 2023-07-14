use super::{RunCommand, CommandContext};
use crate::error::Error;

#[derive(Debug, clap::Args)]
pub(crate) struct RemoveCommand {
    id: String,
}

impl RunCommand for RemoveCommand {
    fn run(self: Box<Self>, context: CommandContext) -> Result<(), Error> {
        println!("Remove: {:?}", self);
        Ok(())
    }
}
