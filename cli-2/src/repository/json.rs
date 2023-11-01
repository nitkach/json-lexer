use super::{Formatter, RecordData};
use crate::error::{Error, ErrorKind};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn from_str(&self, string: &str) -> Result<BTreeMap<String, RecordData>, Error> {
        serde_json::from_str(string).map_err(|err| {
            Error::new(ErrorKind::Deserialization {
                string: string.to_owned(),
                format: "JSON",
                source: Box::new(err),
            })
        })
    }

    fn to_string(&self, records: &BTreeMap<String, RecordData>) -> String {
        serde_json::to_string_pretty(records).unwrap()
    }
}
