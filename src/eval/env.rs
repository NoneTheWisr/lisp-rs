use std::collections::HashMap;

use super::value::Value;

pub struct Env {
    bindings: HashMap<String, Value>,
    parent: Box<Env>,
}