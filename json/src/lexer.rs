mod cursor;

use cursor::Cursor;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) len: u32,
    pub(crate) kind: TokenKind,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { len, kind }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    String(String),
    Number(f64),
    True,
    False,

    // Not Value
    Colon,
    // done
    Comma,
    // done
    Whitespace,

    // done
    OpenCurly,
    // done
    ClosedCurly,

    // done
    OpenBracket,
    // done
    ClosedBracket,

    // done
    Null,

    Invalid(TokenizeError),
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenizeError {
    MetEndOfFile,

    InvalidUnicode(String),
    MissingDoubleQuote(String),

    InvalidUnicodeChar(char),
    NoSuchToken(char),
    NoSuchEscapeSymbol(char),
    ExpectedDigit(char),
    ExpectedDot(char),
    ExpectedTrue(char),
    ExpectedFalse(char),
    ExpectedNull(char),
}

enum NumberState {
    Sign,
    LeadingZero,
    IntegerPart,
    Mantissa,
    Dot,
}

enum StringState {
    String,
    Escape,
}

pub(crate) struct NumberContext {
    number: f64,
    fraction: f64,
    first_char: char,
    state: NumberState,
}

pub(crate) struct StringContext {
    string: String,
    state: StringState,
}

impl NumberContext {
    fn new(first_char: char) -> NumberContext {
        NumberContext {
            number: if let Some(digit) = first_char.to_digit(10) {
                f64::from(digit)
            } else {
                0.0
            },
            fraction: 0.1,
            first_char,
            state: match first_char {
                '-' => NumberState::Sign,
                '0' => NumberState::LeadingZero,
                '1'..='9' => NumberState::IntegerPart,
                _ => unreachable!(),
            },
        }
    }

    fn number_sign(self) -> f64 {
        if self.first_char == '-' {
            -self.number
        } else {
            self.number
        }
    }

    fn push_integer_digit(&mut self, num: char) {
        let digit = f64::from(num.to_digit(10).unwrap());

        // -123
        // first_char = '-' = -0.0
        //  -0.0 * 10 =   -0.0 + (-1.0) = -1.0
        //  -1.0 * 10 =  -10.0 + (-2.0) = -12.0
        // -12.0 * 10 = -120.0 + (-3.0) = -123.0
        // -(12.0 * 10 + (3.0)) = -123.0
        // number * 10 - digit

        // 123
        // first_char = '1' = 1.0
        //   1.0 * 10 =   10.0 + (2.0) = 12.0
        //  12.0 * 10 =  120.0 + (3.0) = 123.0
        // number * 10 + digit

        // suggestion: we know the first character of the number -
        // - we can make it negative at the end of the function or
        // leave it unchanged: if first_char == '-' {
        //    return -number
        // } else {
        //    return number
        // }
        self.number = self.number.mul_add(10.0, digit);
    }

    fn push_mantissa_digit(&mut self, num: char) {
        let digit = f64::from(num.to_digit(10).unwrap());

        self.number = self.fraction.mul_add(digit, self.number);
        self.fraction *= 0.1;
    }
}

impl StringContext {
    fn new() -> StringContext {
        StringContext {
            string: "".to_owned(),
            state: StringState::String,
        }
    }
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn eat_token(&mut self) -> Option<Token> {
        // struct Cursor { len_remaining, chars }
        //
        //    token_len_and_remaining = 9|remaining = chars.as_str().len() = 5
        //      /---|----\
        // Ford,Mare,1950
        //      |   ^
        //      |   chars.next() -> Some(',')
        //      |
        //      ^- (last reset position)
        //         token_len = token_len_and_remainging - chars.as_str().len() = 4
        //

        let first_char = self.eat_char()?;

        let token_kind = match first_char {
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::ClosedCurly,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::ClosedBracket,

            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,

            // null, true, false
            'n' => self.eat_bool_or_null(TokenKind::Null, "ull"),
            't' => self.eat_bool_or_null(TokenKind::True, "rue"),
            'f' => self.eat_bool_or_null(TokenKind::False, "alse"),

            // Number
            '0'..='9' | '-' => self.eat_number(first_char),

            // String
            '"' => self.eat_string(),

            first_char => {
                if first_char.is_whitespace() {
                    TokenKind::Whitespace
                } else {
                    TokenKind::Invalid(TokenizeError::NoSuchToken(first_char))
                }
            }
        };
        let res = Token::new(token_kind, self.token_len());
        self.reset_token_len();
        Some(res)
    }

