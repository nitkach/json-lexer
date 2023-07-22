pub mod colors_test;
mod lexer;
mod parser;

pub use parser::Value;
use crate::parser::{ParsingError, ParsingContext};

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn parse(string: &str) -> Result<Value, ParsingError> {
    let context = ParsingContext::new();
    context.parse(string)
}
