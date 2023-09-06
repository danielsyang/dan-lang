use std::fmt::Debug;

use crate::{
    ast::tree::{Expression, Node, Statement},
    eval::{
        environment::Environment,
        object::{None, Object, Return, RETURN_OBJ},
    },
    lex::token::Token,
};

use super::expression::Identifier;

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
        format!("{} {}", self.token_literal(), self.value.token_literal())
    }

    fn eval_node(&self, env: &Environment) -> Box<dyn Object> {
        let v = self.value.eval_expression(env);
        Box::new(Return::new(v))
    }
}

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

impl Debug for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            self.token.literal,
            self.name.string(),
            self.value.string()
        )
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "{} {} {}",
            self.token_literal(),
            self.name.token_literal(),
            self.value.token_literal()
        )
    }

    fn eval_node(&self, _env: &Environment) -> Box<dyn Object> {
        todo!("eval_node: LetStatement")
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{:?}", self.expression)
    }

    fn eval_node(&self, env: &Environment) -> Box<dyn Object> {
        self.expression.eval_expression(env)
    }
}

impl Debug for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token.kind, self.token_literal())
    }
}

pub struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> Self {
        Self { token, statements }
    }
}

impl Node for BlockStatement {
    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn eval_node(&self, env: &Environment) -> Box<dyn Object> {
        let mut result: Box<dyn Object> = Box::new(None::new());
        for stmt in &self.statements {
            result = stmt.eval_node(env);

            if result.kind() == RETURN_OBJ {
                break;
            }
        }

        result
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}

impl Debug for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}
