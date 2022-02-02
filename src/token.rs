#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Quote,
    Identifier(String),
    Integer(String),
    String(String),
    Boolean(bool),
}

#[cfg(test)]
#[rustfmt::skip]
pub(crate) mod test_macros {
    macro_rules! lp { () => { crate::token::Token::LParen } }
    macro_rules! rp { () => { crate::token::Token::RParen } }
    macro_rules! q  { () => { crate::token::Token::Quote  } }
    macro_rules! ident { ($str:literal)  => { crate::token::Token::Identifier($str.to_string())   } }
    macro_rules! int   { ($str:literal)  => { crate::token::Token::Integer($str.parse().unwrap()) } }
    macro_rules! str   { ($str:literal)  => { crate::token::Token::String($str.to_string())       } }
    macro_rules! bool  { ($bool:literal) => { crate::token::Token::Boolean($bool)                 } }

    pub(crate) use {lp, rp, q, ident, int, str, bool};
}
