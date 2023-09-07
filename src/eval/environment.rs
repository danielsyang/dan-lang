use std::collections::HashMap;

use super::object::Object;

pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: String) -> Option<Box<dyn Object>> {
        match self.store.get(name.as_str()) {
            Some(v) => Some(v.clone_self()),
            _ => None,
        }
    }

    pub fn set(&mut self, name: String, val: Box<dyn Object>) {
        self.store.insert(name, val);
    }

    pub fn clone(&mut self) -> Self {
        Self {
            store: HashMap::clone(&self.store),
        }
    }
}
