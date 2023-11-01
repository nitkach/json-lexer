mod json;
mod yaml;

use crate::error::Error;
use camino::Utf8PathBuf;
use clap::ValueEnum;
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

const ID_LEN: usize = 32;

pub(crate) enum Projection {
    Id,
    Name,
    Breed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum, PartialEq, Ord, PartialOrd, Eq)]
pub(crate) enum Breed {
    Pegasus,
    Earth,
    Unicorn,
}

impl Display for Breed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let breed = match self {
            Breed::Pegasus => "Pegasus",
            Breed::Earth => " Earth ",
            Breed::Unicorn => "Unicorn",
        };

        write!(f, "{breed}")
    }
}

#[derive(Debug)]
pub(crate) struct Repository {
    records: BTreeMap<String, RecordData>,
    path: Utf8PathBuf,
    formatter: Box<dyn Formatter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecordData {
    pub(crate) name: String,
    pub(crate) breed: Breed,
}

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) breed: Breed,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.id, self.breed, self.name)
    }
}

pub(crate) enum Sort {
    Id,
    Name,
    Breed,
}

impl Repository {
    pub(crate) fn new(path: Utf8PathBuf) -> Result<Self, Error> {
        let extension = path.extension().ok_or_else(|| {
            Error::fatal(format!(
                "No file extension was specified for the path to the database: '{path}'\n",
            ))
        })?;

        // let Some(extension) = path.extension() else {
        //     return Err(Error::fatal(format!(
        //         "No file extension was specified for the path to the database: '{path}'\n",
        //     )));
        // };

        let formatter: Box<dyn Formatter> = match extension {
            "json" => Box::new(json::JsonFormatter),
            "yaml" => Box::new(yaml::YamlFormatter),
            _ => {
                return Err(Error::fatal(format!(
                "Unsupported file extension `{extension}` for the path to the database: `{path}`",
            )))
            }
        };

        let data = match std::fs::read_to_string(&path) {
            Ok(string) => {
                info!(
                    "Successfully opened a file with data along the path: `{}`",
                    &path
                );
                formatter.from_str(&string)?
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                info!("Data file not found at specified path: `{}`", &path);
                match path.parent() {
                    Some(parent) if !parent.exists() => {
                        // error!("File path does not exist: `{}`", parent);
                        return Err(Error::fatal(format!(
                            "This path does not exist: `{path}`. Error: {err}",
                        )));
                    }
                    _ => {
                        info!("Created new data file at path: `{}`", &path);
                        BTreeMap::new()
                    }
                }
            }
            Err(err) => {
                return Err(Error::fatal(format!(
                    "Failed to read the database file: `{path}`. Error: {err}",
                )))
            }
        };

        Ok(Self {
            records: data,
            path,
            formatter,
        })
    }

    pub(crate) fn add(&mut self, record: RecordData) -> Result<String, Error> {
        use rand::distributions::{Alphanumeric, DistString};

        let mut rng = rand::thread_rng();
        let id = Alphanumeric.sample_string(&mut rng, ID_LEN);

        self.records.insert(id.clone(), record);

        Ok(id)
    }

    // never return Error
    // "always deleted record"
    pub(crate) fn remove(&mut self, id: &str) -> Result<String, Error> {
        // let Some(record) = self.records.remove(id) else {
        //     return Err(Error::fatal(format!(
        //         "Record by id: `{id}` does not exist",
        //     )));
        // };

        let record = self
            .records
            .remove(id)
            .ok_or_else(|| Error::fatal(format!("Record by id: `{id}` does not exist",)))?;
        Ok(record.name)
    }

    pub(crate) fn commit(&self) -> Result<(), Error> {
        let string = self.formatter.to_string(&self.records);

        std::fs::write(&self.path, string).map_err(|err| {
            Error::fatal(format!(
                "Failed to write the database file: `{}`. Error: {err}",
                self.path,
            ))
        })?;

        info!("Successfully saved data to file: `{}`", &self.path);
        Ok(())

        // let Err(err) = std::fs::write(&self.path, string) else {
        //     info!("Successfully saved data to file: `{}`", &self.path);
        //     return Ok(())
        // };

        // Err(Error::fatal(format!(
        //     "Failed to write the database file: `{}`. Error: {err}",
        //     self.path,
        // )))
    }

    pub(crate) fn get(&self, id: &str) -> Option<RecordData> {
        self.records.get(id).cloned()
    }

    // how to sort / filter
    pub(crate) fn list(&self, breed_filter: Option<Breed>, sort: Option<Sort>) -> Vec<Record> {
        // let mut result = Vec::<Record>::new();
        let mut result: Vec<Record> = self
            .records
            .clone()
            .into_iter()
            .filter(|x| {
                let Some(ref breed_filter) = breed_filter else {
                    return true;
                };
                &x.1.breed == breed_filter
            })
            .map(|(id, RecordData { name, breed })| Record { id, name, breed })
            .collect();

        // breed_filter
        //     .map(|breed_to_project| {
        //         for (id, RecordData { name, breed }) in self.records.clone() {
        //             if breed == breed_to_project {
        //                 result.push(Record { id, name, breed })
        //             }
        //         }
        //     })
        //     .unwrap_or_else(|| {
        //         for (id, RecordData { name, breed }) in self.records.clone() {
        //             result.push(Record { id, name, breed })
        //         }
        //     });
        // -------------
        // match breed {
        //     Some(breed_filter) => {
        //         for (id, RecordData { name, breed }) in self.records.clone() {
        //             if breed == breed_filter {
        //                 result.push(Record { id, name, breed })
        //             }
        //         }
        //     }
        //     None => {
        //         for (id, RecordData { name, breed }) in self.records.clone() {
        //             result.push(Record { id, name, breed })
        //         }
        //     }
        // };

        let Some(sort) = sort else {
            return result
        };

        match sort {
            Sort::Id => result.sort_by(|a, b| a.id.cmp(&b.id)),
            Sort::Name => result.sort_by(|a, b| a.name.cmp(&b.name)),
            Sort::Breed => result.sort_by(|a, b| a.breed.cmp(&b.breed)),
        }

        result
    }
}

trait Formatter: std::fmt::Debug {
    fn from_str(&self, string: &str) -> Result<BTreeMap<String, RecordData>, Error>;
    fn to_string(&self, records: &BTreeMap<String, RecordData>) -> String;
}
