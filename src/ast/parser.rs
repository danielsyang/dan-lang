use crate::{
    ast::statements::{identifier::Identifier, return_statement::ReturnStatement},
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
};

use super::{
    statements::let_statement::LetStatement,
    tree::{Expression, Statement},
};

pub struct Parser {
    tokens: Vec<Token>,
    current_position: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lex = Lexer::new(input);
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = lex.next_token() {
            match token.kind {
                TokenType::Whitespace => {}
                _ => tokens.push(token),
            }
        }

        if tokens.len() == 0 {
            panic!("invalid input {}", input)
        }

        return Parser {
            tokens,
            current_position: 0,
            errors: vec![],
        };
    }

    fn consume_token(&mut self) -> Token {
        let t = self
            .tokens
            .get(self.current_position)
            .expect("invalid state, current position is greater than size of array of tokens")
            .clone();

        self.current_position += 1;

        return t;
    }

    fn expect_next_token(&self, desire_token_type: TokenType) -> bool {
        let next_token = self
            .tokens
            .get(self.current_position + 1)
            .expect("invalid state, I don't know what to do!");

        if next_token.kind == desire_token_type {
            return true;
        }

        return false;
    }

    fn peek_next_token(&self) -> TokenType {
        self.tokens
            .get(self.current_position + 1)
            .expect("invalid state, I don't know what to do!")
            .clone()
            .kind
    }

    fn parse_let_statement(&mut self, curr: Token) -> LetStatement {
        if self.expect_next_token(TokenType::Indentifier) {
            self.set_next_token_error(TokenType::Indentifier);
        }

        let name = Identifier::new(&curr);
        let let_token = curr;

        self.consume_token();

        if self.expect_next_token(TokenType::Asssign) {
            self.set_next_token_error(TokenType::Asssign);
        }

        let val = self.parse_expression();
        let st = LetStatement::new(let_token, name, val);

        return st;
    }

    fn parse_return_statement(&self, curr: Token) -> ReturnStatement {
        let return_token = curr;

        // if self.expect_next_token() {
        //     self.set_next_token_error()
        // }
        let val = self.parse_expression();
        let rt = ReturnStatement::new(return_token, val);

        return rt;
    }

    pub fn parse_program(&mut self) -> Box<dyn Statement> {
        let token = self.consume_token();

        match token.kind {
            TokenType::LET => Box::new(self.parse_let_statement(token)),
            TokenType::Return => Box::new(self.parse_return_statement(token)),
            _ => todo!("not yet implemented"),
        }
    }

    fn set_next_token_error(&mut self, expected_token: TokenType) {
        let str = format!(
            "expected next token to be {:?}, got: {:?}",
            expected_token,
            self.peek_next_token()
        );
        self.errors.push(str)
    }

    fn parse_expression(&self) -> Box<dyn Expression> {
        todo!("todo")
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn test() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";
        let mut p = Parser::new(input);

        p.parse_program();

        // assert_eq!(1, 2)
    }
}
