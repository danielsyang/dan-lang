use ast::parser::Parser;

use crate::{
    ast::{statement::ExpressionStatement, tree::AToAny},
    eval::evaluator::eval_statements,
};

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

    for node in program.statements.iter() {
        eval_statements(node);

        println!("ouside node {:?}", node);
        // let l = (node as &dyn Any).downcast_ref::<ExpressionStatement>().unwrap();
        let l = node.as_any().downcast_ref::<ExpressionStatement>().unwrap();
        // let ll = l.as_any().downcast_ref::<IntegerLiteral>();
        println!("outside eval statements {:?}", l);
        // println!("outside eval statements {:?}", ll);
    }
    // eval(program);
}
