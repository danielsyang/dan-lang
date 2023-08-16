use crate::ast::token::TokenType;

use super::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.input.len() < self.position {
            return None;
        }
        if self.input.len() == self.position {
            self.position += 1;
            return Some(Token::eof());
        }

        let curr = self.consume_char();

        if curr == ' ' {
            return Some(Token::whitespace());
        }

        if curr.is_alphabetic() {
            let word = self.consume_word(curr);
            match word.as_str() {
                "let" => return Some(Token::new_let()),
                _ => return Some(Token::new(TokenType::Variable, word)),
            }
        }

        if curr.is_digit(10) {}

        match curr {
            '=' => Some(Token::equal_sign()),
            ';' => Some(Token::semicolon()),
            '+' => Some(Token::new(TokenType::PlusSign, curr.to_string())),
            '-' => Some(Token::new(TokenType::MinusSign, curr.to_string())),
            '*' => Some(Token::new(TokenType::MultiplicationSign, curr.to_string())),
            '/' => Some(Token::new(TokenType::DivisionSign, curr.to_string())),
            _ => panic!("token: {} has not been implemented yet.", curr),
        }
    }

    fn consume_char(&mut self) -> char {
        let c = self
            .input
            .chars()
            .nth(self.position)
            .expect("Invalid lexer state, current position is larger than input");

        self.position += 1;

        return c;
    }

    fn consume_word(&mut self, mut initial_char: char) -> String {
        let mut word = String::from("");

        loop {
            word.push(initial_char);

            let peek = self.input.chars().nth(self.position);
            match peek {
                Some(' ') => {
                    break;
                }
                _ => {
                    // do nothing
                }
            }

            initial_char = self.consume_char();
        }

        return word;
    }
}

#[cfg(test)]
mod test {}
