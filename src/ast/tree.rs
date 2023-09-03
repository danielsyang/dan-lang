use std::{any::Any, fmt::Debug};

use crate::eval::object::Object;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;

    fn eval_node(&self) -> Box<dyn Object>;
}

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

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