    fn eat_string(&mut self) -> TokenKind {
        // BUG: json control symbols consider invalid and
        // TODO: add other unescape symbols
        let mut context = StringContext::new();
        loop {
            let char = self.eat_char();
            match (&context.state, char) {
                // \
                (StringState::String, Some('\\')) => {
                    context.state = StringState::Escape;
                }
                // "
                (StringState::String, Some('"')) => return TokenKind::String(context.string),
                //
                (StringState::String, Some(char)) => {
                    context.string.push(char);
                    context.state = StringState::String;
                }
                // \"
                (StringState::Escape, Some(char)) => {
                    let unescaped = match char {
                        '"' => '"',
                        '\\' => '\\',
                        '/' => '/',        // solidus: '\/'
                        'b' => '\u{232B}', // backspace
                        'f' => '\u{000C}', // formfeed
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        'u' => {
                            let mut buf = "".to_owned();

                            for _ in 0..4 {
                                let Some(char) = self.peek_first() else {
                                    return TokenKind::Invalid(TokenizeError::MetEndOfFile)
                                };

                                if !char.is_ascii_hexdigit() {
                                    return TokenKind::Invalid(TokenizeError::InvalidUnicodeChar(
                                        char,
                                    ));
                                }

                                buf.push(char);
                                self.eat_char();
                            }

                            let Ok(unicode) = u32::from_str_radix(&buf, 16) else {
                                return TokenKind::Invalid(TokenizeError::InvalidUnicode(buf))
                            };

                            let Some(unicode_char) = char::from_u32(unicode) else {
                                return TokenKind::Invalid(TokenizeError::InvalidUnicode(buf))
                            };
                            unicode_char
                        }
                        _ => return TokenKind::Invalid(TokenizeError::NoSuchEscapeSymbol(char)),
                    };
                    context.string.push(unescaped);
                    context.state = StringState::String;
                }
                // "str\EOF
                (StringState::String | StringState::Escape, None) => {
                    return TokenKind::Invalid(TokenizeError::MissingDoubleQuote(context.string))
                }
            }
        }
    }

    fn eat_number(&mut self, first_char: char) -> TokenKind {
        // TODO: scientific notation | binary OR hexadecimal form
        let mut context = NumberContext::new(first_char);

        loop {
            let char = self.peek_first();
            match (&context.state, char) {
                // -0
                (NumberState::Sign, Some('0')) => {
                    context.state = NumberState::LeadingZero;
                }

                // -1..=9
                (NumberState::Sign, Some(num @ '1'..='9')) => {
                    context.state = NumberState::IntegerPart;
                    context.push_integer_digit(num);
                }

                // 0. | // 0..=9 .
                (NumberState::LeadingZero | NumberState::IntegerPart, Some('.')) => {
                    context.state = NumberState::Dot;
                }

                // .0..=9
                (NumberState::Dot, Some(num @ '0'..='9')) => {
                    context.state = NumberState::Mantissa;
                    context.push_mantissa_digit(num);
                }

                // 0..=9 0..=9
                (NumberState::IntegerPart, Some(num @ '0'..='9')) => {
                    context.push_integer_digit(num);
                }

                // 0..=9: 0..=9
                (NumberState::Mantissa, Some(num @ '0'..='9')) => {
                    context.push_mantissa_digit(num);
                }

                // -AnyChar | .AnyChar | 0{0, 1..=9}
                // -K, .k, 01
                (NumberState::Sign | NumberState::Dot, Some(char)) => {
                    return TokenKind::Invalid(TokenizeError::ExpectedDigit(char))
                }

                (NumberState::Sign | NumberState::Dot, None) => {
                    return TokenKind::Invalid(TokenizeError::MetEndOfFile)
                }

                (NumberState::LeadingZero, Some(char @ '0'..='9')) => {
                    return TokenKind::Invalid(TokenizeError::ExpectedDot(char))
                }

                // .0..=9
                (
                    NumberState::Mantissa | NumberState::IntegerPart | NumberState::LeadingZero,
                    _,
                ) => {
                    return TokenKind::Number(context.number_sign());
                }
            }
            self.eat_char();
        }
    }

