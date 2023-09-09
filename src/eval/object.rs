use std::fmt::Display;

#[derive(Clone)]
pub enum Object {
    None,
    Number(i64),
    String(String),
    Boolean(bool),
    Return(Box<Object>),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::None => write!(f, "None"),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Return(r) => write!(f, "{}", r),
        }
    }
}
