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

    pub fn get(&self, name: String) -> (Option<&dyn Object>, bool) {
        match self.store.get(name.as_str()) {
            Some(val) => (Some(val.as_ref()), true),
            None => (None, false),
        }
    }

    pub fn set(&mut self, name: String, val: Box<dyn Object>) -> Box<dyn Object> {
        self.store
            .insert(name, val)
            .expect("Something went wrong, can't insert into map")
    }
}
