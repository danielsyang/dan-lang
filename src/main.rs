use ast::{
    lexer::Lexer,
    token::{Token, TokenType},
};

mod ast;

fn main() {
    let input = "let x = 5 + 5;";

    let mut lex = Lexer::new(input);
    let result = vec![
        Token::new_let(),
        Token::new(TokenType::Variable, "x".to_string()),
        Token::equal_sign(),
        Token::new(TokenType::Int(5), "5".to_string()),
        Token::new(TokenType::PlusSign, "+".to_string()),
        Token::new(TokenType::Int(5), "5".to_string()),
        Token::semicolon(),
    ];

    let mut tokens: Vec<Token> = vec![];

    while let Some(t) = lex.next_token() {
        match t.kind {
            TokenType::Whitespace => {}
            _ => tokens.push(t),
        }
    }

    assert_eq!(tokens, result)
}
