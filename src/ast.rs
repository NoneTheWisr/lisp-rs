use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum TopLevel {
    Multi(Vec<Expr>),
    Single(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    List(Vec<Expr>),
    Ident(String),
    Int(BigInt),
    Str(String),
}