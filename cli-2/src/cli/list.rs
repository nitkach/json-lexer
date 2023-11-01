use log::info;

use super::projection;
use super::{CommandContext, RunCommand};
use crate::{
    error::Error,
    repository::{self, Breed},
};

#[derive(Debug, clap::Args)]
pub(crate) struct ListCommand {
    #[clap(long, value_enum)]
    breed: Option<Breed>,

    #[clap(long, value_enum)]
    sort: Option<Sort>,

    #[clap(long, value_enum)]
    show: Option<Projection>,
}

#[derive(Debug, clap::ValueEnum, Clone)]
enum Sort {
    Id,
    Name,
    Breed,
}

#[derive(Debug, clap::ValueEnum, Clone)]
enum Projection {
    Id,
    Name,
    Breed,
}

impl RunCommand for ListCommand {
    fn run(self: Box<Self>, context: CommandContext) -> Result<(), Error> {
        let sort = self.sort.map(repository::Sort::from);

        let records = context.repo.list(self.breed, sort);
        let project = self.show.map(repository::Projection::from);

        let projected = projection::projecting(records, project);

        for record in projected {
            info!("{}", record);
            println!("{}", record)
        }

        Ok(())
    }
}

impl From<Projection> for repository::Projection {
    fn from(value: Projection) -> Self {
        match value {
            Projection::Id => Self::Id,
            Projection::Name => Self::Name,
            Projection::Breed => Self::Breed,
        }
    }
}

impl From<Sort> for repository::Sort {
    fn from(value: Sort) -> Self {
        match value {
            Sort::Id => Self::Id,
            Sort::Name => Self::Name,
            Sort::Breed => Self::Breed,
        }
    }
}
