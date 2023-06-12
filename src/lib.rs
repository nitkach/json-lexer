mod lexer;
mod parser;
mod texts;
pub mod colors_test;

use crate::parser::{Value, ParsingContext, ParsingErrorContext};

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// ParsingErrorContext store internal(?) implementation variables
pub fn parse(string: &str) -> Result<Value, ParsingErrorContext> {
    let context = ParsingContext::new();
    context.parse(string)
}
