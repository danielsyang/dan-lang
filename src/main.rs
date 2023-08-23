use ast::parser::Parser;
use lexer::{
    lexer::Lexer,
    token::{Token, TokenType},
};

mod ast;
mod lexer;

fn main() {
    // let input = "let x = 55 + 5;";
    let input = "
    let x = 5;
    let y = 100;
    ";

    let mut p = Parser::new(input);
    println!("{:?}", p.tokens);
}
