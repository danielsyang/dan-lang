use crate::{
    ast::tree::{Expression, Node, Statement},
    lexer::token::Token,
};

pub struct IntegerLiteral {
    token: Token,
    value: i64,
}

impl IntegerLiteral {
    pub fn new(token: &Token, value: i64) -> Self {
        Self { token: token.clone(), value }
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.value.to_string())
    }
}
