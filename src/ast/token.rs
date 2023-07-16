#[derive(Debug)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    EOF,
    Whitespace,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: TokenSpan,
}

#[derive(Debug)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}
