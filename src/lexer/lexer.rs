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
                _ => return Some(Token::identifier(word)),
            }
        }

        if curr.is_ascii_digit() {
            let number = self.consume_number(curr);
            return Some(Token::int(number));
        }

        match curr {
            '=' => {
                let next = self.peek();

                match next {
                    Some('=') => {
                        self.consume_char();
                        Some(Token::new(TokenType::Eq, "==".to_string()))
                    }
                    _ => Some(Token::assign_sign()),
                }
            }
            '!' => {
                let next = self.peek();

                match next {
                    Some('=') => {
                        self.consume_char();
                        Some(Token::new(TokenType::NotEq, "!=".to_string()))
                    }
                    _ => Some(Token::bang()),
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

        c
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

        word
    }

    fn consume_number(&mut self, mut initial_char: char) -> i64 {
        let mut number: i64 = 0;

        loop {
            // safely assume we can parse and unwrap because we have validation down below.
            let d = initial_char.to_digit(10).unwrap() as i64;
            number = number * 10 + d;

            match self.peek() {
                Some(v) => {
                    if !v.is_ascii_digit() {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }

            initial_char = self.consume_char();
        }

        number
    }

    fn skip_whitespace_or_new_line(c: char) -> bool {
        if c == ' ' || c == '\n' || c == '\r' {
            return true;
        };

        false
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    };

    fn run_tokenizer(mut lex: Lexer) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(t) = lex.next_token() {
            match t.kind {
                TokenType::Whitespace => {}
                _ => tokens.push(t),
            }
        }

        return tokens;
    }

    #[test]
    fn tokenize_let_statement() {
        let input = "
            let x = 512;
            let y = 256;
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::new_let(),
            Token::identifier("x".to_string()),
            Token::assign_sign(),
            Token::int(512),
            Token::semicolon(),
            Token::new_let(),
            Token::identifier("y".to_string()),
            Token::assign_sign(),
            Token::int(256),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_if_else_statement() {
        let input = "
            if (x < 10) {
                return 10;
            } else if (x > 12) {
                return 20;
            } else {
                return 30;
            }
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::new(TokenType::If, "if".to_string()),
            Token::left_paren(),
            Token::identifier("x".to_string()),
            Token::lt(),
            Token::int(10),
            Token::right_paren(),
            Token::left_brace(),
            Token::new(TokenType::Return, "return".to_string()),
            Token::int(10),
            Token::semicolon(),
            Token::right_brace(),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::If, "if".to_string()),
            Token::left_paren(),
            Token::identifier("x".to_string()),
            Token::gt(),
            Token::int(12),
            Token::right_paren(),
            Token::left_brace(),
            Token::new(TokenType::Return, "return".to_string()),
            Token::int(20),
            Token::semicolon(),
            Token::right_brace(),
            Token::new(TokenType::Else, "else".to_string()),
            Token::left_brace(),
            Token::new(TokenType::Return, "return".to_string()),
            Token::int(30),
            Token::semicolon(),
            Token::right_brace(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_function_statement() {
        let input = "
            let a = fn(x, y) { };

            fn myFunc() { }
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::new_let(),
            Token::identifier("a".to_string()),
            Token::assign_sign(),
            Token::function(),
            Token::left_paren(),
            Token::identifier("x".to_string()),
            Token::comma(),
            Token::identifier("y".to_string()),
            Token::right_paren(),
            Token::left_brace(),
            Token::right_brace(),
            Token::semicolon(),
            Token::function(),
            Token::identifier("myFunc".to_string()),
            Token::left_paren(),
            Token::right_paren(),
            Token::left_brace(),
            Token::right_brace(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }
}
