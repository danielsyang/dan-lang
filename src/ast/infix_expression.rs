use std::fmt::Debug;

use crate::lexer::token::Token;

use super::tree::{Expression, Node};

pub struct InfixExpression {
    token: Token,
    left: Box<dyn Expression>,
    operator: String,
    right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(
        token: &Token,
        left_expression: Box<dyn Expression>,
        right_expression: Box<dyn Expression>,
    ) -> Self {
        Self {
            token: token.clone(),
            operator: token.literal.clone(),
            left: left_expression,
            right: right_expression,
        }
    }
}

impl Node for InfixExpression {
    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

impl Debug for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}
