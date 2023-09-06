pub type ObjectType = &'static str;

const NULL_OBJ: &str = "NULL";
const _ERROR_OBJ: &str = "ERROR";
pub const NUMBER_OBJ: &str = "NUMBER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NONE_OBJ: &str = "NONE";
pub const RETURN_OBJ: &str = "RETURN_OBJ";
const _FUNCTION_OBJ: &str = "FUNCTION";

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

impl Boolean {
    pub fn new(v: bool) -> Self {
        Self { value: v }
    }

    pub fn opposite(str: String) -> Self {
        // I know that str has to be boolean, we can safely unwrap.
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
}
