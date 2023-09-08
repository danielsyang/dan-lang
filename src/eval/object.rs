use std::collections::HashMap;

use crate::ast::{expression::Identifier, statement::BlockStatement};

use super::environment::Environment;

pub type ObjectType = &'static str;

const NULL_OBJ: &str = "NULL";
const _ERROR_OBJ: &str = "ERROR";
pub const NUMBER_OBJ: &str = "NUMBER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NONE_OBJ: &str = "NONE";
pub const RETURN_OBJ: &str = "RETURN_OBJ";
pub const FUNCTION_OBJ: &str = "FUNCTION";
pub const STRING_OBJ: &str = "STRING_OBJ";

pub trait Object {
    fn kind(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn clone_self(&self) -> Box<dyn Object>;

    // I really don't know what better way to get struct without downcasting
    fn extreme_hack_for_function(&self) -> Function;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_self()
    }
}

pub struct Number {
    value: i64,
}

impl Number {
    pub fn new(v: i64) -> Self {
        Self { value: v }
    }

    pub fn negation(v: String) -> Self {
        let n = v.parse::<i64>().unwrap();
        Self { value: 0 - n }
    }
}

impl Object for Number {
    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn kind(&self) -> ObjectType {
        NUMBER_OBJ
    }

    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(Number::new(self.value))
    }

    fn extreme_hack_for_function(&self) -> Function {
        unreachable!("Should have never happened.")
    }
}

pub struct Boolean {
    value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn kind(&self) -> ObjectType {
        BOOLEAN_OBJ
    }

    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(Boolean::new(self.value))
    }

    fn extreme_hack_for_function(&self) -> Function {
        unreachable!("Should have never happened.")
    }
}

impl Boolean {
    pub fn new(v: bool) -> Self {
        Self { value: v }
    }

    pub fn opposite(str: String) -> Self {
        let prev = str.parse::<bool>().unwrap();

        Boolean::new(!prev)
    }
}

pub struct Null {}

impl Object for Null {
    fn inspect(&self) -> String {
        String::from("null")
    }
    fn kind(&self) -> ObjectType {
        NULL_OBJ
    }

    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(Null {})
    }

    fn extreme_hack_for_function(&self) -> Function {
        unreachable!("Should have never happened.")
    }
}

pub struct None {}

impl None {
    pub fn new() -> Self {
        None {}
    }
}

impl Object for None {
    fn inspect(&self) -> String {
        String::from("none")
    }
    fn kind(&self) -> ObjectType {
        NONE_OBJ
    }

    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(None {})
    }

    fn extreme_hack_for_function(&self) -> Function {
        unreachable!("Should have never happened.")
    }
}

pub struct Return {
    value: Box<dyn Object>,
}

impl Return {
    pub fn new(v: Box<dyn Object>) -> Self {
        Self { value: v }
    }
}

impl Object for Return {
    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn kind(&self) -> ObjectType {
        RETURN_OBJ
    }

    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(Return::new(self.value.clone_self()))
    }

    fn extreme_hack_for_function(&self) -> Function {
        unreachable!("Should have never happened.")
    }
}

pub struct Function {
    name: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    env: Environment,
}

impl Function {
    pub fn new(
        name: Identifier,
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: &mut Environment,
    ) -> Self {
        let new_env = env.clone();

        Self {
            name,
            parameters,
            body,
            env: new_env,
        }
    }
}

impl Object for Function {
    fn clone_self(&self) -> Box<dyn Object> {
        let params_cloned = self.parameters.to_vec().clone();
        let cloned_env = HashMap::clone(&self.env.store);
        let mut new_env = Environment::new_from(cloned_env);

        Box::new(Function::new(
            self.name.clone(),
            params_cloned,
            self.body.clone_block_statement(),
            &mut new_env,
        ))
    }

    fn inspect(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|param| param.value.clone())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "fn {} ( {} ) {{ {} }}",
            self.name.value,
            params,
            self.body.string_hack()
        )
    }

    fn kind(&self) -> ObjectType {
        FUNCTION_OBJ
    }

    fn extreme_hack_for_function(&self) -> Function {
        let params_cloned = self.parameters.to_vec().clone();
        let cloned_env = HashMap::clone(&self.env.store);
        let mut new_env = Environment::new_from(cloned_env);

        Function::new(
            self.name.clone(),
            params_cloned,
            self.body.clone_block_statement(),
            &mut new_env,
        )
    }
}

pub struct StringObj {
    value: String,
}

impl StringObj {
    pub fn new(v: String) -> Self {
        Self { value: v }
    }
}

impl Object for StringObj {
    fn clone_self(&self) -> Box<dyn Object> {
        Box::new(StringObj::new(self.value.clone()))
    }

    fn extreme_hack_for_function(&self) -> Function {
        unreachable!("Should have never happened")
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }

    fn kind(&self) -> ObjectType {
        STRING_OBJ
    }
}
