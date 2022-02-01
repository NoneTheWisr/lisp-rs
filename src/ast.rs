use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub struct TopLevel(pub Vec<Expr>);

#[derive(Debug, PartialEq)]
pub enum Expr {
    List(Vec<Expr>),
    Ident(String),
    Int(BigInt),
    Str(String),
}