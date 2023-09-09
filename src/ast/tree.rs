type Identifier = String;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
}
