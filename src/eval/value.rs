use std::rc::Rc;

use num_bigint::BigInt;

#[derive(Clone)]
pub enum Value {
    Int(BigInt),
    Str(String),
    Fun(Rc<dyn Fn(Vec<Value>) -> Result<Value, ()>>),
    Nil,
    None,
}

