use crate::ast::{Expr, TopLevel};

use super::{env::Env, value::Value};

type EResult = Result<Value, ()>;

pub fn evaluate_toplevel(ast: TopLevel) -> EResult {
    eval_toplevel(ast, &mut Env::default())
}

fn eval_toplevel(ast: TopLevel, env: &mut Env) -> EResult {
    ast.0
        .into_iter()
        .map(|expr| eval_expr(expr, env))
        .last()
        .unwrap_or(Ok(Value::None))
}

fn eval_expr(ast: Expr, env: &mut Env) -> EResult {
    (match ast {
        Expr::List(body) => eval_list(body, env),
        Expr::Ident(name) => lookup_indent(name, env),
        Expr::Int(value) => Ok(Value::Int(value)),
        Expr::Str(value) => Ok(Value::Str(value)),
        Expr::Bool(value) => Ok(Value::Bool(value)),
        Expr::Quoted(expr) => match *expr {
            Expr::Quoted(_) | Expr::Int(_) | Expr::Str(_) | Expr::Bool(_) => unreachable!(),
            Expr::List(_) => unimplemented!(),
            Expr::Ident(name) => Ok(Value::Symbol(name)),
        },
    })
}

fn eval_list(body: Vec<Expr>, env: &mut Env) -> EResult {
    match body.len() {
        0 => Ok(Value::Nil),
        n => {
            let mut values = body.into_iter().map(|item| eval_expr(item, env));
            let first = values.next().unwrap()?;
            let rest = values.collect::<Result<Vec<_>, _>>()?;

            if let Value::Fun(fun) = first {
                fun(rest, env)
            } else if n == 1 {
                Ok(first)
            } else {
                Err(())
            }
        }
    }
}

fn lookup_indent(name: String, env: &Env) -> EResult {
    env.get_binding(name).map(Clone::clone).ok_or(())
}
