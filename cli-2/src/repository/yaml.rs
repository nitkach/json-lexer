use super::{Formatter, RecordData};
use crate::error::{Error, ErrorKind};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct YamlFormatter;

impl Formatter for YamlFormatter {
    fn from_str(&self, string: &str) -> Result<BTreeMap<String, RecordData>, Error> {
        serde_yaml::from_str(string).map_err(|err| {
            Error::new(ErrorKind::Deserialization {
                string: string.to_owned(),
                format: "YAML",
                source: Box::new(err),
            })
        })

        // let err = match serde_yaml::from_str(string) {
        //     Ok(records) => return Ok(records),
        //     Err(err) => err,
        // };

        // Err(Error::new(ErrorKind::Deserialization {
        //     string: string.to_owned(),
        //     format: "YAML",
        //     source: Box::new(err),
        // }))
    }

    fn to_string(&self, records: &BTreeMap<String, RecordData>) -> String {
        serde_yaml::to_string(records).unwrap()
    }
}
