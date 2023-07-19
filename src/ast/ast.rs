pub struct AST {
    pub statment: Vec<ASTStatement>,
}

impl AST {
    pub fn new() -> Self {
        AST { statment: vec![] }
    }

    pub fn add_statement(&mut self, s: ASTStatement) {
        self.statment.push(s)
    }
}

pub struct ASTStatement {
    pub kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement {
            kind: ASTStatementKind::Expression(expr),
        }
    }
}

pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTExpression {
    pub kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn number(n: i64) -> Self {
        ASTExpression {
            kind: ASTExpressionKind::Number(n),
        }
    }
}

pub enum ASTExpressionKind {
    Number(i64),
}

// Debugging
pub struct ASTVisitor {
    pub ast: AST,
}

impl ASTVisitor {
    pub fn visit(&self) {
        self.ast.statment.iter().for_each(|sttm| match sttm.kind {
            ASTStatementKind::Expression(expr) => self.visist_expression(&expr),
            _ => todo!("ASTVisitor not working for all ASTExpression."),
        })
    }

    pub fn visist_expression(&self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::Number(number) => {
                println!("Number: {}", number)
            }
        }
    }
}
