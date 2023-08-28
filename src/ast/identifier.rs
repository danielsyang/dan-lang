use crate::{
    ast::tree::{Expression, Node},
    lexer::token::Token,
};

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    value: String,
}

impl Identifier {
    pub fn new(token: &Token) -> Self {
        Self {
            token: token.clone(),
            value: token.literal.clone(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
