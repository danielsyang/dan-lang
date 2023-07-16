use crate::ast::lexer::Lexer;

pub mod ast;

fn main() {
    let input = "25 + 25 * 2".to_string();

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token)
    }

    println!("{:?}", tokens);
}
