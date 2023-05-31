use std::{collections::BTreeMap, iter::Map};

use crate::lexer::{self, TokenKind};

pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

pub enum ParsingError {
    Syntax,
    ExpectedValue,
    ExpectedEndOfFile,
}

enum State {
    Start,
    Obj {
        acc: BTreeMap<String, Value>,
        kv: KvState,
    },
    Arr {
        acc: Vec<Value>,
        val: ValState,
    },
    SimpleLiteral(Value),
}

enum KvState {
    Start,
    AteKey(String),
    AteColon(String),
    AteValue,
}

enum ValState {
    Start,
    AteValue,
}
/*
{
    "string-number": 10,
    "string-object": {
        "string-array": [
            10,
            20,
            30
        ]
    }
}
*/

pub(crate) struct ParsingContext {
    stack: Vec<State>, // for storing
    cur: State,
}

impl ParsingContext {
    pub(crate) fn new() -> ParsingContext {
        ParsingContext {
            stack: Vec::new(),
            cur: State::Start,
        }
    }

    pub(crate) fn parse(mut self, string: &str) -> Result<Value, ParsingError> {
        /*
        cur: Start
        stack: []
        token: OpenCurly

        cur: Obj { {}, Start }
        stack: []
        token: String("foo")

        cur: Obj { {}, AteKey("foo") }
        stack: []
        token: Colon

        cur: Start
        stack: [Obj { {}, AteColon("foo") }]
        token: Number(25)

        cur: Obj { {"foo": 25}, AteValue }
        stack: []
        token: ,

        cur: Obj { {"foo": 25}, Start }
        stack: []
        token: "bar"

        cur: Obj { {"foo": 25}, AteKey("bar") }
        stack: []
        token: :

        cur: Start
        stack: [Obj { {"foo": 25}, AteColon("bar") }]
        token: [

        cur: Arr { acc: [], Start }
        stack: [Obj { {"foo": 25}, AteColon("bar") }]
        token: 1

        cur: Arr { acc: [1], AteVal }
        stack: [Obj { {"foo": 25}, AteColon("bar") }]
        token: ,



        {
            "foo": 25,
            "bar": [1, 2, [3, 4]]
        }
        */
        // #TODO whitespaces
        for token in lexer::tokenize(string) {
            match (&mut self.cur, token.kind) {
                (State::Start, TokenKind::OpenCurly) => {
                    self.cur = State::Obj {
                        acc: BTreeMap::new(),
                        kv: KvState::Start,
                    };
                }

                (State::Start, TokenKind::OpenBracket) => {
                    self.cur = State::Arr {
                        acc: Vec::new(),
                        val: ValState::Start,
                    };
                }

                (State::Start, TokenKind::Number(num)) => self.make_value(Value::Number(num)),
                (State::Start, TokenKind::String(string)) => self.make_value(Value::String(string)),
                (State::Start, TokenKind::True) => self.make_value(Value::Bool(true)),
                (State::Start, TokenKind::False) => self.make_value(Value::Bool(false)),
                (State::Start, TokenKind::Null) => self.make_value(Value::Null),

                (State::SimpleLiteral(_), _) => {
                    // error
                }

                (State::Arr { acc, val }, TokenKind::Number(num)) => {
                    match val {
                        ValState::Start => {
                            *val = ValState::AteValue;
                            acc.push(Value::Number(num));
                        }
                        ValState::AteValue => {
                            // error
                        }
                    }
                }

                (State::Arr { acc, val }, TokenKind::Comma) => {
                    match val {
                        ValState::Start => {
                            // error
                        }
                        ValState::AteValue => {
                            *val = ValState::Start;
                        }
                    }
                }

                // [1, 2, [3, 4]]

                // []
                // [1, ]
                (State::Arr { acc, val }, TokenKind::ClosedBracket) => match val {
                    ValState::Start => {
                        if acc.is_empty() {
                            self.make_value(Value::Array(Vec::new()));
                        } else {
                            // error: [1, ]
                        }
                    }
                    ValState::AteValue => todo!(),
                },

                (State::Obj { acc, kv }, TokenKind::ClosedCurly) => match kv {
                    KvState::Start | KvState::AteValue => {
                        let map = std::mem::take(acc);
                        self.make_value(Value::Object(map));
                    }
                    KvState::AteKey(_) => todo!(),
                    KvState::AteColon(_) => todo!(),
                },

                (State::Arr { acc, val }, TokenKind::OpenBracket) => {
                    match val {
                        ValState::Start => {
                            *val = ValState::AteValue;
                            self.stack.push(std::mem::replace(
                                &mut self.cur,
                                State::Arr {
                                    acc: Vec::new(),
                                    val: ValState::Start,
                                },
                            ))
                        }
                        ValState::AteValue => {
                            // error
                        }
                    }
                }

                (State::Obj { acc, kv }, TokenKind::Comma) => {
                    match kv {
                        KvState::Start => {
                            // error
                        }
                        KvState::AteKey(_) => {
                            // error
                        }
                        KvState::AteColon(_) => {
                            // error
                        }
                        KvState::AteValue => {
                            *kv = KvState::Start;
                        }
                    }
                }

                (State::Obj { acc, kv }, TokenKind::String(str)) => {
                    match kv {
                        KvState::Start => {
                            *kv = KvState::AteKey(str);
                        }
                        KvState::AteKey(_) => {
                            // error (?)
                        }
                        KvState::AteColon(key_colon) => {
                            acc.insert(std::mem::take(key_colon), Value::String(str));
                            // valid key = String
                        }
                        KvState::AteValue => {
                            // error
                        }
                    }
                }

                (State::Obj { acc, kv }, TokenKind::Colon) => {
                    match kv {
                        KvState::Start => {
                            // error
                        }
                        KvState::AteKey(key) => {
                            *kv = KvState::AteColon(std::mem::take(key));
                            self.stack
                                .push(std::mem::replace(&mut self.cur, State::Start));
                        }
                        KvState::AteColon(_) => {
                            // error
                        }
                        KvState::AteValue => {
                            // error
                        }
                    }
                }

                (State::Obj { acc, kv }, TokenKind::Number(num)) => {
                    match kv {
                        KvState::Start => {
                            // error
                        }
                        KvState::AteKey(_) => {
                            // error
                        }
                        KvState::AteColon(key_colon) => {
                            acc.insert(std::mem::take(key_colon), Value::Number(num));
                        }
                        KvState::AteValue => {
                            // error
                        }
                    }
                }

                // ate colon
                (
                    State::Obj { acc, kv },
                    value @ (TokenKind::True | TokenKind::False | TokenKind::Null),
                ) => {
                    match kv {
                        KvState::Start => {
                            // error
                        }
                        KvState::AteKey(_) => {
                            // error
                        }
                        KvState::AteColon(key_colon) => {
                            acc.insert(
                                std::mem::take(key_colon),
                                match value {
                                    TokenKind::True => Value::Bool(true),
                                    TokenKind::False => Value::Bool(false),
                                    TokenKind::Null => Value::Null,
                                    _ => {
                                        unreachable!();
                                    }
                                },
                            );
                        }
                        KvState::AteValue => {
                            // error
                        }
                    }
                }
                (State::Start, TokenKind::Colon) => todo!(),
                (State::Start, TokenKind::Comma) => todo!(),
                (State::Start, TokenKind::Whitespace) => todo!(),
                (State::Start, TokenKind::ClosedCurly) => todo!(),
                (State::Start, TokenKind::ClosedBracket) => todo!(),
                (State::Start, TokenKind::Invalid) => todo!(),
                (State::Obj { acc, kv }, TokenKind::True) => todo!(),
                (State::Obj { acc, kv }, TokenKind::False) => todo!(),
                (State::Obj { acc, kv }, TokenKind::Whitespace) => todo!(),
                (State::Obj { acc, kv }, TokenKind::OpenCurly) => todo!(),
                (State::Obj { acc, kv }, TokenKind::OpenBracket) => todo!(),
                (State::Obj { acc, kv }, TokenKind::ClosedBracket) => todo!(),
                (State::Obj { acc, kv }, TokenKind::Null) => todo!(),
                (State::Obj { acc, kv }, TokenKind::Invalid) => todo!(),
                (State::Arr { acc, val }, TokenKind::String(_)) => todo!(),
                (State::Arr { acc, val }, TokenKind::True) => todo!(),
                (State::Arr { acc, val }, TokenKind::False) => todo!(),
                (State::Arr { acc, val }, TokenKind::Colon) => todo!(),
                (State::Arr { acc, val }, TokenKind::Whitespace) => todo!(),
                (State::Arr { acc, val }, TokenKind::OpenCurly) => todo!(),
                (State::Arr { acc, val }, TokenKind::ClosedCurly) => todo!(),
                (State::Arr { acc, val }, TokenKind::Null) => todo!(),
                (State::Arr { acc, val }, TokenKind::Invalid) => todo!(),

                // _ => todo!(),
            }
        };
        match self.cur {
            State::Start => todo!(),
            State::Obj { acc, kv } => {
                Ok(Value::Object(acc))
            },
            State::Arr { acc, val } => {
                Ok(Value::Array(acc))
            },
            State::SimpleLiteral(value) => {
                Ok(value)
            }
        }
    }

