mod parsing_error_context;

pub(crate) use parsing_error_context::{ParsingError, ParsingErrorKind};
use std::collections::BTreeMap;

use crate::lexer::{self, Token, TokenKind};

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

#[derive(Debug)]
enum ExpectingValue {
    Obj {
        acc: BTreeMap<String, Value>,
        key: String,
    },

    Arr {
        acc: Vec<Value>,
    },
}

#[derive(Debug)]
enum Expectation {
    Value,
    Obj {
        acc: BTreeMap<String, Value>,
        kv: KvState,
    },
    CommaOrClosedBracket {
        acc: Vec<Value>,
    },
    EndOfTokens(Value),
}

#[derive(Debug)]
enum KvState {
    Start,
    AteKey(String),
    AteValue,
}

#[derive(Debug)]
pub(crate) struct ParsingContext {
    stack: Vec<ExpectingValue>,
    expectation: Expectation,
}

struct ParsingLoopContext {
    base: ParsingContext,
    token: Token,
}

impl ParsingLoopContext {
    fn create_error(self, error: ParsingErrorKind) -> ParsingError {
        ParsingError {
            error,
            context: self.base,
            token_kind: Some(self.token.kind),
            position: Some((self.token.line, self.token.column)),
        }
    }

    fn eat_token(mut self) -> Result<ParsingContext, ParsingError> {
        match &mut self.base.expectation {
            Expectation::Value => match self.token.kind {
                TokenKind::String(string) => self.base.make_value(Value::String(string)),
                TokenKind::Number(num) => self.base.make_value(Value::Number(num)),
                TokenKind::True => self.base.make_value(Value::Bool(true)),
                TokenKind::False => self.base.make_value(Value::Bool(false)),
                TokenKind::Null => self.base.make_value(Value::Null),

                TokenKind::Whitespace => {}

                TokenKind::OpenCurly => {
                    self.base.expectation = Expectation::Obj {
                        acc: BTreeMap::new(),
                        kv: KvState::Start,
                    };
                }
                TokenKind::OpenBracket => {
                    self.base
                        .stack
                        .push(ExpectingValue::Arr { acc: Vec::new() });
                }

                TokenKind::ClosedBracket => {
                    let Some(peeked) = self.base.stack.last() else {
                        return Err(self.create_error(ParsingErrorKind::ExpectedValue))
                    };

                    let acc = match peeked {
                        ExpectingValue::Arr { acc } => acc,
                        ExpectingValue::Obj { acc: _, key: _ } => {
                            return Err(self.create_error(ParsingErrorKind::ExpectedValue))
                        }
                    };
                    if !acc.is_empty() {
                        return Err(self.create_error(ParsingErrorKind::ExpectedValue));
                    }
                    self.base.stack.pop();
                    self.base.make_value(Value::Array(Vec::new()));
                }

                TokenKind::Invalid(_) => return Err(self.create_error(ParsingErrorKind::Syntax)),

                _ => return Err(self.create_error(ParsingErrorKind::ExpectedValue)),
            },

            Expectation::Obj { acc, kv } => match kv {
                KvState::Start => match self.token.kind {
                    TokenKind::String(string) => *kv = KvState::AteKey(string),
                    TokenKind::ClosedCurly => {
                        if acc.is_empty() {
                            self.base.make_value(Value::Object(BTreeMap::new()));
                            return Ok(self.base);
                        }
                        return Err(self.create_error(ParsingErrorKind::TrailingComma));
                    }
                    TokenKind::Whitespace => {}

                    TokenKind::Invalid(_) => {
                        return Err(self.create_error(ParsingErrorKind::Syntax))
                    }
                    _ => return Err(self.create_error(ParsingErrorKind::ExpectedKey)),
                },
                KvState::AteKey(key) => match self.token.kind {
                    TokenKind::Colon => {
                        self.base.stack.push(ExpectingValue::Obj {
                            acc: std::mem::take(acc),
                            key: std::mem::take(key),
                        });
                        self.base.expectation = Expectation::Value;
                    }
                    TokenKind::Whitespace => {}

                    TokenKind::Invalid(_) => {
                        return Err(self.create_error(ParsingErrorKind::Syntax))
                    }
                    _ => return Err(self.create_error(ParsingErrorKind::ExpectedColon)),
                },
                KvState::AteValue => match self.token.kind {
                    TokenKind::Comma => {
                        *kv = KvState::Start;
                    }
                    TokenKind::ClosedCurly => {
                        let buf = Value::Object(std::mem::take(acc));
                        self.base.make_value(buf);
                    }

                    TokenKind::Whitespace => {}

                    TokenKind::Invalid(_) => {
                        return Err(self.create_error(ParsingErrorKind::Syntax))
                    }

                    _ => {
                        return Err(self.create_error(ParsingErrorKind::ExpectedCommaOrClosedCurly))
                    }
                },
            },
            Expectation::CommaOrClosedBracket { acc } => match self.token.kind {
                TokenKind::Comma => {
                    self.base.stack.push(ExpectingValue::Arr {
                        acc: std::mem::take(acc),
                    });
                    self.base.expectation = Expectation::Value;
                }
                TokenKind::ClosedBracket => {
                    let buf = Value::Array(std::mem::take(acc));
                    self.base.make_value(buf);
                }

                TokenKind::Whitespace => {}

                TokenKind::Invalid(_) => return Err(self.create_error(ParsingErrorKind::Syntax)),
                _ => return Err(self.create_error(ParsingErrorKind::ExpectedCommaOrClosedBracket)),
            },
            Expectation::EndOfTokens(_) => {
                if TokenKind::Whitespace == self.token.kind {
                    return Ok(self.base);
                }
                return Err(self.create_error(ParsingErrorKind::ExpectedEndOfFile));
            }
        }

        Ok(self.base)
    }
}

