use crate::{lexer::token::Token, ast::tree::{Expression, Statement, Node}};

pub struct ReturnStatement {
    token: Token,
    value: Box<dyn Expression>
}

impl ReturnStatement {
    pub fn new(token: Token, value: Box<dyn Expression>) -> Self {
        Self { token, value }
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn string(&self) -> String {
       format!("{} {};", self.token_literal(), self.value.token_literal())
    }
}
