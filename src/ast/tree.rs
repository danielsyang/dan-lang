pub struct Program {
    statements: Vec<Statement>,
}

enum Statement {
    Let,
    Return,
    IfElse,
}
