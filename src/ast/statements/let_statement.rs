use crate::{
    ast::tree::{Expression, Node, Statement},
    lexer::token::Token,
};

struct LetStatement {
    token: Token,
    name: String,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}
