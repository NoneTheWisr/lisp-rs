use std::rc::Rc;

use itertools::Itertools;

use super::{env::Env, value::Value};

impl Default for Env {
    #[rustfmt::skip]
    fn default() -> Self {
        Self::new(vec![
            ("+".into(), Value::Fun(Rc::new(add))),
            ("*".into(), Value::Fun(Rc::new(mul))),
            ("=".into(), Value::Fun(Rc::new(eq))),
            (">".into(), Value::Fun(Rc::new(gt))),
            ("<".into(), Value::Fun(Rc::new(lt))),
            ("def".into(), Value::Fun(Rc::new(def))),
        ].into_iter().collect())
    }
}

macro_rules! assert_arg_count {
    ($args:ident, $count:pat) => {
        if !matches!($args.len(), $count) {
            return Err(());
        }
    };
    ($args:ident, $count:pat, $err:expr) => {
        if !matches!($args.len(), $count) {
            return Err($err);
        }
    };
}

pub(super) fn add(args: Vec<Value>, _: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, (2..));

    use Value::*;
    match args[0] {
        Int(_) => args
            .into_iter()
            .map(|v| if let Int(v) = v { Ok(v) } else { Err(()) })
            .sum::<Result<_, _>>()
            .map(Int),
        Str(_) => args
            .into_iter()
            .map(|v| if let Str(v) = v { Ok(v) } else { Err(()) })
            .collect::<Result<_, _>>()
            .map(Str),
        _ => Err(()),
    }
}

pub(super) fn mul(args: Vec<Value>, _: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, (2..));

    use Value::*;
    match args[0] {
        Int(_) => args
            .into_iter()
            .map(|v| if let Int(v) = v { Ok(v) } else { Err(()) })
            .product::<Result<_, _>>()
            .map(Int),
        Str(_) => {
            assert_arg_count!(args, 2);
            match args.into_iter().next_tuple().unwrap() {
                (Value::Str(str), Value::Int(int)) => {
                    let int: usize = int.try_into().map_err(|_| ())?;
                    Ok(Value::Str(str.repeat(int)))
                }
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

pub(super) fn eq(args: Vec<Value>, _: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, (2..));

    Ok(Value::Bool(args.into_iter().all_equal()))
}

pub(super) fn gt(args: Vec<Value>, _: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, 2);

    match args.into_iter().next_tuple().unwrap() {
        (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs > rhs)),
        (Value::Str(lhs), Value::Str(rhs)) => Ok(Value::Bool(lhs > rhs)),
        _ => Err(()),
    }
}

pub(super) fn lt(args: Vec<Value>, _: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, 2);

    match args.into_iter().next_tuple().unwrap() {
        (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs < rhs)),
        (Value::Str(lhs), Value::Str(rhs)) => Ok(Value::Bool(lhs < rhs)),
        _ => Err(()),
    }
}

pub(super) fn def(args: Vec<Value>, env: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, 2);

    use Value::*;
    if let Some((Symbol(name), value)) = args.into_iter().next_tuple() {
        env.add_binding(name, value);
        Ok(None)
    } else {
        Err(())
    }
}
