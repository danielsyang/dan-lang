use crate::eval::object::Object;
use std::collections::HashMap;

use super::builtin_functions;

pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        builtin_functions()
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        self.store.get(name.as_str()).cloned()
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}
