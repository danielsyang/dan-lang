use ast::parser::Parser;
use eval::evaluator::eval;

mod ast;
mod eval;
mod lex;

fn main() {
    // // let input = "let x = 55 + 5;";
    // let input = "
    // let x = 5;
    // let y = 100;
    // ";

    // let mut p = Parser::new(input);
    // let _result = p.build_ast();

    let input = "
    5;
    10;
    ";
    let mut p = Parser::new(input);
    let program = p.build_ast();
    eval(program);
}
