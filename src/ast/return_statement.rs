use crate::{
    ast::tree::{Expression, Node, Statement},
    lexer::token::Token,
};

pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<dyn Expression>,
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
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.value.token_literal())
    }
}
