use crate::ast::statement::Statement;

use super::{environment::Environment, object::Object};

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn eval_statements(&self, env: &mut Environment) -> Object {
        let mut result = Object::None;
        for stmt in self.statements.iter() {
            result = stmt.eval(env);

            match result {
                Object::Return(_) => break,
                _ => {}
            }
        }

        result
    }
}
