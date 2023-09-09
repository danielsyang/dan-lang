use std::fmt::Display;

use crate::eval::{environment::Environment, object::Object};

use super::{literal::Literal, Block, Identifier};

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessThan => write!(f, "<"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Prefix {
    Bang,
    Minus,
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Bang => write!(f, "!"),
            Prefix::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    Infix(Operator, Box<Expression>, Box<Expression>),
    Prefix(Prefix, Box<Expression>),
    If {
        condition: Box<Expression>,
        consequence: Block,
        alternative: Option<Block>,
    },
    Function {
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: Block,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl Expression {
    pub fn eval(&self, env: &Environment) -> Object {
        match self {
            Expression::Literal(l) => l.eval(),
            Expression::Prefix(op, exp) => {
                let right_exp = exp.eval(env);

                match op {
                    Prefix::Bang => match right_exp {
                        Object::Boolean(b) => Object::Boolean(!b),
                        _ => panic!("expected Boolean, got: {}", right_exp),
                    },
                    Prefix::Minus => match right_exp {
                        Object::Number(n) => Object::Number(-n),
                        _ => panic!("expected Number, got: {}", right_exp),
                    },
                }
            }
            _ => panic!("WHAT"),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(Literal::Number(v)) => write!(f, "Number ({})", v),
            Expression::Literal(Literal::String(s)) => write!(f, "String ({})", s),
            Expression::Literal(Literal::Boolean(b)) => write!(f, "Bool ({})", b),
            Expression::Identifier(i) => write!(f, "Ident ({})", i),
            Expression::Infix(op, left, right) => {
                write!(f, "{} Left {} , Right {}", op, left, right)
            }
            Expression::Prefix(pr, exp) => write!(f, "{} {}", pr, exp),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let consequence_block = consequence
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                match alternative {
                    Some(alt) => {
                        let alt_block = alt
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        write!(
                            f,
                            "If {} {{ {} }} else {}",
                            condition, consequence_block, alt_block
                        )
                    }
                    None => {
                        write!(f, "If {} {{ {} }}", condition, consequence_block)
                    }
                }
            }
            Expression::Function {
                identifier,
                parameters,
                body,
            } => write!(
                f,
                "Fn {} ( {} ) {}",
                identifier,
                parameters.join(", "),
                body.iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            Expression::Call {
                function,
                arguments,
            } => write!(
                f,
                "Call {} , {}",
                function,
                arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
