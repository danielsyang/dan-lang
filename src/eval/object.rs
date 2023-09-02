pub type ObjectType = &'static str;

const NULL_OBJ: &str = "NULL";
const ERROR_OBJ: &str = "ERROR";
const INTEGER_OBJ: &str = "INTEGER";
const BOOLEAN_OBJ: &str = "BOOLEAN";
const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
const FUNCTION_OBJ: &str = "FUNCTION";

pub trait Object {
    fn kind(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub struct Number {
    value: i64,
}

impl Number {
    pub fn new(v: i64) -> Self {
        Self { value: v }
    }
}

impl Object for Number {
    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn kind(&self) -> ObjectType {
        INTEGER_OBJ
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
}

pub struct Null {}

impl Object for Null {
    fn inspect(&self) -> String {
        String::from("null")
    }
    fn kind(&self) -> ObjectType {
        NULL_OBJ
    }
}
