use crate::eval::object::Object;
use std::collections::HashMap;

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
        Self {
            store: HashMap::new(),
        }
    }

    pub fn new_from(store: HashMap<String, Object>) -> Self {
        Self { store }
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        match self.store.get(name.as_str()) {
            Some(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }

    pub fn clone(&mut self) -> Self {
        Self {
            store: HashMap::clone(&self.store),
        }
    }
}
