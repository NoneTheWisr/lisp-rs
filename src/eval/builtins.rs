use std::rc::Rc;

use itertools::Itertools;

use super::{env::Env, value::Value};

impl Default for Env {
    #[rustfmt::skip]
    fn default() -> Self {
        Self::new(vec![
            ("+".into(), Value::Fun(Rc::new(add))),
            ("def".into(), Value::Fun(Rc::new(def))),
        ].into_iter().collect())
    }
}

macro_rules! assert_arg_count {
    ($args:ident, $count:pat) => {
        if !matches!($args.len(), pat) {
            return Err(());
        }
    };
    ($args:ident, $count:pat, $err:expr) => {
        if !matches!($args.len(), pat) {
            return Err($err);
        }
    };
}

pub(super) fn add(args: Vec<Value>, _: &mut Env) -> Result<Value, ()> {
    assert_arg_count!(args, (1..));

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
