use std::str::Chars;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
pub(crate) struct Cursor<'a> {
    token_len_and_remaining: usize,
    /// Iterator over chars. Slightly faster than a &str.
    chars: Chars<'a>,
    line: usize,
    column: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            token_len_and_remaining: input.len(),
            chars: input.chars(),
            line: 1,
            column: 0,
        }
    }

    pub(crate) fn get_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    pub(crate) fn peek_first(&self) -> Option<char> {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next()
    }

    /// Resets the number of bytes consumed to 0.
    pub(crate) fn reset_token_len(&mut self) {
        self.token_len_and_remaining = self.chars.as_str().len();
    }

    /// Moves to the next character.
    pub(crate) fn eat_char(&mut self) -> Option<char> {
        let char = self.chars.next()?;
        match char {
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            _ => {
                self.column += 1;
            }
        }
        Some(char)
    }
}