impl ParsingContext {
    pub(crate) fn new() -> ParsingContext {
        ParsingContext {
            stack: Vec::new(),
            expectation: Expectation::Value,
        }
    }

    pub(crate) fn parse(mut self, string: &str) -> Result<Value, ParsingError> {
        for token in lexer::tokenize(string) {
            let ctx = ParsingLoopContext { base: self, token };
            self = ctx.eat_token()?;
        }
        let error = match self.expectation {
            Expectation::EndOfTokens(value) => return Ok(value),
            Expectation::Value => ParsingErrorKind::ExpectedValue,
            Expectation::Obj { acc: _, ref kv } => match kv {
                KvState::Start => ParsingErrorKind::ExpectedKey,
                KvState::AteKey(_) => ParsingErrorKind::ExpectedCommaOrClosedCurly,
                KvState::AteValue => ParsingErrorKind::ExpectedCommaOrClosedCurly,
            },
            Expectation::CommaOrClosedBracket { acc: _ } => {
                ParsingErrorKind::ExpectedCommaOrClosedBracket
            }
        };

        Err(ParsingError {
            error,
            context: self,
            token_kind: None,
            position: None,
        })
    }

    // receive stack, not self
    // context: we find simple literal or finished creating a Value::(Obj or Arr)
    fn make_value(&mut self, value: Value) {
        let Some(popped) = self.stack.pop() else {
            self.expectation = Expectation::EndOfTokens(value);
            return;
        };
        match popped {
            ExpectingValue::Obj {
                acc: mut pop_acc,
                key: pop_key,
            } => {
                pop_acc.insert(pop_key, value);
                self.expectation = Expectation::Obj {
                    acc: pop_acc,
                    kv: KvState::AteValue,
                };
            }
            ExpectingValue::Arr { acc: mut pop_acc } => {
                pop_acc.push(value);
                self.expectation = Expectation::CommaOrClosedBracket { acc: pop_acc };
            }
        }
        // match self.stack.pop() {
        //     Some(popped) => match popped {
        //         ExpectingValue::Obj {
        //             acc: mut pop_acc,
        //             key: pop_key,
        //         } => {
        //             pop_acc.insert(pop_key, value);
        //             self.expectation = Expectation::Obj {
        //                 acc: pop_acc,
        //                 kv: KvState::AteValue,
        //             };
        //         }
        //         ExpectingValue::Arr { acc: mut pop_acc } => {
        //             pop_acc.push(value);
        //             self.expectation = Expectation::CommaOrClosedBracket { acc: pop_acc };
        //         }
        //     },
        //     None => {
        //         self.expectation = Expectation::EndOfTokens(value);
        //     }
        // }
    }
}

#[cfg(test)]
mod test;

// fn iterate_slice(slice: &[usize]) {
//     match slice {
//         [] => {}
//         [x, remainder @ ..] => {
//             println!("{x}");
//             iterate_slice(remainder);
//         }
//     }
// }
