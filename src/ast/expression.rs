use std::fmt::Debug;

use crate::{
    ast::tree::{Expression, Node},
    eval::{
        environment::Environment,
        evaluator::eval_infix_expression,
        object::{Boolean, Function, None, Number, Object, BOOLEAN_OBJ, NUMBER_OBJ},
    },
    lex::token::Token,
};

use super::statement::BlockStatement;

pub struct BooleanLiteral {
    token: Token,
    value: bool,
}

impl BooleanLiteral {
    pub fn new(token: &Token, value: bool) -> Self {
        Self {
            token: token.clone(),
            value,
        }
    }
}

impl Expression for BooleanLiteral {
    fn expression_node(&self) {}

    fn eval_expression(&self, _env: &mut Environment) -> Box<dyn Object> {
        Box::new(Boolean::new(self.value))
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        Box::new(BooleanLiteral::new(&self.token, self.value))
    }
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token_literal()
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_node: BooleanLiteral")
    }
}

pub struct InfixExpression {
    token: Token,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(
        token: &Token,
        left_expression: Box<dyn Expression>,
        right_expression: Box<dyn Expression>,
    ) -> Self {
        Self {
            token: token.clone(),
            left: left_expression,
            right: right_expression,
        }
    }
}

impl Node for InfixExpression {
    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.token.literal,
            self.right.string()
        )
    }

    fn token_literal(&self) -> String {
        format!(
            "({} {} {})",
            self.token.literal.clone(),
            self.left.string(),
            self.right.string()
        )
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_node: InfixExpression")
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn eval_expression(&self, env: &mut Environment) -> Box<dyn Object> {
        let left = self.left.eval_expression(env);
        let right = self.right.eval_expression(env);

        eval_infix_expression(left, right, &self.token)
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        Box::new(InfixExpression::new(
            &self.token,
            self.left.clone_expression(),
            self.right.clone_expression(),
        ))
    }
}

pub struct IntegerLiteral {
    token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: &Token, value: i64) -> Self {
        Self {
            token: token.clone(),
            value,
        }
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}

    fn eval_expression(&self, _env: &mut Environment) -> Box<dyn Object> {
        Box::new(Number::new(self.value))
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        Box::new(IntegerLiteral::new(&self.token, self.value))
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{:?} {}", self.token.kind, self.token_literal())
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_self: IntegerLiteral")
    }
}

pub struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(token: &Token, expression: Box<dyn Expression>) -> Self {
        Self {
            token: token.clone(),
            operator: token.literal.clone(),
            right: expression,
        }
    }
}

impl Node for PrefixExpression {
    fn string(&self) -> String {
        format!("({} {})", self.operator, self.right.string())
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_node: PrefixExpression")
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}

    fn eval_expression(&self, env: &mut Environment) -> Box<dyn Object> {
        let ob = self.right.eval_expression(env);
        let op = self.operator.as_str();

        match ob.kind() {
            BOOLEAN_OBJ => {
                if op != "!" {
                    panic!("unsupported expression: got {}", self.operator)
                } else {
                    Box::new(Boolean::opposite(ob.inspect()))
                }
            }
            NUMBER_OBJ => {
                if op != "-" {
                    panic!("unsupported expression: got {}", self.operator)
                } else {
                    Box::new(Number::negation(ob.inspect()))
                }
            }
            k => panic!("unsupported kind: got {}", k),
        }
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        Box::new(PrefixExpression::new(
            &self.token,
            self.right.clone_expression(),
        ))
    }
}

impl Debug for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: &Token) -> Self {
        Self {
            token: token.clone(),
            value: token.literal.clone(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_self: Identifier")
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}

    fn eval_expression(&self, env: &mut Environment) -> Box<dyn Object> {
        match env.get(self.value.clone()) {
            Some(v) => v,
            None => {
                panic!("identifier not found: {}", self.value)
            }
        }
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        Box::new(Identifier::new(&self.token))
    }
}

#[derive(Debug)]
pub struct IfExpression {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

impl Node for IfExpression {
    fn string(&self) -> String {
        match &self.alternative {
            Some(a) => format!(
                "if {} {} else {}",
                self.condition.string(),
                self.consequence.string(),
                a.string()
            ),
            None => format!(
                "if {} {}",
                self.condition.string(),
                self.consequence.string()
            ),
        }
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_self: IfExpression")
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}

    fn eval_expression(&self, env: &mut Environment) -> Box<dyn Object> {
        let condition = self.condition.eval_expression(env);
        match condition.kind() {
            BOOLEAN_OBJ => {
                match (
                    condition.inspect().parse::<bool>().unwrap(),
                    &self.alternative,
                ) {
                    (true, _) => self.consequence.eval_node(env),
                    (false, Some(alt)) => alt.eval_node(env),
                    _ => Box::new(None::new()),
                }
            }
            _ => {
                panic!("Invalid, should only be boolean")
            }
        }
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        match &self.alternative {
            Some(alt) => Box::new(IfExpression::new(
                self.token.clone(),
                self.condition.clone_expression(),
                self.consequence.clone_block_statement(),
                Some(alt.clone_block_statement()),
            )),
            None => Box::new(IfExpression::new(
                self.token.clone(),
                self.condition.clone_expression(),
                self.consequence.clone_block_statement(),
                None,
            )),
        }
    }
}

pub struct FunctionLiteral {
    token: Token,
    identifier: Identifier,
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(
        token: Token,
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            identifier,
            parameters,
            body,
        }
    }
}

impl Node for FunctionLiteral {
    fn string(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|x| x.string())
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "{} {} ( {} ) {}",
            self.token_literal(),
            self.identifier.string(),
            params,
            self.body.string()
        )
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_self: FunctionLiteral")
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}

    fn eval_expression(&self, env: &mut Environment) -> Box<dyn Object> {
        let func = Box::new(Function::new(
            self.identifier.clone(),
            self.parameters.clone(),
            self.body.clone_block_statement(),
            env,
        ));

        let cloned_func = func.clone_self();

        env.set(self.identifier.value.clone(), cloned_func);

        func
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        let params_cloned = self.parameters.to_vec().clone();

        Box::new(FunctionLiteral::new(
            self.token.clone(),
            self.identifier.clone(),
            params_cloned,
            self.body.clone_block_statement(),
        ))
    }
}

pub struct CallExpression {
    token: Token,
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            function,
            arguments,
        }
    }
}

impl Node for CallExpression {
    fn string(&self) -> String {
        let func = self.function.string();

        let args = self
            .arguments
            .iter()
            .map(|x| x.string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{} ( {} )", func, args)
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn eval_node(&self, _env: &mut Environment) -> Box<dyn Object> {
        todo!("eval_self: CallExpression")
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}

    fn eval_expression(&self, env: &mut Environment) -> Box<dyn Object> {
        let function_eval: Function = self
            .function
            .eval_expression(env)
            .extreme_hack_for_function();
        let args_eval = self
            .arguments
            .iter()
            .map(|arg| arg.eval_expression(env))
            .collect::<Vec<_>>();

        for (idx, param) in function_eval.parameters.iter().enumerate() {
            let arg = args_eval
                .get(idx)
                .expect(format!("Missing parameter: {}", idx).as_str())
                .clone();

            env.set(param.value.clone(), arg);
        }

        function_eval.body.eval_node(env)
    }

    fn clone_expression(&self) -> Box<dyn Expression> {
        let cloned_args = self
            .arguments
            .iter()
            .map(|args| args.clone_expression())
            .collect::<Vec<_>>();

        Box::new(CallExpression::new(
            self.token.clone(),
            self.function.clone_expression(),
            cloned_args,
        ))
    }
}
