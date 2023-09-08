use std::collections::VecDeque;

use crate::lex::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: VecDeque<Token>,
    _errors: Vec<String>,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lex = Lexer::new(input);
        let mut tokens: VecDeque<Token> = VecDeque::new();

        while let Some(token) = lex.next_token() {
            match token.kind {
                TokenType::Whitespace => {}
                _ => tokens.push_back(token),
            }
        }

        let current_token = tokens
            .pop_front()
            .expect("Input did not produce any token.")
            .clone();
        let next_token = tokens.pop_front().expect("Expected at least EOF.").clone();

        Self {
            tokens,
            _errors: vec![],
            current_token,
            next_token,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = 5;
        let y = 100;
        let foobar = y;
        let barfoo = false;
        let myString = \"My string\";
        ";
        let expected = [
            "let x 5",
            "let y 100",
            "let foobar y",
            "let barfoo false",
            "let myString My string",
        ];

        let mut p = Parser::new(input);

        // let result = p.build_ast();

        // for (i, curr) in result.statements.iter().enumerate() {
        //     let l = curr.as_any().downcast_ref::<LetStatement>().unwrap();
        //     assert_eq!(l.string(), expected.get(i).unwrap().to_string());
        // }
    }
}
