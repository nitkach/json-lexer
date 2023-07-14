use super::{Formatter, Record};
use crate::error::{Error, ErrorKind};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn from_str(&self, string: &str) -> Result<BTreeMap<String, Record>, Error> {
        let err = match serde_json::from_str(string) {
            Ok(records) => return Ok(records),
            Err(err) => err,
        };

        Err(Error::new(ErrorKind::Deserialization {
            string: string.to_owned(),
            format: "JSON",
            source: Box::new(err),
        }))
    }

    fn to_string(&self, records: &BTreeMap<String, Record>) -> String {
        serde_json::to_string_pretty(records).unwrap()
    }
}
