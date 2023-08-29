use std::fmt::Debug;

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

pub struct IntegerLiteral {
    token: Token,
    value: i64,
}

impl IntegerLiteral {
    pub fn new(token: &Token, value: i64) -> Self {
        Self {
            token: token.clone(),
            value,
        }
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{};", self.token_literal())
    }
}

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
        format!("({} {})", self.operator, self.right.string())
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

impl Debug for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

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
        format!("{};", self.value)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
