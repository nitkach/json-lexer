use std::error::Error;
use std::fmt;

// use super::{ParsingContext, ParsingError, Expectation, ExpectingValue};
use crate::lexer::{TokenKind, TokenizeError};
use crate::parser::{Expectation, ExpectingValue, KvState, ParsingContext};

#[derive(Debug)]
pub struct ParsingError {
    pub(crate) error: ParsingErrorKind,
    pub(crate) context: ParsingContext,
    pub(crate) token_kind: Option<TokenKind>,
    pub(crate) position: Option<(usize, usize)>,
}

impl Error for ParsingError {}

#[derive(Debug)]
pub(crate) enum ParsingErrorKind {
    Syntax,
    ExpectedValue,
    ExpectedKey,
    ExpectedEndOfFile,
    ExpectedColon,
    TrailingComma,
    ExpectedCommaOrClosedCurly,
    ExpectedCommaOrClosedBracket,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.context.expectation {
            Expectation::Value => match self.context.stack.last() {
                Some(obj_or_arr) => match obj_or_arr {
                    ExpectingValue::Obj { acc: _, key } => {
                        write!(f, "Expected value after key \"{key}\" ")?;
                    }
                    ExpectingValue::Arr { acc } => {
                        write!(f, "Expected array value ")?;
                        if acc.is_empty() {
                            write!(f, "or closing bracket, ")?;
                        }
                    }
                },
                None => {
                    write!(f, "Expected JSON object, array or literal - ")?;
                }
            },
            // Expected string after "Fluttershy" but found comma unexpectedly | ExpectedKey
            Expectation::Obj { acc, kv } => match kv {
                KvState::Start => {
                    write!(f, "Expected string ")?;
                    if acc.is_empty() {
                        write!(f, "or closing curly, ")?;
                    }
                }
                KvState::AteKey(key) => {
                    write!(f, "Expected colon after key \"{key}\", ")?;
                }
                KvState::AteValue => {
                    write!(f, "Expected comma or closing curly, ")?;
                }
            },
            Expectation::CommaOrClosedBracket { acc: _ } => {
                write!(f, "Expected comma or closed bracket, ")?;
            }
            Expectation::EndOfTokens(_) => {
                write!(f, "Expected end of tokens, ")?;
            }
        }

