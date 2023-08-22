use crate::{
    ast::tree::{Node, Statement},
    lexer::token::Token,
};

pub struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Statement for Identifier {
    fn statement_node(&self) {}
}

impl Identifier {
    pub fn new(token: &Token) -> Self {
        Self {
            token: token.clone(),
            value: token.literal.clone(),
        }
    }
}
