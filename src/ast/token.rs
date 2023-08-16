#[derive(Debug, PartialEq)]
pub enum TokenType {
    Comman,
    Semicolon,
    Illegal,
    EOF,
    Variable,
    Int(i64),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    LET,
    EqualSign,
    Whitespace,
    // math
    PlusSign,
    MinusSign,
    MultiplicationSign,
    DivisionSign,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tt: TokenType, literal: String) -> Self {
        Self { kind: tt, literal }
    }

    pub fn new_let() -> Self {
        Self {
            kind: TokenType::LET,
            literal: "let".to_string(),
        }
    }

    pub fn eof() -> Self {
        Self {
            kind: TokenType::EOF,
            literal: "\0".to_string(),
        }
    }

    pub fn whitespace() -> Self {
        Self {
            kind: TokenType::Whitespace,
            literal: " ".to_string(),
        }
    }

    pub fn equal_sign() -> Self {
        Self {
            kind: TokenType::EqualSign,
            literal: "=".to_string(),
        }
    }

    pub fn semicolon() -> Self {
        Self {
            kind: TokenType::Semicolon,
            literal: ";".to_string(),
        }
    }
}
