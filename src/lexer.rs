use itertools::Itertools;
use std::iter::Peekable;

use crate::token::Token;

type TResult = Result<Token, ()>;

struct Lexer<I: Iterator<Item = char>> {
    source: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn new(source: Peekable<I>) -> Self {
        Self { source }
    }

    fn next_token(&mut self) -> Option<TResult> {
        use Token::*;

        let next_char = self.source.peek()?;
        let next_token = match next_char {
            '(' => self.accept(LParen),
            ')' => self.accept(RParen),
            c if Self::starts_integer(c) => self.parse_integer(),
            c if Self::starts_string(c) => self.parse_string(),
            _ => Err(()),
        };

        Some(next_token)
    }

    fn consume(&mut self) {
        self.source.next();
    }
    fn accept(&mut self, t: Token) -> TResult {
        self.consume();
        Ok(t)
    }

    fn starts_integer(c: &char) -> bool {
        c.is_ascii_digit()
    }
    fn parse_integer(&mut self) -> TResult {
        Ok(Token::Integer(
            self.source
                .peeking_take_while(char::is_ascii_digit)
                .collect(),
        ))
    }

    fn starts_string(c: &char) -> bool {
        *c == '"'
    }
    fn parse_string(&mut self) -> Result<Token, ()> {
        self.consume();
        let contents = self.source.peeking_take_while(|&c| c != '"').collect();
        
        if self.source.peek().is_none() {
            Err(())
        } else {
            self.consume();
            Ok(Token::String(contents))
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = TResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<I> From<I> for Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn from(source: I) -> Self {
        Self::new(source.peekable())
    }
}

impl<'a> From<&'a str> for Lexer<std::str::Chars<'a>> {
    fn from(source: &'a str) -> Self {
        Self::new(source.chars().peekable())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    macro_rules! lexer_tests {
        ($($name:ident {$input:expr, $output:expr}),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(
                        Lexer::from($input).collect::<Result<Vec<_>, _>>(),
                        $output
                    );
                }
            )*
        };
    }

    lexer_tests! {
        test_ok {"()", Ok(vec![LParen, RParen])},
        test_err {"a", Err(())},
        test_integer_ok_single_digit {"1", Ok(vec![Integer("1".into())])},
        test_integer_ok_multi_gidit {"12345", Ok(vec![Integer("12345".into())])},
        test_string_ok_single {r#"" ""#, Ok(vec![String(" ".into())])},
        test_string_ok_multi {r#""12345""#, Ok(vec![String("12345".into())])},
        test_string_err {r#""123"#, Err(())},
    }
}
