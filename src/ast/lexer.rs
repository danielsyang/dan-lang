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

        let curr = self.input.chars().nth(self.position);

        return curr.map(|c| {
            if c.is_alphabetic() {
                let word = self.consume_word();

                match word.as_str() {
                    "let" => return Token::new_let(),
                    _ => return Token::new(TokenType::Variable, word),
                }
            } else if c == ' ' {
                self.consume_char();
                return Token::whitespace();
            } else if c == '=' {
                self.consume_char();
                return Token::equal_sign();
            } else if c == ';' {
                self.consume_char();
                return Token::semicolon();
            } else if Self::current_is_operator(c) {
                self.consume_char();
                return Token::new(self.consume_operator(), c.to_string());
            } else {
                self.consume_char();
                return Token::eof();
            }
        });
    }

    pub fn consume_char(&mut self) -> char {
        let c = self
            .input
            .chars()
            .nth(self.position)
            .expect("Invalid lexer state, current position is larger than input");

        self.position += 1;

        return c;
    }

    pub fn consume_word(&mut self) -> String {
        let mut word = String::from("");
        let mut curr_char = self.consume_char();

        while curr_char != ' ' {
            word.push(curr_char);
            curr_char = self.consume_char();
        }

        // since whitespace is not consumed, decrement pointer, weird situation
        self.position -= 1;

        return word;
    }

    pub fn consume_operator(&mut self) -> TokenType {
        let c = self.consume_char();

        match c {
            '+' => TokenType::PlusSign,
            '-' => TokenType::MinusSign,
            '*' => TokenType::MultiplicationSign,
            '/' => TokenType::DivisionSign,
            _ => panic!("invalid operator"),
        }
    }

    pub fn current_is_operator(c: char) -> bool {
        match c {
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::lexer::Lexer;

    #[test]
    fn test_lexer() {
        let input = "let x = 5 + 5;";
        let mut lex = Lexer::new(input);

        while let Some(t) = lex.next_token() {
            println!("{:?}", t);
        }

        // assert_eq!(1, 2)
    }
}
