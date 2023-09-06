use std::{any::Any, fmt::Debug};

use crate::eval::object::{None, Object, RETURN_OBJ};

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;

    fn eval_node(&self) -> Box<dyn Object>;
}
// TOOD: Remove downcasting
pub trait Statement: Node + AToAny {
    fn statement_node(&self);
}

impl Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

pub trait Expression: Node {
    fn expression_node(&self);

    fn eval_expression(&self) -> Box<dyn Object>;
}

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn eval_statements(&self) -> Box<dyn Object> {
        let mut result: Box<dyn Object> = Box::new(None::new());
        for stmt in self.statements.iter() {
            result = stmt.eval_node();

            if result.kind() == RETURN_OBJ {
                break;
            }
        }

        result
    }
}

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
