use super::{RunCommand, CommandContext};
use crate::error::Error;

#[derive(Debug, clap::Args)]
pub(crate) struct ListCommand;

impl RunCommand for ListCommand {
    fn run(self: Box<Self>, context: CommandContext) -> Result<(), Error> {
        println!("List: {:?}", self);
        Ok(())
    }
}
