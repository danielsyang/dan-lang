use std::fmt::Debug;

use crate::{
    ast::tree::{Expression, Node},
    lex::token::Token,
};

use super::statement::BlockStatement;

pub struct BooleanLiteral {
    token: Token,
    _value: bool,
}

impl BooleanLiteral {
    pub fn new(token: &Token, value: bool) -> Self {
        Self {
            token: token.clone(),
            _value: value,
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
        self.token_literal()
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
    _value: i64,
}

impl IntegerLiteral {
    pub fn new(token: &Token, value: i64) -> Self {
        Self {
            token: token.clone(),
            _value: value,
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
        self.token_literal()
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
        self.value.to_string()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct IfExpression {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

impl Node for IfExpression {
    fn string(&self) -> String {
        match &self.alternative {
            Some(a) => format!(
                "if {} {} else {}",
                self.condition.string(),
                self.consequence.string(),
                a.string()
            ),
            None => format!(
                "if {} {}",
                self.condition.string(),
                self.consequence.string()
            ),
        }
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}

pub struct FunctionLiteral {
    token: Token,
    identifier: Identifier,
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(
        token: Token,
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            identifier,
            parameters,
            body,
        }
    }
}

impl Node for FunctionLiteral {
    fn string(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|x| x.string())
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "{} {} ( {} ) {}",
            self.token_literal(),
            self.identifier.string(),
            params,
            self.body.string()
        )
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

pub struct CallExpression {
    token: Token,
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            function,
            arguments,
        }
    }
}

impl Node for CallExpression {
    fn string(&self) -> String {
        let func = self.function.string();

        let args = self
            .arguments
            .iter()
            .map(|x| x.string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{} ( {} )", func, args)
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
}
