use crate::lexer::token::Token;

use super::tree::{Expression, Node};

pub struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(token: &Token, expression: Box<dyn Expression>) -> Self {
        Self {
            token: token.clone(),
            operator: token.literal.clone(),
            right: expression,
        }
    }
}

impl Node for PrefixExpression {
    fn string(&self) -> String {
        format!("({} {} )", self.operator, self.right.string())
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}
