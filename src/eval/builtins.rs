use itertools::Itertools;
use std::rc::Rc;

use super::{env::Env, error::Error, value::Value};

type EResult = Result<Value, Error>;

impl Default for Env {
    #[rustfmt::skip]
    fn default() -> Self {
        Self::new(vec![
            ("+".into(), Value::Fun(Rc::new(add))),
            ("*".into(), Value::Fun(Rc::new(mul))),
            ("=".into(), Value::Fun(Rc::new(eq))),
            ("def".into(), Value::Fun(Rc::new(def))),
        ].into_iter().collect())
    }
}

macro_rules! assert_arg_count {
    ($args:ident, $count:pat) => {
        if !matches!($args.len(), $count) {
            return Err(crate::eval::error::Error::WrongNumberOfArguments);
        }
    };
}

pub(super) fn add(args: Vec<Value>, _: &mut Env) -> EResult {
    assert_arg_count!(args, (2..));

    use Value::*;
    match args[0] {
        Int(_) => args
            .into_iter()
            .map(|v| {
                if let Int(v) = v {
                    Ok(v)
                } else {
                    Err(Error::TypeError)
                }
            })
            .sum::<Result<_, _>>()
            .map(Int),
        Str(_) => args
            .into_iter()
            .map(|v| {
                if let Str(v) = v {
                    Ok(v)
                } else {
                    Err(Error::TypeError)
                }
            })
            .collect::<Result<_, _>>()
            .map(Str),
        _ => Err(Error::TypeError),
    }
}

pub(super) fn mul(args: Vec<Value>, _: &mut Env) -> EResult {
    assert_arg_count!(args, (2..));

    use Value::*;
    match args[0] {
        Int(_) => args
            .into_iter()
            .map(|v| {
                if let Int(v) = v {
                    Ok(v)
                } else {
                    Err(Error::TypeError)
                }
            })
            .product::<Result<_, _>>()
            .map(Int),
        Str(_) => {
            assert_arg_count!(args, 2);
            match args.into_iter().next_tuple().unwrap() {
                (Value::Str(str), Value::Int(int)) => {
                    let int: usize = int.try_into().map_err(|_| Error::InternalError)?;
                    Ok(Value::Str(str.repeat(int)))
                }
                _ => Err(Error::TypeError),
            }
        }
        _ => Err(Error::TypeError),
    }
}

pub(super) fn eq(args: Vec<Value>, _: &mut Env) -> EResult {
    assert_arg_count!(args, (2..));

    Ok(Value::Bool(args.into_iter().all_equal()))
}

pub(super) fn def(args: Vec<Value>, env: &mut Env) -> EResult {
    assert_arg_count!(args, 2);

    use Value::*;
    if let Some((Symbol(name), value)) = args.into_iter().next_tuple() {
        env.add_binding(name, value);
        Ok(None)
    } else {
        Err(Error::TypeError)
    }
}
