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
        ";
        let result = ["5", "10", "-10", "-5"];
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
        ";
        let result = ["true", "false"];
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
