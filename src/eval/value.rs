use std::rc::Rc;

use num_bigint::BigInt;

use super::env::Env;

#[derive(Clone)]
pub enum Value {
    Int(BigInt),
    Str(String),
    Fun(Rc<dyn Fn(Vec<Value>, &mut Env) -> Result<Value, ()>>),
    Nil,
    None,
}
