pub mod colors_test;
mod lexer;
mod parser;

use crate::parser::{ParsingContext, ParsingError};
pub use parser::Value;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn parse(string: &str) -> Result<Value, ParsingError> {
    let context = ParsingContext::new();
    context.parse(string)
}
