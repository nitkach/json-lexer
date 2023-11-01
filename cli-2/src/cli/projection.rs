use crate::repository::{Projection, Record};

// remove traits
pub(crate) fn projecting(records: Vec<Record>, to_project: Option<Projection>) -> Vec<String> {
    let projections: Vec<String> = records
        .into_iter()
        .map(|record| match to_project {
            Some(Projection::Id) => record.id,
            Some(Projection::Name) => record.name,
            Some(Projection::Breed) => record.breed.to_string(),
            None => record.to_string(),
        })
        .collect();

    projections
}
