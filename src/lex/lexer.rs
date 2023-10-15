use crate::lex::token::TokenType;

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
                "true" => return Some(Token::boolean(true)),
                "false" => return Some(Token::boolean(false)),
                "return" => return Some(Token::new(TokenType::Return, word)),
                "if" => return Some(Token::new(TokenType::If, word)),
                "else" => return Some(Token::new(TokenType::Else, word)),
                "while" => return Some(Token::while_token()),
                _ => return Some(Token::identifier(word)),
            }
        }

        if curr.is_ascii_digit() {
            let number = self.consume_number(curr);
            return Some(Token::int(number));
        }

        match curr {
            '=' => match self.peek() {
                Some('=') => {
                    self.consume_char();
                    Some(Token::new(TokenType::Eq, "==".to_string()))
                }
                _ => Some(Token::assign_sign()),
            },
            '!' => match self.peek() {
                Some('=') => {
                    self.consume_char();
                    Some(Token::new(TokenType::NotEq, "!=".to_string()))
                }
                _ => Some(Token::bang()),
            },
            '.' => Some(Token::dot()),
            ';' => Some(Token::semicolon()),
            '+' => Some(Token::new(TokenType::PlusSign, curr.to_string())),
            '-' => Some(Token::new(TokenType::MinusSign, curr.to_string())),
            '*' => Some(Token::new(TokenType::MultiplicationSign, curr.to_string())),
            '/' => Some(Token::new(TokenType::SlashSign, curr.to_string())),
            '{' => Some(Token::left_brace()),
            '}' => Some(Token::right_brace()),
            '(' => Some(Token::left_paren()),
            ')' => Some(Token::right_paren()),
            '[' => Some(Token::left_bracket()),
            ']' => Some(Token::right_bracket()),
            ',' => Some(Token::comma()),
            '<' => match self.peek() {
                Some('=') => {
                    self.consume_char();
                    Some(Token::lte())
                }
                _ => Some(Token::lt()),
            },
            '>' => match self.peek() {
                Some('=') => {
                    self.consume_char();
                    Some(Token::gte())
                }
                _ => Some(Token::gt()),
            },
            '"' => Some(Token::string(self.consume_string())),
            ':' => Some(Token::colon()),
            '&' => match self.peek() {
                Some('&') => {
                    self.consume_char();
                    Some(Token::and())
                }
                // Bitwise operation
                _ => Some(Token::illegal()),
            },
            '|' => match self.peek() {
                Some('|') => {
                    self.consume_char();
                    Some(Token::or())
                }
                // Bitwise operation
                _ => Some(Token::illegal()),
            },
            _ => Some(Token::illegal()),
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
                    if !d.is_alphanumeric() {
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

    fn consume_string(&mut self) -> String {
        let mut curr = self.consume_char();
        let mut string = String::from("");

        loop {
            match curr {
                '"' => break,
                _ => {
                    string.push(curr);
                    curr = self.consume_char();
                }
            }
        }

        string
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
    use crate::lex::{
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
            let x1 = 128;
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
            Token::new_let(),
            Token::identifier("x1".to_string()),
            Token::assign_sign(),
            Token::int(128),
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

    #[test]
    fn tokenize_strings() {
        let input = "
            let abc = \"HELLO\";
            let cde = \"Hello world\";
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::new_let(),
            Token::identifier("abc".to_string()),
            Token::assign_sign(),
            Token::string("HELLO".to_string()),
            Token::semicolon(),
            Token::new_let(),
            Token::identifier("cde".to_string()),
            Token::assign_sign(),
            Token::string("Hello world".to_string()),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_arrays() {
        let input = "
            let abc = [1, 2, \"hello world\"];
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::new_let(),
            Token::identifier("abc".to_string()),
            Token::assign_sign(),
            Token::left_bracket(),
            Token::int(1),
            Token::comma(),
            Token::int(2),
            Token::comma(),
            Token::string("hello world".to_string()),
            Token::right_bracket(),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_indexes() {
        let input = "
            arr[1];
            [1, 2, 3][100];
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::identifier("arr".to_string()),
            Token::left_bracket(),
            Token::int(1),
            Token::right_bracket(),
            Token::semicolon(),
            Token::left_bracket(),
            Token::int(1),
            Token::comma(),
            Token::int(2),
            Token::comma(),
            Token::int(3),
            Token::right_bracket(),
            Token::left_bracket(),
            Token::int(100),
            Token::right_bracket(),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_hashmaps() {
        let input = "
            {\"foobar\": 10}
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::left_brace(),
            Token::string("foobar".to_string()),
            Token::colon(),
            Token::int(10),
            Token::right_brace(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_lte_gte() {
        let input = "
            1 >= 2;
            2 <= 1;
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::int(1),
            Token::gte(),
            Token::int(2),
            Token::semicolon(),
            Token::int(2),
            Token::lte(),
            Token::int(1),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn tokenize_and_or() {
        let input = "
            5 && 5;
            5 || 5;
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::int(5),
            Token::and(),
            Token::int(5),
            Token::semicolon(),
            Token::int(5),
            Token::or(),
            Token::int(5),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn closures() {
        let input = "
            let closure = fn(a, b) {
                let c = a + b;
                return fn(d) {
                    return c + d;
                };
            };

            let closure2 = fn() {
                fn test() {

                }
                return test;
            };
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::new_let(),
            Token::identifier("closure".into()),
            Token::assign_sign(),
            Token::function(),
            Token::left_paren(),
            Token::identifier("a".into()),
            Token::comma(),
            Token::identifier("b".into()),
            Token::right_paren(),
            Token::left_brace(),
            Token::new_let(),
            Token::identifier("c".into()),
            Token::assign_sign(),
            Token::identifier("a".into()),
            Token::new(TokenType::PlusSign, "+".into()),
            Token::identifier("b".into()),
            Token::semicolon(),
            Token::new(TokenType::Return, "return".into()),
            Token::function(),
            Token::left_paren(),
            Token::identifier("d".into()),
            Token::right_paren(),
            Token::left_brace(),
            Token::new(TokenType::Return, "return".into()),
            Token::identifier("c".into()),
            Token::new(TokenType::PlusSign, "+".into()),
            Token::identifier("d".into()),
            Token::semicolon(),
            Token::right_brace(),
            Token::semicolon(),
            Token::right_brace(),
            Token::semicolon(),
            Token::new_let(),
            Token::identifier("closure2".into()),
            Token::assign_sign(),
            Token::function(),
            Token::left_paren(),
            Token::right_paren(),
            Token::left_brace(),
            Token::function(),
            Token::identifier("test".into()),
            Token::left_paren(),
            Token::right_paren(),
            Token::left_brace(),
            Token::right_brace(),
            Token::new(TokenType::Return, "return".into()),
            Token::identifier("test".into()),
            Token::semicolon(),
            Token::right_brace(),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }

    #[test]
    fn while_statements() {
        let input = "
            while (true) {
                let a = 0;
            }
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::while_token(),
            Token::left_paren(),
            Token::boolean(true),
            Token::right_paren(),
            Token::left_brace(),
            Token::new_let(),
            Token::identifier("a".into()),
            Token::assign_sign(),
            Token::int(0),
            Token::semicolon(),
            Token::right_brace(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }
    #[test]
    fn dot_operator() {
        let input = "
        test.interval;
        ";

        let lex = Lexer::new(input);
        let expected: Vec<Token> = vec![
            Token::identifier("test".into()),
            Token::dot(),
            Token::identifier("interval".into()),
            Token::semicolon(),
            Token::eof(),
        ];
        let result = run_tokenizer(lex);

        assert_eq!(expected, result)
    }
}
