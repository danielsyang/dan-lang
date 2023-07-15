use crate::ast::token::{TokenKind, TokenSpan};

use super::token::Token;

pub struct Lexer {
    input: String,
    current_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            current_pos: 0,
            input,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.input.len() < self.current_pos {
            return None;
        }

        if self.input.len() == self.current_pos {
            self.current_pos += 1;
            return Some(Token {
                kind: TokenKind::EOF,
                literal: TokenSpan {
                    start: 0,
                    end: 0,
                    literal: '\0'.to_string(),
                },
            });
        }

        let start = self.current_pos;
        let curr = self.consume_char();
        let number = Self::transform_number(&curr);
        let end = self.current_pos;
        let literal = self.input[start..end].to_string();

        return Some(Token {
            kind: TokenKind::Number(number),
            literal: TokenSpan {
                start,
                end,
                literal,
            },
        });
    }

    fn consume_char(&mut self) -> char {
        let c = self
            .input
            .chars()
            .nth(self.current_pos)
            .expect("Invalid Lexer state, current_pos is larger than input");

        self.current_pos += 1;

        return c;
    }

    fn transform_number(c: &char) -> i64 {
        c.to_digit(10).unwrap() as i64
    }

    fn lookahead() -> bool {
        todo!("look ahead")
    }
}
