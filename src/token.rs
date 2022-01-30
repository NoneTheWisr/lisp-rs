#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(String),
    String(String),
}