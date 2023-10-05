use std::{
    fs,
    io::{stdin, stdout, Write},
};

use clap::Parser as Parser_Clap;

use crate::{ast::parser::Parser, eval::env::Environment};

mod ast;
mod eval;
mod lex;

fn read_file(file_name: &str) -> String {
    match fs::read_to_string(file_name) {
        Ok(s) => s,
        Err(_) => panic!("Cannot read file: {}", file_name),
    }
}

#[derive(Debug, Parser_Clap)]
struct Args {
    #[arg(long)]
    file_name: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut env = Environment::new();

    match args.file_name {
        Some(file_name) => {
            let input = read_file(&file_name);

            let program = Parser::build_ast(&input);
            let obj = program.eval_statements(&mut env);

            println!("{}", obj);
        }
        None => {
            println!("This is the Dan-Lang programming language!");
            println!("Feel free to type in commands");

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
    }
}
