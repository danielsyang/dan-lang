use self::statement::Statement;

pub mod expression;
pub mod literal;
pub mod parser;
pub mod statement;

pub type Identifier = String;
pub type Block = Vec<Statement>;
