use crate::lexer::{lexer::Lexer, token::Token};

use super::tree::Program;

pub struct Parser {
    lexer: Lexer,
    curr_token: Option<Token>,
    next_token: Option<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lex = Lexer::new(input);
        let curr = lex.next_token();
        let next = lex.next_token();

        return Parser {
            lexer: lex,
            curr_token: curr,
            next_token: next,
        };
    }

    pub fn next_token(&mut self) {
        let n = self.lexer.next_token();

        self.curr_token = self.next_token.clone();
        self.next_token = n
    }

    pub fn parse_program(&self) -> Program {
        todo!("")
    }
}
