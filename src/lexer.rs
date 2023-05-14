mod cursor;

use std::ops::Mul;

use cursor::Cursor;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) len: u32,
    pub(crate) kind: TokenKind,
}

pub(crate) struct NumberContext {
    number: f64,
    fraction: f64,
    first_char: char,
    state: State,
}
/*
{
    "a": "string",
    "b": true,
    "m": [
        10,
        20
    ]
}
*/

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    String,
    Number(f64),
    // done
    True,
    // done
    False,

    Colon,
    Comma,
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

    Invalid,
}

enum State {
    Sign,
    LeadingZero,
    IntegerPart,
    Mantissa,
    Dot,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { len, kind }
    }
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
                '-' => State::Sign,
                '0' => State::LeadingZero,
                '1'..='9' => State::IntegerPart,
                _ => unreachable!(),
            },
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

    fn number_sign(self) -> f64 {
        if self.first_char == '-' {
            -self.number
        } else {
            self.number
        }
    }

    fn push_mantissa_digit(&mut self, num: char) {
        let digit = f64::from(num.to_digit(10).unwrap());

        self.number = self.fraction.mul_add(digit, self.number);
        self.fraction *= 0.1;
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

        /*
        {
            "a": "string",
            "b": tru,
            "m": [
                10,
                20
            ]
        }
        */

        let token_kind = match first_char {
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::ClosedCurly,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::ClosedBracket,

            // have: 'n', 'u'. need: peek_first:'l'-> peek_second: 'l'
            'n' => self.eat_chars(TokenKind::Null, "ull"),
            't' => self.eat_chars(TokenKind::True, "rue"),
            'f' => self.eat_chars(TokenKind::False, "alse"),
            // Number
            // -0.9, 1
            // 00.9
            '0'..='9' | '-' => self.eat_number(first_char),

            _ => todo!(),
        };
        let res = Token::new(token_kind, self.token_len());
        self.reset_token_len();
        Some(res)
    }

    fn eat_number(&mut self, first_char: char) -> TokenKind {
        // TODO: scientific notation | binary OR hexadecimal form
        let mut context = NumberContext::new(first_char);

        loop {
            let char = self.peek_first();
            match (&context.state, char) {
                // -0
                (State::Sign, Some('0')) => {
                    context.state = State::LeadingZero;
                }

                // -1..=9
                (State::Sign, Some(num @ '1'..='9')) => {
                    context.state = State::IntegerPart;
                    context.push_integer_digit(num);
                }

                // 0. | // 0..=9 .
                (State::LeadingZero | State::IntegerPart, Some('.')) => {
                    context.state = State::Dot;
                }

                // .0..=9
                (State::Dot, Some(num @ '0'..='9')) => {
                    context.state = State::Mantissa;
                    context.push_mantissa_digit(num);
                }

                // 0..=9 0..=9
                (State::IntegerPart, Some(num @ '0'..='9')) => {
                    context.push_integer_digit(num);
                }

                // 0..=9: 0..=9
                (State::Mantissa, Some(num @ '0'..='9')) => {
                    context.push_mantissa_digit(num);
                }

                // -AnyChar | .AnyChar | 0{0, 1..=9}
                (State::Sign, _) | (State::Dot, _) | (State::LeadingZero, Some('0'..='9')) => {
                    return TokenKind::Invalid
                }

                // .0..=9
                (State::Mantissa | State::IntegerPart | State::LeadingZero, _) => {
                    return TokenKind::Number(context.number_sign());
                }
            }
            self.eat_char();
        }
    }

    fn eat_chars(&mut self, kind: TokenKind, chars: &str) -> TokenKind {
        for char in chars.chars() {
            if Some(char) == self.peek_first() {
                self.eat_char();
            } else {
                return TokenKind::Invalid;
            }
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
    fn smoke() {
        assert_snapshot("-0", "2:Number(-0.0)");
        // assert_snapshot("-1", "2:Number(-1.0)");
        assert_snapshot("0.", "2:Invalid");
        // assert_snapshot(".9", "2:Invalid");
        assert_snapshot("10", "2:Number(10.0)");
        assert_snapshot("1.1", "3:Number(1.1)");
        assert_snapshot("0", "1:Number(0.0)");

        assert_snapshot("10.250", "6:Number(10.25)");
        assert_snapshot("-0.01", "5:Number(-0.01)");
        assert_snapshot("-100.000001", "11:Number(-100.0)");
        assert_snapshot("[100.200]", "1:OpenBracket,7:Number(100.2),1:ClosedBracket");
        assert_snapshot("1-00", "1:Number(1.0),2:Invalid,1:Number(0.0)");
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
