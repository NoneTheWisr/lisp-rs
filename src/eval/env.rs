use std::collections::HashMap;

use super::value::Value;

pub struct Env {
    scopes: Vec<HashMap<String, Value>>,
}

impl Env {
    pub fn new(builtins: HashMap<String, Value>) -> Self {
        Env {
            scopes: vec![builtins],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn add_binding(&mut self, name: String, value: Value) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }
    pub fn get_binding(&self, name: String) -> Option<&Value> {
        self.scopes.iter().rev().find_map(|scope| scope.get(&name))
    }
}
