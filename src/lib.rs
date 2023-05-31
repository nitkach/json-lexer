mod lexer;
mod parser;
pub mod colors_test;

use crate::parser::{Value, ParsingError, ParsingContext};

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn parse(string: &str) -> Result<Value, ParsingError> {
    let context = ParsingContext::new();
    context.parse(string)
}
