use std::fmt::Display;

pub type Identifier = String;
pub type Block = Vec<Statement>;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(identifier, exp) => write!(f, "Let {} {}", identifier, exp.to_string()),
            Statement::Return(exp) => {
                write!(f, "Return {}", exp.to_string())
            }
            Statement::Expression(exp) => {
                write!(f, "{}", exp.to_string())
            }
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
        arguments: Vec<Box<Expression>>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(Literal::Number(v)) => write!(f, "Number ({})", v),
            Expression::Literal(Literal::String(s)) => write!(f, "String ({})", s),
            Expression::Literal(Literal::Boolean(b)) => write!(f, "Bool ({})", b),
            Expression::Identifier(i) => write!(f, "Ident ({})", i),
            Expression::Infix(op, left, right) => write!(
                f,
                "{} Left {} , Right {}",
                op.to_string(),
                left.to_string(),
                right.to_string()
            ),
            Expression::Prefix(pr, exp) => write!(f, "{} Exp {}", pr.to_string(), exp.to_string()),
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
                            condition.to_string(),
                            consequence_block,
                            alt_block
                        )
                    }
                    None => {
                        write!(
                            f,
                            "If {} {{ {} }}",
                            condition.to_string(),
                            consequence_block
                        )
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
                function.to_string(),
                arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
}

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
