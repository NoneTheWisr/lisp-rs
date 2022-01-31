use num_bigint::BigInt;


pub enum Ast {
    ExprList(Vec<Expr>),
    Expr(Expr),
}

pub enum Expr {
    List(Vec<Expr>),
    Ident(String),
    Int(BigInt),
    Str(String),
}