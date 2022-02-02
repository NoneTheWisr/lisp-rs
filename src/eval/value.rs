use std::rc::Rc;

use num_bigint::BigInt;

use super::env::Env;

#[derive(Clone)]
pub enum Value {
    Int(BigInt),
    Str(String),
    Bool(bool),
    Fun(Rc<dyn Fn(Vec<Value>, &mut Env) -> Result<Value, ()>>),
    Nil,
    Symbol(String),
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Str(lhs), Self::Str(rhs)) => lhs == rhs,
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            (Self::Symbol(lhs), Self::Symbol(rhs)) => lhs == rhs,
            (Self::Fun(_), Self::Fun(_)) => false,
            _ => true
        }
    }
}