use std::collections::HashMap;

use super::value::Value;

// I only managed to implement a single, global scope so far.
pub struct Env {
    scope: HashMap<String, Value>,
}

impl Env {
    pub fn new(builtins: HashMap<String, Value>) -> Self {
        Self {
            scope: builtins,
        }
    }

    pub fn add_binding(&mut self, name: String, value: Value) {
        self.scope.insert(name, value);
    }
    pub fn get_binding(&self, name: String) -> Option<&Value> {
        self.scope.get(&name)
    }
}
