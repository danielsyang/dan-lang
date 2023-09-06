use std::io::{stdin, stdout, Write};

use crate::{ast::parser::Parser, eval::environment::Environment};

mod ast;
mod eval;
mod lex;

fn main() {
    println!("This is the Dan-Lang programming language!");
    println!("Feel free to type in commands");

    loop {
        let env = Environment::new();
        print!(">> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read line");

        let mut p = Parser::new(buffer.as_str());
        let program = p.build_ast();

        let obj = program.eval_statements(&env);

        println!("{}", obj.inspect());
    }
}
