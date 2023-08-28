use crate::{
    ast::tree::{Expression, Node},
    lexer::token::Token,
};

pub struct BooleanLiteral {
    token: Token,
    value: bool,
}

impl BooleanLiteral {
    pub fn new(token: &Token, value: bool) -> Self {
        Self {
            token: token.clone(),
            value,
        }
    }
}

impl Expression for BooleanLiteral {
    fn expression_node(&self) {}
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{};", self.token_literal())
    }
}