        match &self.error {
            ParsingErrorKind::Syntax => {
                match &self.token_kind {
                    Some(TokenKind::Invalid(tokenize_error)) => match tokenize_error {
                        TokenizeError::NoSuchToken(char) => {
                            write!(f, "found '{char}' ")?;
                        }
                        TokenizeError::NoSuchEscapeSymbol(char) => {
                            write!(f, "'{char}' - invalid escape symbol ")?;
                        }
                        TokenizeError::MissingDoubleQuote(string) => {
                            write!(f, "missing double quote in: \"{string}\" ")?;
                        }
                        TokenizeError::ExpectedDigit(char) => {
                            write!(f, "expected digit, found '{char}' ")?;
                        }
                        TokenizeError::ExpectedDot(char) => {
                            write!(f, "expected dot, found '{char}'")?;
                        }
                        TokenizeError::MetEndOfFile => {
                            write!(f, "met end of file ")?;
                        }
                        TokenizeError::ExpectedTrue(char) => {
                            write!(f, "expected 'true' literal, found \"{char}\" ")?;
                        }
                        TokenizeError::ExpectedFalse(char) => {
                            write!(f, "expected 'false' literal, found \"{char}\" ")?;
                        }
                        TokenizeError::ExpectedNull(char) => {
                            write!(f, "expected 'null' literal, found \"{char}\" ")?;
                        }
                        TokenizeError::InvalidUnicode(num) => {
                            write!(f, "there is no symbol with code {num} ")?;
                        }
                        TokenizeError::InvalidUnicodeChar(char) => {
                            write!(f, "invalid unicode symbol: '{char}' ")?;
                        }
                    },
                    Some(_) => write!(f, "BUG(Some({:?})) ", &self.token_kind)?,
                    None => write!(f, "BUG(None) ")?,
                };
                if f.alternate() {
                    write!(f, "(Syntax) ")?;
                }
            }

            ParsingErrorKind::ExpectedValue => {
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::Colon => write!(f, "colon ")?,
                            TokenKind::Comma => write!(f, "comma ")?,
                            TokenKind::ClosedCurly => write!(f, "closed curly ")?,
                            TokenKind::ClosedBracket => write!(f, "closed bracket ")?,
                            _ => {
                                write!(f, "BUG({:?}) ", token_kind)?;
                            }
                        };
                    }
                    None => write!(f, "but the string ended ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(ExpectedValue) ")?;
                }
            }

            ParsingErrorKind::ExpectedKey => {
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::Number(num) => write!(f, "number {num} ")?,
                            TokenKind::True => write!(f, "'true' ")?,
                            TokenKind::False => write!(f, "'false' ")?,
                            TokenKind::Colon => write!(f, "colon ")?,
                            TokenKind::Comma => write!(f, "comma ")?,
                            TokenKind::OpenCurly => write!(f, "open curly ")?,
                            TokenKind::OpenBracket => write!(f, "open bracket ")?,
                            TokenKind::ClosedBracket => write!(f, "closed bracket ")?,
                            TokenKind::Null => write!(f, "'null' ")?,

                            _ => write!(f, "BUG({:?}) ", token_kind)?,
                        }
                    }
                    None => write!(f, "but the string ended ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(ExpectedKey) ")?;
                }
            }
            ParsingErrorKind::ExpectedEndOfFile => {
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::String(string) => write!(f, "string \"{string}\"")?,
                            TokenKind::Number(number) => write!(f, "number {number}")?,
                            TokenKind::True => write!(f, "'true' ")?,
                            TokenKind::False => write!(f, "'false' ")?,
                            TokenKind::Colon => write!(f, "colon ")?,
                            TokenKind::Comma => write!(f, "comma ")?,
                            TokenKind::OpenCurly => write!(f, "open curly ")?,
                            TokenKind::ClosedCurly => write!(f, "closed curly ")?,
                            TokenKind::OpenBracket => write!(f, "open bracket ")?,
                            TokenKind::ClosedBracket => write!(f, "closed bracket ")?,
                            TokenKind::Null => write!(f, "'null' ")?,
                            TokenKind::Invalid(_) => write!(f, "extra characters ")?,

                            TokenKind::Whitespace => write!(f, "BUG(TokenKind::Whitespace) ")?,
                        }
                    }
                    None => write!(f, "BUG(None) ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(ExpectedEndOfFile) ")?;
                }
            }
            ParsingErrorKind::ExpectedColon => {
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::String(string) => write!(f, "string \"{string}\"")?,
                            TokenKind::Number(num) => write!(f, "number {num}")?,
                            TokenKind::True => write!(f, "'true' ")?,
                            TokenKind::False => write!(f, "'false' ")?,
                            TokenKind::Comma => write!(f, "comma ")?,
                            TokenKind::OpenCurly => write!(f, "open curly ")?,
                            TokenKind::ClosedCurly => write!(f, "closed curly ")?,
                            TokenKind::OpenBracket => write!(f, "open bracket ")?,
                            TokenKind::ClosedBracket => write!(f, "closed bracket ")?,
                            TokenKind::Null => write!(f, "'null' ")?,
                            TokenKind::Invalid(_) => write!(f, "extra characters ")?,

                            _ => write!(f, "BUG({:?}) ", token_kind)?,
                        }
                    }
                    None => write!(f, "but the string ended ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(ExpectedColon) ")?;
                }
            }
            ParsingErrorKind::TrailingComma => {
                match &self.token_kind {
                    Some(TokenKind::ClosedCurly) => write!(f, "but found trailing comma ")?,
                    Some(_) => write!(f, "but found BUG({:?})", &self.token_kind)?,
                    None => write!(f, "BUG(None) ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(TrailingComma) ")?;
                }
            }
            ParsingErrorKind::ExpectedCommaOrClosedCurly => {
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::String(string) => write!(f, "string \"{string}\"")?,
                            TokenKind::Number(number) => write!(f, "number {number}")?,
                            TokenKind::True => write!(f, "'true' ")?,
                            TokenKind::False => write!(f, "'false' ")?,
                            TokenKind::Colon => write!(f, "colon ")?,
                            TokenKind::OpenCurly => write!(f, "open curly ")?,
                            TokenKind::OpenBracket => write!(f, "open bracket ")?,
                            TokenKind::ClosedBracket => write!(f, "closed bracket ")?,
                            TokenKind::Null => write!(f, "'null' ")?,
                            TokenKind::Invalid(_) => write!(f, "extra characters ")?,

                            _ => write!(f, "BUG({:?}) ", token_kind)?,
                        }
                    }
                    None => write!(f, "but the string ended ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(ExpectedCommaOrClosedCurly) ")?;
                }
            }
            ParsingErrorKind::ExpectedCommaOrClosedBracket => {
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::String(string) => write!(f, "string \"{string}\"")?,
                            TokenKind::Number(number) => write!(f, "number {number}")?,
                            TokenKind::True => write!(f, "'true' ")?,
                            TokenKind::False => write!(f, "'false' ")?,
                            TokenKind::Colon => write!(f, "colon ")?,
                            TokenKind::OpenCurly => write!(f, "open curly ")?,
                            TokenKind::OpenBracket => write!(f, "open bracket ")?,
                            TokenKind::Null => write!(f, "'null' ")?,
                            TokenKind::Invalid(_) => write!(f, "extra characters ")?,
                            TokenKind::ClosedCurly => write!(f, "closed curly ")?,

                            _ => write!(f, "BUG({:?}) ", token_kind)?,
                        }
                    }
                    None => write!(f, "but the string ended ")?,
                };
                write!(f, "unexpectedly ")?;
                if f.alternate() {
                    write!(f, "(ExpectedCommaOrClosedBracket) ")?;
                }
            }
        }

        match self.position {
            Some((line, column)) => {
                write!(f, "at line {}, column {}", line, column)?;
            }
            None => {
                write!(f, "at the end")?;
            }
        }
        Ok(())
    }
}
