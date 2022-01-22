use std::iter::Peekable;

use crate::token::Token;

struct Lexer<I: Iterator<Item = char>> {
    source: Peekable<I>,
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next()? {
            '(' => Some(Ok(Token::LParen)),
            ')' => Some(Ok(Token::RParen)),
            _ => Some(Err(())),
        }
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
        ($($name:ident {$input:expr, $output:expr},)*) => {
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
    }
}
