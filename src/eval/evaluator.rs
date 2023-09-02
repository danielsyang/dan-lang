use std::any::Any;

use crate::ast::{
    statement::ExpressionStatement,
    tree::{AToAny, Program, Statement},
};

pub fn eval(p: Program) {
    for p in p.statements.iter() {
        let l = p.as_any().downcast_ref::<ExpressionStatement>().unwrap();
        println!("outside eval statements {:?}", l);
        eval_statements(&p);
    }
}

pub fn eval_statements(node: &Box<dyn Statement>) -> () {
    println!("inside node: {:?}", node);
    // let l = node.as_any().downcast_ref::<Box<ExpressionStatement>>();
    let l = (node as &dyn Any).downcast_ref::<ExpressionStatement>();
    println!("inside eval_statements {:?}", l);
}

// print_type_of(&node);

// println!("123123 {:?}", node.expression_type());
// let aa = node
//     .as_any()
//     .downcast_ref::<Box<ExpressionStatement>>()
//     .unwrap();

// print_type_of(&aa);

// todo!(" WORKING ON THIS")

// match node.as_any().downcast_ref::<ExpressionStatement>() {
//     Some(n) => {
//         match n.as_any().downcast_ref::<IntegerLiteral>() {
//             Some(il) => Box::new(Number::new(il.value)),
//             Some(_) => {
//                 println!(" WHAT TO DO???");
//                 Box::new(Number::new(2))
//             }
//             _ => {
//                 println!(" WHAT TO DO222???");
//                 panic!("not yet implemented, got {:?}", node)
//             }
//         };
//         println!("HELLO!!! {:?} ", n.expression);
//         match n {
//             _ => Box::new(Number::new(1)),
//         };
//         Box::new(Number::new(1))
//     }
//     _ => {
//         dbg!(node);
//         panic!("12398127389123 not yet implemented, got {:?}", node)
//     }
// }

#[cfg(test)]
mod test {

    use crate::ast::parser::Parser;

    // use super::eval_statements;

    #[test]
    fn eval_integer_expression() {
        let input = "
        5;
        10;
        ";
        let mut p = Parser::new(input);
        let program = p.build_ast();

        for node in program.statements.iter() {
            let obj = node.eval_self();
            // println!("{:?}", obj);
        }
    }
}
