use core::fmt;
use std::collections::BTreeMap;

use crate::lexer::{self, TokenKind, TokenizeError};

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
enum ParsingError {
    Syntax,
    ExpectedValue,
    ExpectedKey,
    ExpectedEndOfFile,
    ExpectedColon,
    TrailingComma,
    ExpectedCommaOrClosedCurly,
    ExpectedCommaOrClosedBracket,
}

#[derive(Debug)]
pub struct ParsingErrorContext {
    error: ParsingError,
    context: ParsingContext,
    token_kind: Option<TokenKind>,
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

impl fmt::Display for ParsingErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.context.expectation {
            Expectation::Value => match self.context.stack.last() {
                Some(obj_or_arr) => match obj_or_arr {
                    ExpectingValue::Obj { acc: _, key } => {
                        write!(f, "Expected value after key \"{key}\" ")?;
                    }
                    ExpectingValue::Arr { acc: _ } => {
                        write!(f, "Expected array value ")?;
                    }
                },
                None => {
                    write!(f, "Expected JSON object, array or literal - ")?;
                }
            },
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
            // passed Syntax with found error-caused context
            ParsingError::Syntax => {
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
                        },
                        TokenizeError::InvalidUnicodeChar(char) => {
                            write!(f, "invalid unicode symbol: '{char}' ")?;
                        },
                    },
                    Some(_) => write!(f, "BUG(Some({:?})) ", &self.token_kind)?,
                    None => write!(f, "BUG(None) ")?,
                };
                write!(f, "| Syntax")?;
            }

            // passed like "effect" after discrepancy
            ParsingError::ExpectedValue => {
                // cause: Expectation::Value -> effect: ParsingError: ExpectedValue
                match &self.token_kind {
                    Some(token_kind) => {
                        write!(f, "but found ")?;
                        match token_kind {
                            TokenKind::Colon => write!(f, "colon ")?,
                            TokenKind::Comma => write!(f, "comma ")?,
                            TokenKind::ClosedCurly => write!(f, "closed curly ")?,
                            TokenKind::ClosedBracket => write!(f, "closed bracket ")?,
                            _ => {
                                // because in other TokenKind's process of deserialization will continue
                                write!(f, "BUG({:?}) ", token_kind)?;
                            }
                        };
                    }
                    None => write!(f, "but the string ended ")?,
                };
                write!(f, "unexpectedly | ExpectedValue")?
            }

            ParsingError::ExpectedKey => {
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
                write!(f, "unexpectedly | ExpectedKey")?
                // cause: Expectation::Obj { acc: _, kv: Start } -> effect: ParsingError: ExpectedKey
            }
            ParsingError::ExpectedEndOfFile => {
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
                // cause: Expectation::EndOfTokens, but found another one -> effect: ParsingError: ExpectedEndOfFile
                // write!(f, )?;
                write!(f, "unexpectedly | ExpectedEndOfFile")?
            }
            ParsingError::ExpectedColon => {
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
                write!(f, "unexpectedly | ExpectedColon")?
            }
            ParsingError::TrailingComma => {
                /*
                {
                    "string-num": 10,
                }
                 */
                match &self.token_kind {
                    Some(TokenKind::ClosedCurly) => write!(f, "but found trailing comma ")?,
                    Some(_) => write!(f, "but found BUG({:?})", &self.token_kind)?,
                    None => write!(f, "BUG(None) ")?,
                };
                write!(f, "unexpectedly | TrailingComma")?;
            }
            ParsingError::ExpectedCommaOrClosedCurly => {
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
                write!(f, "unexpectedly | ExpectedCommaOrClosedCurly")?;
            }
            ParsingError::ExpectedCommaOrClosedBracket => {
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
                write!(f, "unexpectedly | ExpectedCommaOrClosedBracket")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct ParsingContext {
    stack: Vec<ExpectingValue>,
    expectation: Expectation,
}

impl ParsingContext {
    pub(crate) fn new() -> ParsingContext {
        ParsingContext {
            stack: Vec::new(),
            expectation: Expectation::Value,
        }
    }

    pub(crate) fn parse(mut self, string: &str) -> Result<Value, ParsingErrorContext> {
        for token in lexer::tokenize(string) {
            match &mut self.expectation {
                Expectation::Value => match token.kind {
                    TokenKind::String(string) => self.make_value(Value::String(string)),
                    TokenKind::Number(num) => self.make_value(Value::Number(num)),
                    TokenKind::True => self.make_value(Value::Bool(true)),
                    TokenKind::False => self.make_value(Value::Bool(false)),
                    TokenKind::Null => self.make_value(Value::Null),

                    TokenKind::Whitespace => {}

                    TokenKind::OpenCurly => {
                        self.expectation = Expectation::Obj {
                            acc: BTreeMap::new(),
                            kv: KvState::Start,
                        };
                    }
                    TokenKind::OpenBracket => {
                        self.stack.push(ExpectingValue::Arr { acc: Vec::new() });
                    }

                    TokenKind::ClosedBracket => {
                        let Some(peeked) = self.stack.last() else {
                            return Err(ParsingErrorContext {
                                error: ParsingError::ExpectedValue,
                                context: self,
                                token_kind: Some(token.kind)
                            })
                        };

                        let acc = match peeked {
                            ExpectingValue::Arr { acc } => acc,
                            ExpectingValue::Obj { acc: _, key: _ } => {
                                return Err(ParsingErrorContext {
                                    error: ParsingError::ExpectedValue,
                                    context: self,
                                    token_kind: Some(token.kind),
                                })
                            }
                        };
                        if !acc.is_empty() {
                            return Err(ParsingErrorContext {
                                error: ParsingError::ExpectedValue,
                                context: self,
                                token_kind: Some(token.kind),
                            });
                        }
                        self.stack.pop();
                        self.make_value(Value::Array(Vec::new()));
                    }

                    TokenKind::Invalid(_) => {
                        //   take here ^
                        return Err(ParsingErrorContext {
                            error: ParsingError::Syntax,
                            //                move here ^
                            context: self,
                            token_kind: Some(token.kind),
                            //               ^ = TokenKind(Invalid(/*moved TokenizeError*/))
                            // token_kind: None
                        });
                    }

                    _ => {
                        return Err(ParsingErrorContext {
                            error: ParsingError::ExpectedValue,
                            context: self,
                            token_kind: Some(token.kind),
                        })
                    }
                },

                Expectation::Obj { acc, kv } => match kv {
                    KvState::Start => match token.kind {
                        TokenKind::String(string) => *kv = KvState::AteKey(string),
                        TokenKind::ClosedCurly => {
                            if acc.is_empty() {
                                self.make_value(Value::Object(BTreeMap::new()));
                                continue;
                            }
                            return Err(ParsingErrorContext {
                                error: ParsingError::TrailingComma,
                                context: self,
                                token_kind: Some(token.kind),
                            });
                        }
                        TokenKind::Whitespace => {}

                        TokenKind::Invalid(_) => {
                            return Err(ParsingErrorContext {
                                error: ParsingError::Syntax,
                                context: self,
                                token_kind: Some(token.kind),
                            })
                        }
                        _ => {
                            return Err(ParsingErrorContext {
                                error: ParsingError::ExpectedKey,
                                context: self,
                                token_kind: Some(token.kind),
                            })
                        }
                    },
                    KvState::AteKey(key) => match token.kind {
                        TokenKind::Colon => {
                            self.stack.push(ExpectingValue::Obj {
                                acc: std::mem::take(acc),
                                key: std::mem::take(key),
                            });
                            self.expectation = Expectation::Value;
                        }
                        TokenKind::Whitespace => {}

                        TokenKind::Invalid(_) => {
                            return Err(ParsingErrorContext {
                                error: ParsingError::Syntax,
                                context: self,
                                token_kind: Some(token.kind),
                            })
                        }
                        _ => {
                            return Err(ParsingErrorContext {
                                error: ParsingError::ExpectedColon,
                                context: self,
                                token_kind: Some(token.kind),
                            })
                        }
                    },
                    KvState::AteValue => match token.kind {
                        TokenKind::Comma => {
                            *kv = KvState::Start;
                        }
                        TokenKind::ClosedCurly => {
                            let buf = Value::Object(std::mem::take(acc));
                            self.make_value(buf);
                        }

                        TokenKind::Whitespace => {}

                        TokenKind::Invalid(_) => {
                            return Err(ParsingErrorContext {
                                error: ParsingError::Syntax,
                                context: self,
                                token_kind: Some(token.kind),
                            })
                        }

                        _ => {
                            return Err(ParsingErrorContext {
                                error: ParsingError::ExpectedCommaOrClosedCurly,
                                context: self,
                                token_kind: Some(token.kind),
                            })
                        }
                    },
                },
                Expectation::CommaOrClosedBracket { acc } => match token.kind {
                    TokenKind::Comma => {
                        self.stack.push(ExpectingValue::Arr {
                            acc: std::mem::take(acc),
                        });
                        self.expectation = Expectation::Value;
                    }
                    TokenKind::ClosedBracket => {
                        let buf = Value::Array(std::mem::take(acc));
                        self.make_value(buf);
                    }

                    TokenKind::Whitespace => {}

                    TokenKind::Invalid(_) => {
                        return Err(ParsingErrorContext {
                            error: ParsingError::Syntax,
                            context: self,
                            token_kind: Some(token.kind),
                        })
                    }
                    _ => {
                        return Err(ParsingErrorContext {
                            error: ParsingError::ExpectedCommaOrClosedBracket,
                            context: self,
                            token_kind: Some(token.kind),
                        })
                    }
                },
                Expectation::EndOfTokens(_) => {
                    if TokenKind::Whitespace == token.kind {
                        continue;
                    }
                    return Err(ParsingErrorContext {
                        error: ParsingError::ExpectedEndOfFile,
                        context: self,
                        token_kind: Some(token.kind),
                    });
                }
            }
        }
        match self.expectation {
            Expectation::EndOfTokens(value) => Ok(value),
            Expectation::Value => Err(ParsingErrorContext {
                error: ParsingError::ExpectedValue,
                context: self,
                token_kind: None,
            }),
            Expectation::Obj { acc: _, ref kv } => match kv {
                KvState::Start => Err(ParsingErrorContext {
                    error: ParsingError::ExpectedKey,
                    context: self,
                    token_kind: None,
                }),
                KvState::AteKey(_) => Err(ParsingErrorContext {
                    error: ParsingError::ExpectedColon,
                    context: self,
                    token_kind: None,
                }),
                KvState::AteValue => Err(ParsingErrorContext {
                    error: ParsingError::ExpectedCommaOrClosedCurly,
                    context: self,
                    token_kind: None,
                }),
            },
            Expectation::CommaOrClosedBracket { acc: _ } => Err(ParsingErrorContext {
                error: ParsingError::ExpectedCommaOrClosedBracket,
                context: self,
                token_kind: None,
            }),
        }
    }

    // receive stack, not self
    // context: we find simple literal or finished creating a Value::(Obj or Arr)
    fn make_value(&mut self, value: Value) {
        match self.stack.pop() {
            Some(popped) => match popped {
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
            },
            None => {
                self.expectation = Expectation::EndOfTokens(value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[track_caller]
    fn assert_snapshot(string: &str, expected: &str) {
        let json_value = crate::parse(string);

        match json_value {
            Ok(value) => {
                let mut actual = format!("{value:?}");
                if actual.len() > 60 {
                    actual = format!("{value:#?}");
                }
                assert_eq!(actual, expected);
            }
            Err(error) => {
                assert_eq!(format!("{error}"), expected);
            }
        }
    }

    #[test]
    fn smoke_ok() {
        assert_snapshot(
            r#"{"mare": true, "snowpity": "legendary", "cute_level": 999}"#,
            r#"Object({"cute_level": Number(999.0), "mare": Bool(true), "snowpity": String("legendary")})"#,
        );
    }

    #[test]
    fn smoke_error() {
        assert_snapshot(
            r#"{"mare": true, "snowpity":"#,
            "Expected value after key \"snowpity\" but the string ended unexpectedly | ExpectedValue"
        );
    }

    #[test]
    fn empty_complex() {
        assert_snapshot("{}", "Object({})");
        assert_snapshot("[]", "Array([])");
        assert_snapshot(
            "[[[[[[]]]]]]",
            "Array([Array([Array([Array([Array([Array([])])])])])])",
        );
        assert_snapshot(
            r#"{"a":{},"b":{},"c":{}}"#,
            r#"Object({"a": Object({}), "b": Object({}), "c": Object({})})"#,
        );
        assert_snapshot("[{}, []]", "Array([Object({}), Array([])])");
        assert_snapshot(
            r#"{"arr": [], "obj": {}}"#,
            r#"Object({"arr": Array([]), "obj": Object({})})"#,
        );
    }

    #[test]
    fn derpibooru() {
        let response = std::fs::read_to_string("./src/derpibooru_example_response.json").unwrap();
        let actual = crate::texts::derpibooru_deserealized();
        assert_snapshot(&response, &actual);
    }

    #[test]
    fn menu() {
        let response = crate::texts::menu_string();
        let actual = crate::texts::menu_deserealized();
        // timer start

        assert_snapshot(&response, &actual);
        // timer end
        // print spended time
    }

    #[test]
    fn simple_literal() {
        assert_snapshot("10", "Number(10.0)");
        assert_snapshot("\"string\"", "String(\"string\")");
        assert_snapshot("true", "Bool(true)");
        assert_snapshot("false", "Bool(false)");
        assert_snapshot("null", "Null");
    }

    #[test]
    fn object_in_object() {
        assert_snapshot(
            r#"{"mare": {"name": "fluttershy"}}"#,
            r#"Object({"mare": Object({"name": String("fluttershy")})})"#,
        );
    }

    #[test]
    fn object_in_array() {
        assert_snapshot(
            r#"[{"mare": true}]"#,
            r#"Array([Object({"mare": Bool(true)})])"#,
        );
    }

    #[test]
    fn error_object() {
        assert_snapshot(
            r#"{""}"#,
            r#"Expected colon after key "", but found closed curly unexpectedly | ExpectedColon"#
        );

        assert_snapshot(
            r#"{"string"}"#,
            r#"Expected colon after key "string", but found closed curly unexpectedly | ExpectedColon"#
        );

        assert_snapshot(
            r#"{"string":}"#,
            r#"Expected value after key "string" but found closed curly unexpectedly | ExpectedValue"#
        );

        assert_snapshot(
            r#"{"string-invalid": bbb}"#,
            r#"Expected value after key "string-invalid" found 'b' | Syntax"#
        );

        assert_snapshot(
            r#"{"string-num": 10,}"#,
            r#"Expected string but found trailing comma unexpectedly | TrailingComma"#
        );

        assert_snapshot(
            r#"{"string-num": 10}{"#,
            r#"Expected end of tokens, but found open curly unexpectedly | ExpectedEndOfFile"#
        )
    }

    #[test]
    fn error_arr() {
        assert_snapshot(
            r#"[,]"#,
            r#"Expected array value but found comma unexpectedly | ExpectedValue"#
        );

        assert_snapshot(
            r#"[10,]]"#,
            r#"Expected array value but found closed bracket unexpectedly | ExpectedValue"#
        );

        assert_snapshot(
            r#"[10,{]}]"#,
            r#"Expected string or closing curly, but found closed bracket unexpectedly | ExpectedKey"#
        );

        assert_snapshot(
            r#"["string""#,
            r#"Expected comma or closed bracket, but the string ended unexpectedly | ExpectedCommaOrClosedBracket"#
        );

        assert_snapshot(
            r#"["string": 10]"#,
            r#"Expected comma or closed bracket, but found colon unexpectedly | ExpectedCommaOrClosedBracket"#
        );
    }

    #[test]
    fn error_string() {
        assert_snapshot(
            r#""string1"#,
            r#"Expected JSON object, array or literal - missing double quote in: "string1" | Syntax"#
        );

        assert_snapshot(
            r#""string2\""#,
            r#"Expected JSON object, array or literal - missing double quote in: "string2"" | Syntax"#
        );
    }

    #[test]
    fn error_string_unicode() {
        assert_snapshot(
            r#""mare \u2764""#,
            r#"String("mare ❤")"#
        );
    }
}

// fn iterate_slice(slice: &[usize]) {
//     match slice {
//         [] => {}
//         [x, remainder @ ..] => {
//             println!("{x}");
//             iterate_slice(remainder);
//         }
//     }
// }
