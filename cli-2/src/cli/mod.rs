mod add;
mod get;
mod remove;
mod list;

use crate::error::{CliError, Error};
use crate::repository::Repository;
use camino::Utf8PathBuf;

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
    #[clap(subcommand)]
    kind: ArgsKind,

    // --output-format text --output-format json / --output-format yaml/toml
    // output_format: OutputFormat,
    #[clap(
        long,
        global = true,
        default_value_t = Utf8PathBuf::from("./db.json")
    )]
    path: Utf8PathBuf,

    /// Enable verbose output (errors will include backtraces)
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, clap::Subcommand)]
enum ArgsKind {
    Add(add::AddCommand),
    Get(get::GetCommand),
    Remove(remove::RemoveCommand),
    List(list::ListCommand),
}

impl Args {
    pub(crate) fn run(self) -> Result<(), CliError> {
        let verbose = self.verbose;
        match self.run_imp() {
            Ok(()) => Ok(()),
            Err(err) => Err(CliError {
                inner: err,
                verbose,
            }),
        }
    }

    fn run_imp(self) -> Result<(), Error> {
        let context = CommandContext {
            repo: Repository::new(self.path)?,
        };

        let command: Box<dyn RunCommand> = match self.kind {
            ArgsKind::Add(command) => Box::new(command),
            ArgsKind::Get(command) => Box::new(command),
            ArgsKind::Remove(command) => Box::new(command),
            ArgsKind::List(command) => Box::new(command),
        };

        command.run(context)?;

        Ok(())
    }
}

#[derive(Debug)]
struct CommandContext {
    repo: Repository,
}

trait RunCommand {
    fn run(self: Box<Self>, context: CommandContext) -> Result<(), Error>;
}
