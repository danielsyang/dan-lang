use crate::ast::token::{TokenKind, TokenSpan};

use super::token::Token;

pub struct Lexer {
    pub input: String,
    pub current_pos: usize,
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
                literal: TokenSpan::new(0, 0, '\0'.to_string()),
            });
        }

        let start = self.current_pos;
        let curr = self.input.chars().nth(self.current_pos);

        return curr.map(|c| {
            if Self::is_number(&c) {
                let number = self.consume_number();
                let end = self.current_pos;
                return Token {
                    kind: TokenKind::Number(number),
                    literal: TokenSpan::new(start, end, number.to_string()),
                };
            } else if Self::is_whitespace(&c) {
                let consume = self.consume_char();
                let end = self.current_pos;
                return Token {
                    kind: TokenKind::Whitespace,
                    literal: TokenSpan::new(start, end, consume.to_string()),
                };
            } else if let Some(operator) = Self::is_operator(&c) {
                let consume = self.consume_char();
                let end = self.current_pos;
                return Token {
                    kind: operator,
                    literal: TokenSpan::new(start, end, consume.to_string()),
                };
            } else {
                todo!("Validate other tokens")
            }
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

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;

        while let Some(c) = self.input.chars().nth(self.current_pos) {
            if c.is_digit(10) {
                self.consume_char();
                number = number * 10 + c.to_digit(10).unwrap() as i64
            } else {
                break;
            }
        }

        return number;
    }

    fn is_number(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn is_operator(c: &char) -> Option<TokenKind> {
        match c {
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '*' => Some(TokenKind::Asterisk),
            '/' => Some(TokenKind::Slash),
            _ => None,
        }
    }
}
