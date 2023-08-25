use std::collections::HashMap;

use crate::{
    ast::statements::{
        identifier::Identifier, integer_literal::IntegerLiteral, return_statement::ReturnStatement,
    },
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
};

use super::{
    statements::{expression_statement::ExpressionStatement, let_statement::LetStatement},
    tree::{Expression, InfixParseFn, PrefixParseFn, Statement},
};

enum Precedence {
    Int = 0,
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

pub struct Parser {
    pub tokens: Vec<Token>,
    current_position: usize,
    errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lex = Lexer::new(input);
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = lex.next_token() {
            match token.kind {
                TokenType::Whitespace => {}
                _ => tokens.push(token),
            }
        }

        if tokens.len() == 0 {
            panic!("invalid input {}", input)
        }

        return Parser {
            tokens,
            current_position: 0,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
    }
    // Improve here
    fn run_parser(mut p: Parser) -> Vec<Box<dyn Statement>> {
        let mut statments: Vec<Box<dyn Statement>> = vec![];
        loop {
            let parsed = p.parse_program();
            statments.push(parsed);
            match p.tokens.get(p.current_position) {
                Some(val) => match val.kind {
                    TokenType::EOF => break,
                    _ => {}
                },
                _ => panic!("invalid state!!!!"),
            }
        }

        return statments;
    }

    fn consume_token(&mut self) -> Token {
        let t = self
            .tokens
            .get(self.current_position)
            .expect("invalid state, current position is greater than size of array of tokens")
            .clone();

        self.current_position += 1;

        return t;
    }

    fn expect_next_token(&self, desire_token_type: TokenType) -> bool {
        let next_token = self
            .tokens
            .get(self.current_position + 1)
            .expect("invalid state, I don't know what to do!");

        println!("next: {:?}, desire {:?}", next_token.kind, desire_token_type);

        if next_token.kind == desire_token_type {
            return true;
        }

        return false;
    }

    fn peek_next_token(&self) -> TokenType {
        self.tokens
            .get(self.current_position + 1)
            .expect("invalid state, I don't know what to do!")
            .clone()
            .kind
    }

    fn parse_let_statement(&mut self, mut curr: Token) -> LetStatement {
        if self.expect_next_token(TokenType::Identifier) {
            self.set_next_token_error(TokenType::Identifier);
        }

        let let_token = curr.clone();

        curr = self.consume_token();

        let name = Identifier::new(&curr);

        if self.expect_next_token(TokenType::Asssign) {
            self.set_next_token_error(TokenType::Asssign);
        }

        self.consume_token();
        curr = self.consume_token();

        let val = self.parse_expression(Precedence::Lowest, curr);
        let st = LetStatement::new(let_token, name, val);

        if self.expect_next_token(TokenType::Semicolon) {
            self.set_next_token_error(TokenType::Semicolon);
        }

        self.consume_token();
        return st;
    }

    fn parse_return_statement(&mut self, mut curr: Token) -> ReturnStatement {
        let return_token = curr.clone();
        curr = self.consume_token();

        println!("jeeeez: {:?}", curr);
        
        let val = self.parse_expression(Precedence::Int, curr);
        let rt = ReturnStatement::new(return_token, val);

        println!(" I CAN'T UNDERSTAND WHAT'S GOING ON");
        if self.expect_next_token(TokenType::Semicolon) {
            println!("ejeeeeezzz222: ");
            self.consume_token();
        }

        return rt;
    }

    fn parse_expression_statement(&mut self, mut curr: Token) -> ExpressionStatement {
        let exp_current = curr.clone();
        curr = self.consume_token();

        if !self.expect_next_token(TokenType::Semicolon) {
            self.set_next_token_error(TokenType::Semicolon);
        }

        let exp = self.parse_expression(Precedence::Lowest, curr);
        let exp_sttm = ExpressionStatement::new(exp_current, exp);

        self.consume_token();
        curr = self.consume_token();
        println!("I don't understand: {:?}", curr);

        return exp_sttm;
    }

    pub fn parse_program(&mut self) -> Box<dyn Statement> {
        let token = self.consume_token();

        match token.kind {
            TokenType::LET => Box::new(self.parse_let_statement(token)),
            TokenType::Return => Box::new(self.parse_return_statement(token)),
            _ => Box::new(self.parse_expression_statement(token)),
        }
    }

    fn set_next_token_error(&mut self, expected_token: TokenType) {
        let str = format!(
            "expected next token to be {:?}, got: {:?}",
            expected_token,
            self.peek_next_token()
        );
        self.errors.push(str)
    }

    fn parse_expression(&self, p: Precedence, curr: Token) -> Box<dyn Expression> {
        println!("curr {:?}", curr.kind);
        match curr.kind {
            TokenType::Int(v) => Box::new(IntegerLiteral::new(curr, v)),
            TokenType::Identifier => Box::new(Identifier::new(&curr)),
            // TokenType::Return => Box::new(x),
            _ => panic!("not yet implemented"),
        }
        // let val = self.prefix_parse_fns.get(&curr.kind);
        // match val {
        //     Some(prefix) => {
        //         let left_expr = prefix();
        //         return left_expr;
        //     }
        //     None => {
        //         todo!("WHAT THE FUCK!")
        //     }
        // }
    }

    fn register_prefix(&mut self, t: TokenType, prefix_parse_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(t, prefix_parse_fn);
    }

    fn register_infix(&mut self, t: TokenType, infix_parse_fn: InfixParseFn) {
        self.infix_parse_fns.insert(t, infix_parse_fn);
    }

    // fn populate_prefix() -> HashMap<TokenType, PrefixParseFn>{
    //     let mut hm: HashMap<TokenType, PrefixParseFn>  = HashMap::new();
    //     hm.insert(TokenType::Int(_), v);

    //     return hm;
    // }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{
            statements::{let_statement::LetStatement, return_statement},
            tree::Node,
        },
        lexer::token::TokenType,
    };

    use super::Parser;

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = 5;
        let y = 100;
        let foobar = y;
        ";
        let let_name = ["x", "y", "foobar"];
        let let_val = ["5", "100", "y"];
        let p = Parser::new(input);

        let result = Parser::run_parser(p);

        for (i, curr) in result.iter().enumerate() {
            let l = curr.as_any().downcast_ref::<LetStatement>().unwrap();
            assert_eq!(l.token.kind, TokenType::LET);
            assert_eq!(l.name.token_literal(), let_name.get(i).unwrap().to_string());
            assert_eq!(l.value.token_literal(), let_val.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn parse_return_statement() {
        let input = "
        return 5;
        return 100;
        return foobar;
        ";
        let p = Parser::new(input);
        println!("tokens: {:?}", p.tokens);
        let result = Parser::run_parser(p);
    }
}
