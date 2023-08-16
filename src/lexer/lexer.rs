use crate::lexer::token::TokenType;

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

        if Lexer::skip_whitespace_or_new_line(curr) {
            return Some(Token::whitespace());
        }

        if curr.is_alphabetic() {
            let word = self.consume_word(curr);
            match word.as_str() {
                "let" => return Some(Token::new_let()),
                "fn" => return Some(Token::function()),
                "true" => return Some(Token::new(TokenType::True, word)),
                "false" => return Some(Token::new(TokenType::False, word)),
                "return" => return Some(Token::new(TokenType::Return, word)),
                "if" => return Some(Token::new(TokenType::If, word)),
                "else" => return Some(Token::new(TokenType::Else, word)),
                _ => return Some(Token::new(TokenType::Indentifier, word)),
            }
        }

        if curr.is_digit(10) {
            let (number, literal) = self.consume_number(curr);
            return Some(Token::new(TokenType::Int(number), literal));
        }

        match curr {
            '=' => {
                let next = self.peek();

                match next {
                    Some(d) => match d {
                        '=' => {
                            self.consume_char();
                            Some(Token::new(TokenType::Eq, "==".to_string()))
                        }
                        _ => Some(Token::equal_sign()),
                    },
                    None => Some(Token::equal_sign()),
                }
            }
            '!' => {
                let next = self.peek();

                match next {
                    Some(c) => match c {
                        '=' => {
                            self.consume_char();
                            Some(Token::new(TokenType::NotEq, "!=".to_string()))
                        }
                        _ => Some(Token::bang()),
                    },
                    None => Some(Token::bang()),
                }
            }

            ';' => Some(Token::semicolon()),
            '+' => Some(Token::new(TokenType::PlusSign, curr.to_string())),
            '-' => Some(Token::new(TokenType::MinusSign, curr.to_string())),
            '*' => Some(Token::new(TokenType::MultiplicationSign, curr.to_string())),
            '/' => Some(Token::new(TokenType::SlashSign, curr.to_string())),
            '{' => Some(Token::left_brace()),
            '}' => Some(Token::right_brace()),
            '(' => Some(Token::left_paren()),
            ')' => Some(Token::right_paren()),
            ',' => Some(Token::comma()),
            '<' => Some(Token::lt()),
            '>' => Some(Token::gt()),
            _ => panic!("token: {:?} has not been implemented yet.", curr),
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

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn consume_word(&mut self, mut initial_char: char) -> String {
        let mut word = String::from("");

        loop {
            word.push(initial_char);

            match self.peek() {
                Some(d) => {
                    if !d.is_alphabetic() {
                        break;
                    }
                }
                None => {
                    break;
                }
            }

            initial_char = self.consume_char();
        }

        return word;
    }

    fn consume_number(&mut self, mut initial_char: char) -> (i64, String) {
        let mut number: i64 = 0;
        let mut literal = String::from("");

        loop {
            literal.push(initial_char);
            // safely assume we can parse and unwrap because we have validation down below.
            let d = initial_char.to_digit(10).unwrap() as i64;
            number = number * 10 + d;

            match self.peek() {
                Some(v) => {
                    if !v.is_digit(10) {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }

            initial_char = self.consume_char();
        }

        return (number, literal);
    }

    fn skip_whitespace_or_new_line(c: char) -> bool {
        if c == ' ' || c == '\n' || c == '\r' {
            return true;
        };

        return false;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn t() {
        assert_eq!(' '.is_alphabetic(), true)
    }
}
