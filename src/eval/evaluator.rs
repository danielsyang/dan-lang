use crate::lex::token::{Token, TokenType};

use super::object::{Boolean, Number, Object, BOOLEAN_OBJ, NUMBER_OBJ};

pub fn eval_infix_expression(
    left: Box<dyn Object>,
    right: Box<dyn Object>,
    operator: &Token,
) -> Box<dyn Object> {
    match (left.kind(), right.kind(), operator.kind) {
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::PlusSign) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Number::new(ln + rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::MinusSign) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Number::new(ln - rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::MultiplicationSign) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Number::new(ln * rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::SlashSign) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Number::new(ln / rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::Eq) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Boolean::new(ln == rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::NotEq) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Boolean::new(ln != rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::GT) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Boolean::new(ln > rn))
        }
        (NUMBER_OBJ, NUMBER_OBJ, TokenType::LT) => {
            let ln = left.inspect().parse::<i64>().unwrap();
            let rn = right.inspect().parse::<i64>().unwrap();
            Box::new(Boolean::new(ln < rn))
        }
        (BOOLEAN_OBJ, BOOLEAN_OBJ, TokenType::Eq) => {
            let ln = left.inspect().parse::<bool>().unwrap();
            let rn = right.inspect().parse::<bool>().unwrap();
            Box::new(Boolean::new(ln == rn))
        }
        (BOOLEAN_OBJ, BOOLEAN_OBJ, TokenType::NotEq) => {
            let ln = left.inspect().parse::<bool>().unwrap();
            let rn = right.inspect().parse::<bool>().unwrap();
            Box::new(Boolean::new(ln != rn))
        }
        (_, _, _) => panic!("eval_infix_expression: operation not yet implemented"),
    }
}

#[cfg(test)]
mod test {
    use crate::ast::parser::Parser;

    #[test]
    fn eval_integer_expression() {
        let input = "
        5;
        10;
        -10;
        -5;
        5 + 5 + 5 + 5 - 10;
        2 * 2 * 2 * 2 * 2;
        50 / 2 * 2 + 10;
        3 * (3 * 3) + 10;
        (5 + 10 * 2 + 15 / 3) * 2 + -10;
        ";
        let result = ["5", "10", "-10", "-5", "10", "32", "60", "37", "50"];
        let mut p = Parser::new(input);
        let program = p.build_ast();

        for (i, node) in program.statements.iter().enumerate() {
            let obj = node.eval_node();

            assert_eq!(obj.inspect(), result.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn eval_boolean_expression() {
        let input = "
        true;
        false;
        1 < 2;
        1 > 2;
        1 == 2;
        1 != 2;
        true == true;
        true != true;
        1 + 2 == 3;
        1 + 2 == 2 + 1;
        ";
        let result = ["true", "false", "true", "false", "false", "true", "true", "false", "true", "true"];
        let mut p = Parser::new(input);
        let program = p.build_ast();

        for (i, node) in program.statements.iter().enumerate() {
            let obj = node.eval_node();

            assert_eq!(obj.inspect(), result.get(i).unwrap().to_string());
        }
    }

    #[test]
    fn eval_bang_expression() {
        let input = "
        !true;
        !false;
        !!true;
        !!false;
        ";
        let result = ["false", "true", "true", "false"];
        let mut p = Parser::new(input);
        let program = p.build_ast();

        for (i, node) in program.statements.iter().enumerate() {
            let obj = node.eval_node();

            assert_eq!(obj.inspect(), result.get(i).unwrap().to_string());
        }
    }
}
