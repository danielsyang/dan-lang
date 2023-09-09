use std::collections::VecDeque;

use crate::{
    eval::program::Program,
    lex::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
};

use super::{
    expression::{Expression, Operator, Prefix},
    literal::Literal,
    statement::{Block, Identifier, Statement},
};

#[derive(Clone, Copy, Debug)]
enum Precedence {
    _Int = 0,
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    _errors: Vec<String>,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lex = Lexer::new(input);
        let mut tokens: VecDeque<Token> = VecDeque::new();

        while let Some(token) = lex.next_token() {
            match token.kind {
                TokenType::Whitespace => {}
                _ => tokens.push_back(token),
            }
        }

        let current_token = tokens
            .pop_front()
            .expect("Input did not produce any token.")
            .clone();
        let next_token = tokens.pop_front().expect("Expected at least EOF.").clone();

        Self {
            tokens,
            _errors: vec![],
            current_token,
            next_token,
        }
    }

    fn consume_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self
            .tokens
            .pop_front()
            .expect("Invalid state, there are no more tokens to consume.");
    }

    fn expect_next_token(&mut self, kind: TokenType) -> bool {
        if self.next_token.kind == kind {
            self.consume_token();
            return true;
        }
        false
    }

    pub fn build_ast(&mut self) -> Program {
        let mut result: Vec<Statement> = vec![];

        loop {
            let parsed = self.parse_statement();
            result.push(parsed);

            if self.next_token.kind == TokenType::Eof {
                break;
            }
            self.consume_token();
        }

        Program { statements: result }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.kind {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Statement {
        if !self.expect_next_token(TokenType::Identifier) {
            panic!(
                "Expected next token to be TokenType::Identifier, got: {:?}",
                self.next_token.kind
            )
        }

        let identifier = self.current_token.literal.clone();

        if !self.expect_next_token(TokenType::Asssign) {
            panic!(
                "Expected next token to be TokenType::Assign, got {:?}",
                self.next_token.kind
            )
        }

        self.consume_token();

        let val = self.parse_expression(Precedence::Lowest);

        if !self.expect_next_token(TokenType::Semicolon) {
            panic!(
                "Expected next token to be TokenType::Semicolon, got {:?}",
                self.next_token.kind
            )
        }

        Statement::Let(identifier, val)
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.consume_token();

        let return_val = self.parse_expression(Precedence::Lowest);

        if self.next_token.kind == TokenType::Semicolon {
            self.consume_token();
        }

        Statement::Return(return_val)
    }

    fn parse_expression(&mut self, p: Precedence) -> Expression {
        let mut left_exp = match &self.current_token.kind {
            TokenType::Int(v) => Expression::Literal(Literal::Number(*v)),
            TokenType::Identifier => Expression::Identifier(self.current_token.literal.clone()),
            TokenType::String(s) => Expression::Literal(Literal::String(s.clone())),
            TokenType::Boolean(b) => Expression::Literal(Literal::Boolean(*b)),
            TokenType::BangSign => self.parse_prefix_expression(Prefix::Bang),
            TokenType::MinusSign => self.parse_prefix_expression(Prefix::Minus),
            TokenType::LeftParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_function_expression(),
            _ => panic!(
                "parse_expression: not yet implemented, got {:?}",
                self.current_token.kind
            ),
        };

        while (p as u8) < self.next_precedence() && self.next_token.kind != TokenType::Semicolon {
            left_exp = match self.next_token.kind {
                TokenType::PlusSign => self.parse_infix_expression(left_exp, Operator::Plus),
                TokenType::MinusSign => self.parse_infix_expression(left_exp, Operator::Minus),
                TokenType::MultiplicationSign => {
                    self.parse_infix_expression(left_exp, Operator::Multiply)
                }
                TokenType::SlashSign => self.parse_infix_expression(left_exp, Operator::Divide),
                TokenType::Eq => self.parse_infix_expression(left_exp, Operator::Equal),
                TokenType::NotEq => self.parse_infix_expression(left_exp, Operator::NotEqual),
                TokenType::LT => self.parse_infix_expression(left_exp, Operator::LessThan),
                TokenType::GT => self.parse_infix_expression(left_exp, Operator::GreaterThan),
                TokenType::LeftParen => self.parse_call_expression(left_exp),
                _ => left_exp,
            };
        }

        left_exp
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let exp = self.parse_expression(Precedence::Lowest);

        if self.next_token.kind == TokenType::Semicolon {
            self.consume_token();
        }

        Statement::Expression(exp)
    }

    fn parse_infix_expression(&mut self, left: Expression, op: Operator) -> Expression {
        self.consume_token();
        let precedence = self.current_precedence();
        self.consume_token();

        let right_expression = self.parse_expression(precedence);

        Expression::Infix(op, Box::new(left), Box::new(right_expression))
    }

    fn parse_prefix_expression(&mut self, pr: Prefix) -> Expression {
        self.consume_token();

        let expr = self.parse_expression(Precedence::Prefix);

        Expression::Prefix(pr, Box::new(expr))
    }

    fn parse_grouped_expression(&mut self) -> Expression {
        self.consume_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if !self.expect_next_token(TokenType::RightParen) {
            panic!("unexpected next token: TokenType::RightParen")
        }

        exp
    }

    fn parse_if_expression(&mut self) -> Expression {
        if !self.expect_next_token(TokenType::LeftParen) {
            panic!(
                "expected token: TokenType::LeftParen, got: {:?}",
                self.next_token.kind
            )
        }

        self.consume_token();

        let condition = self.parse_expression(Precedence::Lowest);

        if !self.expect_next_token(TokenType::RightParen) {
            panic!(
                "expected token: TokenType::RightParen, got: {:?}",
                self.next_token.kind
            )
        }

        if !self.expect_next_token(TokenType::LeftBrace) {
            panic!(
                "expected token: TokenType::LeftBrace, got: {:?}",
                self.next_token.kind
            )
        }

        let consequence = self.parse_block_statement();

        let mut alternative: Option<Block> = None;

        if self.next_token.kind == TokenType::Else {
            self.consume_token();

            if !self.expect_next_token(TokenType::LeftBrace) {
                panic!(
                    "else: expected token: TokenType::LeftBrace, got {:?}",
                    self.next_token.kind
                )
            }

            alternative = Some(self.parse_block_statement());
        }

        Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        }
    }

    fn parse_block_statement(&mut self) -> Block {
        let mut statements: Vec<Statement> = vec![];

        self.consume_token();

        while self.current_token.kind != TokenType::RightBrace
            && self.current_token.kind != TokenType::Eof
        {
            statements.push(self.parse_statement());
            self.consume_token();
        }

        statements
    }

    fn parse_function_expression(&mut self) -> Expression {
        if !self.expect_next_token(TokenType::Identifier) {
            panic!(
                "expected TokenType::Identifier, got {:?}",
                self.next_token.kind
            )
        }

        let identifier = self.current_token.literal.clone();

        if !self.expect_next_token(TokenType::LeftParen) {
            panic!(
                "expected TokenType::LeftParen, got {:?}",
                self.next_token.kind
            )
        }

        let params = self.parse_function_parameters();

        if !self.expect_next_token(TokenType::LeftBrace) {
            panic!(
                "expected TokenType::LeftParen, got {:?}",
                self.next_token.kind
            )
        }

        let body = self.parse_block_statement();

        Expression::Function {
            identifier,
            parameters: params,
            body,
        }
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers: Vec<Identifier> = vec![];

        if self.next_token.kind == TokenType::RightParen {
            self.consume_token();
            return identifiers;
        }

        self.consume_token();

        identifiers.push(self.current_token.literal.clone());

        while self.next_token.kind == TokenType::Comma {
            self.consume_token();
            self.consume_token();
            identifiers.push(self.current_token.literal.clone());
        }

        if !self.expect_next_token(TokenType::RightParen) {
            panic!(
                "expected TokenType::RightParen, got {:?}",
                self.next_token.kind
            )
        }

        identifiers
    }

    fn parse_call_expression(&mut self, function: Expression) -> Expression {
        self.consume_token();
        let args = self.parse_call_arguments();

        Expression::Call {
            function: Box::new(function),
            arguments: args,
        }
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args: Vec<Expression> = vec![];

        if self.next_token.kind == TokenType::RightParen {
            self.consume_token();
            return args;
        }

        self.consume_token();
        args.push(self.parse_expression(Precedence::Lowest));

        while self.next_token.kind == TokenType::Comma {
            self.consume_token();
            self.consume_token();
            args.push(self.parse_expression(Precedence::Lowest));
        }

        if !self.expect_next_token(TokenType::RightParen) {
            panic!(
                "parse_call_arguments expected TokenType::RightParen, got {:?}",
                self.next_token.kind
            )
        }

        args
    }

    fn current_precedence(&self) -> Precedence {
        match self.current_token.kind {
            TokenType::Eq => Precedence::Equals,
            TokenType::NotEq => Precedence::Equals,
            TokenType::LT => Precedence::LessGreater,
            TokenType::GT => Precedence::LessGreater,
            TokenType::PlusSign => Precedence::Sum,
            TokenType::MinusSign => Precedence::Sum,
            TokenType::SlashSign => Precedence::Product,
            TokenType::MultiplicationSign => Precedence::Product,
            TokenType::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn next_precedence(&self) -> u8 {
        match self.next_token.kind {
            TokenType::Eq => Precedence::Equals as u8,
            TokenType::NotEq => Precedence::Equals as u8,
            TokenType::LT => Precedence::LessGreater as u8,
            TokenType::GT => Precedence::LessGreater as u8,
            TokenType::PlusSign => Precedence::Sum as u8,
            TokenType::MinusSign => Precedence::Sum as u8,
            TokenType::SlashSign => Precedence::Product as u8,
            TokenType::MultiplicationSign => Precedence::Product as u8,
            TokenType::LeftParen => Precedence::Call as u8,
            _ => Precedence::Lowest as u8,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = 5;
        let y = 100;
        let foobar = y;
        let barfoo = false;
        let myString = \"My string\";
        ";
        let expected = [
            "Let x Number (5)",
            "Let y Number (100)",
            "Let foobar Ident (y)",
            "Let barfoo Bool (false)",
            "Let myString String (My string)",
        ];

        let mut p = Parser::new(input);

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_return_statement() {
        let input = "
        return 5;
        return 100;
        return foobar + 2;
        ";

        let mut p = Parser::new(input);
        let expected = [
            "Return Number (5)",
            "Return Number (100)",
            "Return + Left Ident (foobar) , Right Number (2)",
        ];
        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_prefix_expression() {
        let input = "
        !5;
        -15;
        !foobar;
        -foobar;
        !true;
        !false;
        5;
        ";

        let mut p = Parser::new(input);
        let expected = [
            "! Number (5)",
            "- Number (15)",
            "! Ident (foobar)",
            "- Ident (foobar)",
            "! Bool (true)",
            "! Bool (false)",
            "Number (5)",
        ];

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_infix_expression() {
        let input = "
        5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 > 5;
        5 < 5;
        5 == 5;
        5 != 5;
        foobar + foobar;
        bar - bar;
        bar * bar;
        true == true;
        false != true;
        5 + 5 * 5;
        -1 + 2;
        a + b * c + d / e - f;
        3 > 5 == false;
        ";

        let mut p = Parser::new(input);
        let expected = [
            "+ Left Number (5) , Right Number (5)",
            "- Left Number (5) , Right Number (5)",
            "* Left Number (5) , Right Number (5)",
            "/ Left Number (5) , Right Number (5)",
            "> Left Number (5) , Right Number (5)",
            "< Left Number (5) , Right Number (5)",
            "== Left Number (5) , Right Number (5)",
            "!= Left Number (5) , Right Number (5)",
            "+ Left Ident (foobar) , Right Ident (foobar)",
            "- Left Ident (bar) , Right Ident (bar)",
            "* Left Ident (bar) , Right Ident (bar)",
            "== Left Bool (true) , Right Bool (true)",
            "!= Left Bool (false) , Right Bool (true)",
            "+ Left Number (5) , Right * Left Number (5) , Right Number (5)",
            "+ Left - Number (1) , Right Number (2)",
            "- Left + Left + Left Ident (a) , Right * Left Ident (b) , Right Ident (c) , Right / Left Ident (d) , Right Ident (e) , Right Ident (f)",
            "== Left > Left Number (3) , Right Number (5) , Right Bool (false)",
        ];

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_grouped_expression() {
        let input = "
        1 + (2 + 3) + 4;
        (5 + 5) * 2;
        2 / (5 + 5);
        -(5 + 5);
        ";

        let mut p = Parser::new(input);
        let expected = [
            "+ Left + Left Number (1) , Right + Left Number (2) , Right Number (3) , Right Number (4)",
            "* Left + Left Number (5) , Right Number (5) , Right Number (2)",
            "/ Left Number (2) , Right + Left Number (5) , Right Number (5)",
            "- + Left Number (5) , Right Number (5)",
        ];

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_if_expression() {
        let input = "
        if (x > y) {
            return x;
        }
        ";

        let mut p = Parser::new(input);
        let expected = ["If > Left Ident (x) , Right Ident (y) { Return Ident (x) }"];

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_if_else_expression() {
        let input = "
        if (x > y) {
            return x;
        } else {
            return y;
        }
        ";

        let mut p = Parser::new(input);
        let expected =
            ["If > Left Ident (x) , Right Ident (y) { Return Ident (x) } else Return Ident (y)"];

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_function_parameters() {
        let input = "
        fn abc(x, y, w, z, a, b, c) { }
        ";

        let mut p = Parser::new(input);
        let expected = ["Fn abc ( x, y, w, z, a, b, c ) "];

        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_function_expression() {
        let input = "
        fn abc(x, y) { 
            return x;
        }

        fn xyz(a) {
            return a + 3;
        }
        ";

        let mut p = Parser::new(input);
        let expected = [
            "Fn abc ( x, y ) Return Ident (x)",
            "Fn xyz ( a ) Return + Left Ident (a) , Right Number (3)",
        ];
        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_call_expression() {
        let input = "
        add(1, 2 * 3, 4 + 5);
        multiply (1, 2);
        ";

        let mut p = Parser::new(input);
        let expected = [
            "Call Ident (add) , Number (1), * Left Number (2) , Right Number (3), + Left Number (4) , Right Number (5)",
            "Call Ident (multiply) , Number (1), Number (2)",
        ];
        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_string_expression() {
        let input = "
        \"Hello world\";
        ";

        let mut p = Parser::new(input);
        let expected = ["String (Hello world)"];
        let result = p.build_ast();

        for (i, curr) in result.statements.iter().enumerate() {
            assert_eq!(curr.to_string(), expected.get(i).unwrap().to_string());
        }
    }
}
