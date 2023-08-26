use crate::{
    ast::tree::{Expression, Node, Statement},
    lexer::token::Token,
};

use super::identifier::Identifier;
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn string(&self) -> String {
        format!(
            "{} {} = {}",
            self.token_literal(),
            self.name.token_literal(),
            self.value.token_literal()
        )
    }
}
