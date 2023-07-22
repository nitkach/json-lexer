mod parsing_error_context;
mod texts;

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
        let response = std::fs::read_to_string("./json/src/derpibooru_example_response.json").unwrap();
        let actual = super::texts::derpibooru_deserealized();
        assert_snapshot(&response, &actual);
    }

    #[test]
    fn menu() {
        let response = super::texts::menu_string();
        let actual = super::texts::menu_deserealized();
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
            r#"Expected colon after key "", but found closed curly unexpectedly | ExpectedColon"#,
        );

        assert_snapshot(
            r#"{"string"}"#,
            r#"Expected colon after key "string", but found closed curly unexpectedly | ExpectedColon"#,
        );

        assert_snapshot(
            r#"{"string":}"#,
            r#"Expected value after key "string" but found closed curly unexpectedly | ExpectedValue"#,
        );

        assert_snapshot(
            r#"{"string-invalid": bbb}"#,
            r#"Expected value after key "string-invalid" found 'b' | Syntax"#,
        );

        assert_snapshot(
            r#"{"string-num": 10,}"#,
            r#"Expected string but found trailing comma unexpectedly | TrailingComma"#,
        );

        assert_snapshot(
            r#"{"string-num": 10}{"#,
            r#"Expected end of tokens, but found open curly unexpectedly | ExpectedEndOfFile"#,
        )
    }

    #[test]
    fn error_arr() {
        assert_snapshot(
            r#"[,]"#,
            r#"Expected array value but found comma unexpectedly | ExpectedValue"#,
        );

        assert_snapshot(
            r#"[10,]]"#,
            r#"Expected array value but found closed bracket unexpectedly | ExpectedValue"#,
        );

        assert_snapshot(
            r#"[10,{]}]"#,
            r#"Expected string or closing curly, but found closed bracket unexpectedly | ExpectedKey"#,
        );

        assert_snapshot(
            r#"["string""#,
            r#"Expected comma or closed bracket, but the string ended unexpectedly | ExpectedCommaOrClosedBracket"#,
        );

        assert_snapshot(
            r#"["string": 10]"#,
            r#"Expected comma or closed bracket, but found colon unexpectedly | ExpectedCommaOrClosedBracket"#,
        );
    }

    #[test]
    fn error_string() {
        assert_snapshot(
            r#""string1"#,
            r#"Expected JSON object, array or literal - missing double quote in: "string1" | Syntax"#,
        );

        assert_snapshot(
            r#""string2\""#,
            r#"Expected JSON object, array or literal - missing double quote in: "string2"" | Syntax"#,
        );
    }

    #[test]
    fn error_string_unicode() {
        assert_snapshot(r#""mare \u2764""#, r#"String("mare ❤")"#);
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
