use crate::eval::object::Object;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
}

impl Literal {
    pub fn eval(&self) -> Object {
        match self {
            Literal::Number(n) => Object::Number(*n),
            Literal::Boolean(b) => Object::Boolean(*b),
            Literal::String(s) => Object::String(s.clone()),
        }
    }
}
