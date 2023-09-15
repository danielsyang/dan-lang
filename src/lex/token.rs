use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum TokenType {
    Comma,
    Semicolon,
    Colon,
    Eof,
    Identifier,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // keywords
    Let,
    Function,
    If,
    Else,
    Return,

    // literals
    Boolean(bool),
    String(String),
    Int(i64),

    // whitespace is a generic term that represents ' ', or '\n', or '\r'
    Whitespace,

    // math
    PlusSign,
    MinusSign,
    MultiplicationSign,
    SlashSign,
    Asssign,
    // -> !
    BangSign,
    LT,
    GT,
    Eq,
    NotEq,
}

#[derive(Debug, PartialEq, Clone)]
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
            kind: TokenType::Let,
            literal: "let".to_string(),
        }
    }

    pub fn eof() -> Self {
        Self {
            kind: TokenType::Eof,
            literal: "\0".to_string(),
        }
    }

    pub fn whitespace() -> Self {
        Self {
            kind: TokenType::Whitespace,
            literal: " ".to_string(),
        }
    }

    pub fn assign_sign() -> Self {
        Self {
            kind: TokenType::Asssign,
            literal: "=".to_string(),
        }
    }

    pub fn semicolon() -> Self {
        Self {
            kind: TokenType::Semicolon,
            literal: ";".to_string(),
        }
    }

    pub fn left_paren() -> Self {
        Self {
            kind: TokenType::LeftParen,
            literal: "(".to_string(),
        }
    }

    pub fn right_paren() -> Self {
        Self {
            kind: TokenType::RightParen,
            literal: ")".to_string(),
        }
    }

    pub fn left_brace() -> Self {
        Self {
            kind: TokenType::LeftBrace,
            literal: "{".to_string(),
        }
    }

    pub fn right_brace() -> Self {
        Self {
            kind: TokenType::RightBrace,
            literal: "}".to_string(),
        }
    }

    pub fn left_bracket() -> Self {
        Self {
            kind: TokenType::LeftBracket,
            literal: "[".to_string(),
        }
    }

    pub fn right_bracket() -> Self {
        Self {
            kind: TokenType::RightBracket,
            literal: "]".to_string(),
        }
    }

    pub fn function() -> Self {
        Self {
            kind: TokenType::Function,
            literal: "fn".to_string(),
        }
    }

    pub fn comma() -> Self {
        Self {
            kind: TokenType::Comma,
            literal: ",".to_string(),
        }
    }

    pub fn bang() -> Self {
        Self {
            kind: TokenType::BangSign,
            literal: "!".to_string(),
        }
    }
    pub fn lt() -> Self {
        Self {
            kind: TokenType::LT,
            literal: "<".to_string(),
        }
    }
    pub fn gt() -> Self {
        Self {
            kind: TokenType::GT,
            literal: ">".to_string(),
        }
    }

    pub fn int(n: i64) -> Self {
        Self {
            kind: TokenType::Int(n),
            literal: n.to_string(),
        }
    }

    pub fn identifier(name: String) -> Self {
        Self {
            kind: TokenType::Identifier,
            literal: name,
        }
    }

    pub fn string(string: String) -> Self {
        Self {
            kind: TokenType::String(string.clone()),
            literal: string,
        }
    }

    pub fn boolean(b: bool) -> Self {
        Self {
            kind: TokenType::Boolean(b),
            literal: b.to_string(),
        }
    }

    pub fn colon() -> Self {
        Self {
            kind: TokenType::Colon,
            literal: ":".to_string(),
        }
    }
}
