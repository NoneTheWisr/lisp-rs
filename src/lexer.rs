use std::iter::Peekable;
use crate::token::Token;

struct Lexer<I: Iterator<Item = char>> {
    source: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn next_token(&mut self) -> Option<Result<Token, ()>> {
        use Token::*;

        let next_char = self.source.next()?;
        let next_token = match next_char {
            ')' => Ok(RParen),
            '(' => Ok(LParen),
            _ => Err(()),
        };
        
        Some(next_token)
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, ()>;

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
