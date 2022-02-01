use num_bigint::BigInt;

use crate::ast::Expr;

pub enum Value {
    Int(BigInt),
    Str(String),
}

impl From<Expr> for Value {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::List(_) => panic!(),
            Expr::Ident(_) => panic!(),
            Expr::Int(value) => Self::Int(value),
            Expr::Str(value) => Self::Str(value),
        }
    }
}