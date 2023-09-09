use std::collections::VecDeque;

use crate::lex::{
    lexer::Lexer,
    token::{Token, TokenType},
};

use super::tree::{Expression, Literal, Program, Statement};

#[derive(Clone, Copy, Debug)]
enum Precedence {
    _Int = 0,
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

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

    fn consume_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self
            .tokens
            .pop_front()
            .expect("Invalid state, there are no more tokens to consume.");
    }

    fn expect_next_token(&mut self, kind: TokenType) -> bool {
        if self.next_token.kind == kind {
            self.consume_token();
            return true;
        }
        false
    }

    pub fn build_ast(&mut self) -> Program {
        let mut result: Vec<Statement> = vec![];

        loop {
            let parsed = match self.current_token.kind {
                TokenType::Let => self.parse_let_statement(),
                TokenType::Return => self.parse_return_statement(),
                _ => panic!(""),
                // _ => self.parse_expression_statement(),
            };
            result.push(parsed);

            if self.next_token.kind == TokenType::Eof {
                break;
            }
            self.consume_token();
        }

        Program { statements: result }
    }

    fn parse_let_statement(&mut self) -> Statement {
        if !self.expect_next_token(TokenType::Identifier) {
            panic!(
                "Expected next token to be TokenType::Identifier, got: {:?}",
                self.next_token.kind
            )
        }

        let identifier = self.current_token.literal.clone();

        if !self.expect_next_token(TokenType::Asssign) {
            panic!(
                "Expected next token to be TokenType::Assign, got {:?}",
                self.next_token.kind
            )
        }

        self.consume_token();

        let val = self.parse_expression(Precedence::Lowest);

        if !self.expect_next_token(TokenType::Semicolon) {
            panic!(
                "Expected next token to be TokenType::Semicolon, got {:?}",
                self.next_token.kind
            )
        }

        Statement::Let(identifier, val)
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.consume_token();

        let return_val = self.parse_expression(Precedence::Lowest);

        if self.next_token.kind == TokenType::Semicolon {
            self.consume_token();
        }

        Statement::Return(return_val)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        match &self.current_token.kind {
            TokenType::Int(v) => Expression::Literal(Literal::Number(*v)),
            TokenType::Identifier => Expression::Identifier(self.current_token.literal.clone()),
            TokenType::String(s) => Expression::Literal(Literal::String(s.clone())),
            TokenType::Boolean(b) => Expression::Literal(Literal::Boolean(*b)),
            _ => panic!(
                "parse_expression: not yet implemented, got {:?}",
                self.current_token.kind
            ),
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
            "Let(\"x\", Literal(Number(5)))",
            "Let(\"y\", Literal(Number(100)))",
            "Let(\"foobar\", Identifier(\"y\"))",
            "Let(\"barfoo\", Literal(Boolean(false)))",
            "Let(\"myString\", Literal(String(\"My string\")))",
        ];

        let mut p = Parser::new(input);

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(format!("{:?}", curr), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_return_statement() {
        let input = "
        return 5;
        return 100;
        return foobar + 2;
        ";

        let mut p = Parser::new(input);
        let expected = ["return 5", "return 100", "return (+ foobar Int(2) 2)"];
        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(format!("{:?}", curr), expected.get(i).unwrap().to_string());
        }
    }
}
