#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Identifier(String),
    Integer(String),
    String(String),
}