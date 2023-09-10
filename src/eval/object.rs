use std::fmt::Display;

use crate::ast::statement::{Block, Identifier};

type BuiltinFunction = fn(Vec<Object>) -> Object;
type Elements = Vec<Object>;

#[derive(Debug, Clone)]
pub enum Object {
    None,
    Number(i64),
    String(String),
    Boolean(bool),
    Return(Box<Object>),
    Error(String),
    Function {
        name: Identifier,
        parameters: Vec<Identifier>,
        body: Block,
    },
    Array(Elements),
    Builtin {
        func: BuiltinFunction,
    },
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::None => write!(f, "None"),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Return(r) => write!(f, "{}", r),
            Object::Error(s) => write!(f, "error: {}", s),
            Object::Function {
                name,
                parameters,
                body,
            } => {
                write!(
                    f,
                    "Fn {} ( {} ) {{ {} }}",
                    name,
                    parameters
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    body.iter()
                        .map(|sttm| sttm.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Object::Builtin { func: _ } => write!(f, ""),
            Object::Array(elements) => write!(
                f,
                "[ {} ]",
                elements
                    .iter()
                    .map(|elem| elem.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
