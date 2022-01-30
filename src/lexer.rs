use itertools::Itertools;
use std::iter::Peekable;

use crate::token::Token;

type TResult = Result<Token, ()>;

struct Lexer<I: Iterator<Item = char>> {
    source: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn next_token(&mut self) -> Option<TResult> {
        use Token::*;

        let next_char = self.source.peek()?;
        let next_token = match next_char {
            '(' => self.accept(LParen),
            ')' => self.accept(RParen),
            c if Self::starts_integer(c) => self.parse_integer(),
            _ => Err(()),
        };

        Some(next_token)
    }

    fn accept(&mut self, t: Token) -> TResult {
        self.source.next();
        Ok(t)
    }

    fn is_ascii_numeric(c: &char) -> bool {
        "0123456789".contains(*c)
    }
    fn starts_integer(c: &char) -> bool {
        Self::is_ascii_numeric(c)
    }
    fn parse_integer(&mut self) -> TResult {
        Ok(Token::Integer(
            self.source
                .peeking_take_while(Self::is_ascii_numeric)
                .collect(),
        ))
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
        Lexer {
            source: source.peekable(),
        }
    }
}

impl<'a> From<&'a str> for Lexer<std::str::Chars<'a>> {
    fn from(source: &'a str) -> Self {
        Lexer {
            source: source.chars().peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    macro_rules! lexer_tests {
        ($($name:ident {$input:expr, $output:expr}),*) => {
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
        test_integer_ok_multi_gidit {"12345", Ok(vec![Integer("12345".into())])}
    }
}
