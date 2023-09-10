use std::fmt::Display;

use crate::eval::{env::Environment, eval_block, object::Object};

use super::{
    literal::Literal,
    statement::{Block, Identifier},
};

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
    pub fn eval(&self, env: &mut Environment) -> Object {
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
            Expression::Infix(op, left_exp, right_exp) => {
                let left = left_exp.eval(env);
                let right = right_exp.eval(env);

                match (op, &left, &right) {
                    (Operator::Plus, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l + r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Number(l + r),
                            _ => Object::Error(format!(
                                "Can only perform operation + on numbers, got: {} and {} ",
                                l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Number(l + r),
                            _ => Object::Error(format!(
                                "Can only perform operation + on numbers, got: {} and {} ",
                                l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Number(l + r),
                            _ => Object::Error(format!(
                                "Can only perform operation + on numbers, got: {} and {} ",
                                l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation + on numbers, got: {} and {} ",
                            &left, &right,
                        )),
                    },
                    (Operator::Minus, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l - r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Number(l - r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Number(l - r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Number(l - r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on numbers, got: {} and {} ",
                            op, &left, &right,
                        )),
                    },

                    (Operator::Multiply, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l * r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Number(l * r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Number(l * r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Number(l * r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on numbers, got: {} and {} ",
                            op, left, right,
                        )),
                    },

                    (Operator::Divide, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l / r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Number(l / r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Number(l / r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Number(l / r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on numbers, got: {} and {} ",
                            op, left, right,
                        )),
                    },

                    (Operator::GreaterThan, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l > r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Boolean(l > r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Boolean(l > r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Boolean(l > r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on numbers, got: {} and {} ",
                            op, left, right,
                        )),
                    },

                    (Operator::LessThan, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l < r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Boolean(l < r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Boolean(l < r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Boolean(l < r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on numbers, got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on numbers, got: {} and {} ",
                            op, left, right,
                        )),
                    },

                    (Operator::Equal, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l == r),
                        (Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l == r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Boolean(l == r),
                            (Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l == r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Boolean(l == r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Boolean(l == r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Boolean(l), Object::Return(r)) => match r.as_ref() {
                            Object::Boolean(r) => Object::Boolean(l == r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Boolean(r)) => match l.as_ref() {
                            Object::Boolean(l) => Object::Boolean(l == r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                            op, left, right,
                        )),
                    },
                    (Operator::NotEqual, _, _) => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l != r),
                        (Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l != r),
                        (Object::Return(l), Object::Return(r)) => match (l.as_ref(), r.as_ref()) {
                            (Object::Number(l), Object::Number(r)) => Object::Boolean(l != r),
                            (Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l != r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Number(l), Object::Return(r)) => match r.as_ref() {
                            Object::Number(r) => Object::Boolean(l != r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Number(r)) => match l.as_ref() {
                            Object::Number(l) => Object::Boolean(l != r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Boolean(l), Object::Return(r)) => match r.as_ref() {
                        Object::Boolean(r) => Object::Boolean(l != r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        (Object::Return(l), Object::Boolean(r)) => match l.as_ref() {
                            Object::Boolean(l) => Object::Boolean(l != r),
                            _ => Object::Error(format!(
                                "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                                op, l, r,
                            )),
                        },
                        _ => Object::Error(format!(
                            "Can only perform operation {} on (numbers | boolean), got: {} and {} ",
                            op, left, right,
                        )),
                    },
                }
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition_result = condition.eval(env);
                match (condition_result, alternative) {
                    (Object::Boolean(true), _) => eval_block(consequence, env),
                    (Object::Boolean(false), Some(alt)) => eval_block(alt, env),
                    (Object::Boolean(false), None) => Object::None,
                    (_, _) => panic!("condition did not evaluate to boolean"),
                }
            }
            Expression::Identifier(ident) => match env.get(ident.clone()) {
                Some(obj) => obj,
                None => Object::Error(format!("identifier not found: {}", ident)),
            },
            Expression::Function {
                identifier,
                parameters,
                body,
            } => {
                let fun = Object::Function {
                    name: identifier.clone(),
                    parameters: parameters.to_vec().clone(),
                    body: body.to_vec().clone(),
                };
                env.set(identifier.clone(), fun.clone());

                fun
            }
            Expression::Call {
                function,
                arguments,
            } => {
                let func = function.eval(env);
                let args = arguments
                    .iter()
                    .map(|arg| arg.eval(env))
                    .collect::<Vec<_>>();

                match (func, &args) {
                    (
                        Object::Function {
                            name: _name,
                            parameters,
                            body,
                        },
                        _,
                    ) => {
                        for (idx, param) in parameters.iter().enumerate() {
                            let arg = args
                                .get(idx)
                                .unwrap_or_else(|| panic!("Missing parameter: {}", idx))
                                .clone();

                            env.set(param.clone(), arg);
                        }

                        eval_block(&body, env)
                    }
                    (Object::Builtin { func }, _) => func(args),
                    (_, _) => {
                        panic!("not a valid call {} ", self)
                    }
                }
            }
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
