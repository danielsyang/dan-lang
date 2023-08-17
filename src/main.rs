use lexer::{
    lexer::Lexer,
    token::{Token, TokenType},
};

mod ast;
mod lexer;

fn main() {
    // let input = "let x = 55 + 5;";
    let input = "
    if (5 < 10) {
        return true;
    } else if (1 != 1) {
        return false;
    } else if (1 == 2) {
        return false;
    } else {
        return 1;
    }
    ";

    let mut lex = Lexer::new(input);

    let mut tokens: Vec<Token> = vec![];

    while let Some(t) = lex.next_token() {
        match t.kind {
            TokenType::Whitespace => {}
            _ => tokens.push(t),
        }
    }

    println!("{:?}", tokens)
}
