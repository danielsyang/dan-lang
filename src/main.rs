use ast::parser::Parser;
use lex::token::TokenType;

use crate::ast::tree::Statement;

mod ast;
mod lex;

fn main() {
    // let input = "let x = 55 + 5;";
    let input = "
    let x = 5;
    let y = 100;
    ";

    let mut p = Parser::new(input);
    println!("{:?}", p.tokens);
    let mut result: Vec<Box<dyn Statement>> = vec![];
    loop {
        let parsed = p.parse_program();
        result.push(parsed);

        if p.next_token.kind == TokenType::Eof {
            break;
        }
        p.consume_token();
    }
}
