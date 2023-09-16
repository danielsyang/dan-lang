use std::fmt::Display;

use crate::eval::{env::Environment, object::Object};

use super::expression::Expression;

pub type Block = Vec<Statement>;
pub type Identifier = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
    Error(String),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(identifier, exp) => write!(f, "Let {} {}", identifier, exp),
            Statement::Return(exp) => {
                write!(f, "Return {}", exp)
            }
            Statement::Expression(exp) => {
                write!(f, "{}", exp)
            }
            Statement::Error(s) => write!(f, "error: ( {} )", s),
        }
    }
}

impl Statement {
    pub fn eval(&self, env: &mut Environment) -> Object {
        match self {
            Statement::Expression(exp) => exp.eval(env),
            Statement::Return(r) => {
                let result = r.eval(env);
                Object::Return(Box::new(result))
            }
            Statement::Let(ident, exp) => {
                let val = exp.eval(env);
                env.set(ident.clone(), val.clone());

                val
            }
            Statement::Error(s) => Object::Error(s.clone()),
        }
    }
}
