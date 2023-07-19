use super::{
    ast::{ASTExpression, ASTStatement},
    lexer::{self, Lexer},
    token::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn from_input(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = vec![];

        while let Some(token) = lexer.next_token() {
            tokens.push(token)
        }

        Parser { tokens, current: 0 }
    }

    pub fn next_statement(&self) -> Option<ASTStatement> {
        self.parse_statement()
    }

    fn parse_statement(&self) -> Option<ASTStatement> {
        let current_token = self.tokens.get(self.current);
        let expression = self.parse_expression();
        Some(ASTStatement::expression(expression))
        // todo!("parse_statement")
    }

    fn parse_expression(&self) -> Option<ASTExpression> {
        todo!("parse_expression")
    }
}
