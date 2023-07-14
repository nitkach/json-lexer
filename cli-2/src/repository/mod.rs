mod json;
mod yaml;

use crate::error::Error;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const ID_LEN: usize = 32;

#[derive(Debug)]
pub(crate) struct Repository {
    records: BTreeMap<String, Record>,
    path: Utf8PathBuf,
    formatter: Box<dyn Formatter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Record {
    pub(crate) name: String,
    // TODO create enum for breed
    pub(crate) breed: String,
}

impl Repository {
    pub(crate) fn new(path: Utf8PathBuf) -> Result<Self, Error> {
        let Some(extension) = path.extension() else {
            return Err(Error::fatal(format!(
                "No file extension was specified for the path to the database: '{path}'\n",
            )));
        };

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
            Ok(string) => formatter.from_str(&string)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
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

    pub(crate) fn add(&mut self, record: Record) -> Result<String, Error> {
        use rand::distributions::{Alphanumeric, DistString};

        let mut rng = rand::thread_rng();
        let id = Alphanumeric.sample_string(&mut rng, ID_LEN);

        self.records.insert(id.clone(), record);

        Ok(id)
    }

    pub(crate) fn commit(&self) -> Result<(), Error> {
        let string = self.formatter.to_string(&self.records);

        let Err(err) = std::fs::write(&self.path, string) else {
            return Ok(())
        };

        Err(Error::fatal(format!(
            "Failed to write the database file: `{}`. Error: {err}",
            self.path,
        )))
    }

    pub(crate) fn get(&self, id: &str) -> Option<Record> {
        self.records.get(id).cloned()
    }
}

trait Formatter: std::fmt::Debug {
    fn from_str(&self, string: &str) -> Result<BTreeMap<String, Record>, Error>;
    fn to_string(&self, records: &BTreeMap<String, Record>) -> String;
}
