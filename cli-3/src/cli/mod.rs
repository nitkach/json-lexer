use crate::{
    database::Database,
    error::{CliError, Error},
};

mod add;
mod get;
mod list;
mod remove;
mod set;

#[derive(Debug)]
pub(crate) struct CommandContext {
    database: Database,
}

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    kind: ArgsKind,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum ArgsKind {
    Add(add::AddCommand),
    Get(get::GetCommand),
    Remove(remove::RemoveCommand),
    List(list::ListCommand),
    Set(set::SetCommand),
}

impl Args {
    pub(crate) async fn run(self) -> Result<(), CliError> {
        self.run_imp().await.map_err(|err| CliError { inner: err })
    }

    async fn run_imp(self) -> Result<(), Error> {
        let mut context = CommandContext {
            database: Database::new().await?,
        };

        match self.kind {
            ArgsKind::Add(command) => command.run(&mut context).await,
            ArgsKind::Get(command) => command.run(&context).await,
            ArgsKind::Remove(command) => command.run(&mut context).await,
            ArgsKind::List(command) => command.run(&context).await,
            ArgsKind::Set(command) => command.run(&context).await,
        }
    }
}
