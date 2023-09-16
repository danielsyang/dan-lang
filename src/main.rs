use std::io::{stdin, stdout, Write};

use crate::{ast::parser::Parser, eval::env::Environment};

mod ast;
mod eval;
mod lex;

fn main() {
    println!("This is the Dan-Lang programming language!");
    println!("Feel free to type in commands");
    let mut env = Environment::new();

    loop {
        print!(">> ");

        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read line");

        let program = Parser::build_ast(&buffer);

        let obj = program.eval_statements(&mut env);

        println!("{}", obj);
    }
}