    // context: we find simple literal or finished creating a Value::(Obj or Arr)
    fn make_value(&mut self, value: Value) {
        match self.stack.pop() {
            Some(popped) => match popped {
                State::Obj {
                    acc: mut pop_acc,
                    kv: pop_kv,
                } => {
                    if let KvState::AteColon(key_colon) = pop_kv {
                        pop_acc.insert(key_colon, value);
                        self.cur = State::Obj {
                            acc: pop_acc,
                            kv: KvState::AteValue,
                        }
                    } else {
                        unreachable!()
                    }
                }
                State::Arr {
                    acc: mut pop_acc,
                    val: pop_val,
                } => {
                    if let ValState::Start = pop_val {
                        pop_acc.push(value);
                        self.cur = State::Arr {
                            acc: pop_acc,
                            val: ValState::AteValue,
                        }
                    } else {
                        // error - we expect value
                    }
                }
                State::Start => todo!(),
                State::SimpleLiteral(_) => todo!(),

            },
            None => {
                // have one primitive literal
                self.cur = State::SimpleLiteral(value);
            },
        }
    }
}
//     fn array_to_value(&mut self, acc: Vec<Value>) {
//         if acc.is_empty() {
//             match self.stack.pop() {
//                 Some(popped) => match popped {
//                     State::Obj {
//                         acc: mut pop_acc,
//                         kv: pop_kv,
//                     } => {
//                         if let KvState::AteColon(key_colon) = pop_kv {
//                             pop_acc.insert(key_colon, Value::Array(acc));
//                             self.cur = State::Obj {
//                                 acc: pop_acc,
//                                 kv: KvState::AteValue,
//                             };
//                         } else {
//                             unreachable!()
//                         }
//                     }

//                     State::Start => {
//                         unreachable!();
//                     }
//                 },
//                 None => {
//                     // maybe just "[]"
//                 }
//             }
//         } else {
//             // error
//             // [1, ]
//         }
//     }
// }

// fn iterate_slice(slice: &[usize]) {
//     match slice {
//         [] => {}
//         [x, remainder @ ..] => {
//             println!("{x}");
//             iterate_slice(remainder);
//         }
//     }
// }
