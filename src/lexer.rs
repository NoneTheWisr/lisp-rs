use itertools::Itertools;
use std::iter::Peekable;

use crate::token::Token;

type TResult = Result<Token, ()>;

// -------------------------------------------------------------------------- //
// Main struct                                                                //
// -------------------------------------------------------------------------- //

struct Lexer<I: Iterator<Item = char>> {
    source: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn new(source: Peekable<I>) -> Self {
        Self { source }
    }

    // ---------------------------------------------------------------------- //
    // Primary lexing method                                                  //
    // ---------------------------------------------------------------------- //

    fn next_token(&mut self) -> Option<TResult> {
        use Token::*;

        self.skip_whitespace();
        let next_char = self.peek()?;

        let next_token = match next_char {
            '(' => self.accept(LParen),
            ')' => self.accept(RParen),
            c if Self::starts_identifier(c) => self.parse_identifier(),
            c if Self::starts_integer(c) => self.parse_integer(),
            c if Self::starts_string(c) => self.parse_string(),
            // Unexpected symbol
            _ => Err(()),
        };

        Some(next_token)
    }

    // ---------------------------------------------------------------------- //
    // Whitespace handling                                                    //
    // ---------------------------------------------------------------------- //

    fn is_whitespace(c: &char) -> bool {
        c.is_ascii_whitespace()
    }
    fn skip_whitespace(&mut self) {
        self.skip_while(Self::is_whitespace);
    }

    // ---------------------------------------------------------------------- //
    // Token specific lexing methods                                          //
    // ---------------------------------------------------------------------- //

    // Identifier ----------------------------------------------------------- //
    fn starts_identifier(c: &char) -> bool {
        c.is_ascii_graphic() && !c.is_ascii_digit() && !"()\"".contains(*c)
    }
    fn parse_identifier(&mut self) -> TResult {
        let matcher = |c: &char| c.is_ascii_graphic() && !"()\"".contains(*c);
        Ok(Token::Identifier(self.collect_while(matcher)))
    }

    // Integer -------------------------------------------------------------- //
    fn starts_integer(c: &char) -> bool {
        c.is_ascii_digit()
    }
    fn parse_integer(&mut self) -> TResult {
        Ok(Token::Integer(self.collect_while(char::is_ascii_digit)))
    }

    // String --------------------------------------------------------------- //
    fn starts_string(c: &char) -> bool {
        *c == '"'
    }
    fn parse_string(&mut self) -> TResult {
        self.consume();
        let contents = self.collect_while(|&c| c != '"');

        if self.peek().is_none() {
            // Missing the closing " on a string. Reached EOF
            Err(())
        } else {
            self.consume();
            Ok(Token::String(contents))
        }
    }

    // ---------------------------------------------------------------------- //
    // Helper methods                                                         //
    // ---------------------------------------------------------------------- //

    // Single element ------------------------------------------------------- //
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
    fn consume(&mut self) {
        self.source.next();
    }
    fn accept(&mut self, t: Token) -> TResult {
        self.consume();
        Ok(t)
    }

    // Multi element -------------------------------------------------------- //
    fn collect_while<F: FnMut(&char) -> bool>(&mut self, matcher: F) -> String {
        self.source.peeking_take_while(matcher).collect()
    }
    fn skip_while<F: FnMut(&char) -> bool>(&mut self, matcher: F) {
        self.source.peeking_take_while(matcher).for_each(drop)
    }
}

// -------------------------------------------------------------------------- //
// Trait implementations                                                      //
// -------------------------------------------------------------------------- //

// Iterator ----------------------------------------------------------------- //
impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = TResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

// From for iterators ------------------------------------------------------- //
impl<I> From<I> for Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn from(source: I) -> Self {
        Self::new(source.peekable())
    }
}

// From for string slices --------------------------------------------------- //
impl<'a> From<&'a str> for Lexer<std::str::Chars<'a>> {
    fn from(source: &'a str) -> Self {
        Self::new(source.chars().peekable())
    }
}

// -------------------------------------------------------------------------- //
// Tests                                                                      //
// -------------------------------------------------------------------------- //

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
        test_ok {"()", Ok(vec![
            LParen,
            RParen
        ])},
        test_identifier {"a124<./S?>F", Ok(vec![
            Identifier("a124<./S?>F".into())
        ])},
        test_integer_ok_single_digit {"1", Ok(vec![
            Integer("1".into())
        ])},
        test_integer_ok_multi_gidit {"12345", Ok(vec![
            Integer("12345".into())
        ])},
        test_string_ok_single {r#"" ""#, Ok(vec![
            String(" ".into())
        ])},
        test_string_ok_multi {r#""12345""#, Ok(vec![
            String("12345".into())
        ])},
        test_string_err {r#""123"#, Err(())},
        test_mixed_1 {"124<./S?>F", Ok(vec![
            Integer("124".into()),
            Identifier("<./S?>F".into())
        ])},
        test_mixed_2 {"(+ -5-124<./S?>F 35)", Ok(vec![
            LParen,
            Identifier("+".into()),
            Identifier("-5-124<./S?>F".into()),
            Integer("35".into()),
            RParen
        ])},
    }
}
