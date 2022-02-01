use std::rc::Rc;

use super::{env::Env, value::Value};

impl Default for Env {
    #[rustfmt::skip]
    fn default() -> Self {
        Self::new(vec![
            ("+".into(), Value::Fun(Rc::new(add))),
        ].into_iter().collect())
    }
}

pub(super) fn add(args: Vec<Value>) -> Result<Value, ()> {
    use Value::*;
    match args[0] {
        Fun(_) | Nil | None => Err(()),
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
    }
}