    fn eat_bool_or_null(&mut self, kind: TokenKind, expected: &str) -> TokenKind {
        for expected_char in expected.chars() {
            let char = if let Some(char) = self.peek_first() {
                char
            } else {
                return TokenKind::Invalid(TokenizeError::MetEndOfFile);
            };

            if char == expected_char {
                self.eat_char();
                continue;
            }

            let error = match kind {
                TokenKind::True => TokenizeError::ExpectedTrue(char),
                TokenKind::False => TokenizeError::ExpectedFalse(char),
                TokenKind::Null => TokenizeError::ExpectedNull(char),
                _ => unreachable!(),
            };
            return TokenKind::Invalid(error);
        }
        kind
    }
}

// Box<[Token]>
pub(crate) fn tokenize(string: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut cursor = Cursor::new(string);

    while let Some(token) = cursor.eat_token() {
        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    // create tests for state transition (for every match arm)
    #[test]
    fn smoke_number() {
        assert_snapshot("-0", "2:Number(-0.0)");
        // assert_snapshot("-1", "2:Number(-1.0)");
        assert_snapshot("0.", "2:Invalid(MetEndOfFile)");
        // assert_snapshot(".9", "2:Invalid");
        assert_snapshot("10", "2:Number(10.0)");
        assert_snapshot("1.1", "3:Number(1.1)");
        assert_snapshot("0", "1:Number(0.0)");

        assert_snapshot("10.250", "6:Number(10.25)");
        assert_snapshot("-0.01", "5:Number(-0.01)");
        assert_snapshot("-100.000001", "11:Number(-100.0)");
        assert_snapshot("[100.200]", "1:OpenBracket,7:Number(100.2),1:ClosedBracket");
        assert_snapshot(
            "1-00",
            "1:Number(1.0),2:Invalid(ExpectedDot('0')),1:Number(0.0)",
        );
        assert_snapshot("-201.102", "8:Number(-201.102)");
    }

    #[test]
    fn smoke_string() {
        assert_snapshot("\"abcd\"", "6:String(\"abcd\")");
        assert_snapshot("\"a\\\"bc\\\"d\"", "10:String(\"a\\\"bc\\\"d\")"); // "a\"b\"c" -> a"b"c
        assert_snapshot("\"ab\\ncd\"", "8:String(\"ab\\ncd\")");
        assert_snapshot("\"ab\\tcd\"", "8:String(\"ab\\tcd\")");
        assert_snapshot("\"ab\\\\cd\"", "8:String(\"ab\\\\cd\")");
        assert_snapshot("\"ab\\rcd\"", "8:String(\"ab\\rcd\")");

        assert_snapshot("\"abcd", "5:Invalid(MissingDoubleQuote)");
    }

    #[track_caller]
    fn assert_snapshot(string: &str, expected: &str) {
        let tokens = tokenize(string);

        let mut actual = vec![];

        for elem in tokens {
            let Token { len, mut kind } = elem;

            if let TokenKind::Number(num) = &mut kind {
                *num = (*num * 1000.0).trunc() / 1000.0;
            }
            // let to_print = match kind {
            //     TokenKind::Number(num) => TokenKind::Number(((num * 1000.0).trunc()) / 1000.0),
            //     _ => kind,
            // };
            actual.push(format!("{len}:{kind:?}"));
        }

        assert_eq!(actual.join(","), expected)
    }
}
