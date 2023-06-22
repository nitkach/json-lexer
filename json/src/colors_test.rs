// use crate::lexer::{self, TokenKind};
// use nu_ansi_term::{Color, Style};
// use std::io;

// pub fn colorise() {
//     let stdin = io::read_to_string(io::stdin()).unwrap();
//     let mut len: usize = 0;

//     let tokens = lexer::tokenize(&stdin);

//     for token in tokens {
//         let mut style = Style::new();
//         match token.kind {
//             TokenKind::String(_) => {
//                 style = style.fg(Color::Blue).bold();
//             }
//             TokenKind::Number(_) => {
//                 style = style.bold();
//             }
//             TokenKind::True => {
//                 style = style.fg(Color::Green);
//             }
//             TokenKind::False => {
//                 style = style.fg(Color::Red);
//             }
//             TokenKind::Null => {
//                 style = style.fg(Color::LightGray).italic();
//             }
//             TokenKind::Invalid(_) => {
//                 style = style.fg(Color::Magenta).dimmed();
//             }
//             TokenKind::Whitespace => {}
//             _ => {
//                 style = style.fg(Color::White).bold();
//             }
//         }
//         // style = style.fg(Color::Yellow);
//         print!("{}", style.paint(&stdin[len..(len + token.len as usize)]));

//         len += token.len as usize;
//     }
// }
