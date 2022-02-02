use std::collections::HashSet;

use super::ast::*;
use super::lexer::Error as LError;
use super::token::Token;

// -------------------------------------------------------------------------- //
// Error type                                                                 //
// -------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    LexerError(LError),
    UnbalancedParens,
    QuotingNotSupported,
}

impl From<LError> for Error {
    fn from(err: LError) -> Self {
        Self::LexerError(err)
    }
}

// -------------------------------------------------------------------------- //
// Type aliases                                                               //
// -------------------------------------------------------------------------- //

type Item = Result<Token, LError>;
type AstResult = Result<TopLevel, Error>;

// -------------------------------------------------------------------------- //
// Macros                                                                     //
// -------------------------------------------------------------------------- //

macro_rules! assert_unqoted {
    ($self:ident) => {
        if $self.should_quote() {
            return Some(crate::frontend::parser::Error::QuotingNotSupported)
        }
    }
}

// -------------------------------------------------------------------------- //
// Parser struct                                                              //
// -------------------------------------------------------------------------- //

// list_stack is a stack of lists that we've encountered so far. When a '('
// is encountered, level is bumped up and a new list is pushed onto the stack.
// When a ')' is encountered, level is bumped down and the current list is
// popped from the stack and appended to the parent list.
pub struct Parser<I: Iterator<Item = Item>> {
    tokens: I,
    list_stack: Vec<Vec<Expr>>,
    level: u64,
    quote_levels: HashSet<u64>
}

impl<I: Iterator<Item = Item>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Parser {
            tokens,
            list_stack: Vec::new(),
            level: 0,
            quote_levels: HashSet::new()
        }
    }

    // ---------------------------------------------------------------------- //
    // Main parse method                                                      //
    // ---------------------------------------------------------------------- //

    pub fn parse(&mut self) -> AstResult {
        // This vec (list) will contain all top-level expressions
        self.list_stack.push(Vec::new());
        
        // I must be doing something really wrong if I have to use this instead
        // of `for token in self.tokens`. I've tried multiple things but this
        // seemed to be the only reasonable one that worked 
        while let Some(token_result) = self.tokens.next() {
            let token = token_result?;
            if let Some(err) = match token {
                Token::LParen => self.lparen(),
                Token::RParen => self.rparen(),
                Token::Quote => self.quote(),
                Token::Identifier(str) => self.ident(str),
                Token::Integer(str) => self.int(str),
                Token::String(str) => self.str(str),
                Token::Boolean(val) => self.bool(val),
            } {
                return Err(err);
            }
        }
        
        if self.level != 0 {
            return Err(Error::UnbalancedParens);
        }

        // NOTE: Emtpy top-level expressions are supported.
        Ok(TopLevel(self.list_stack.pop().unwrap()))
    }

    // ---------------------------------------------------------------------- //
    // Token specific methods                                                 //
    // ---------------------------------------------------------------------- //

    // Grouping token methods: (, ), ' -------------------------------------- //
    fn lparen(&mut self) -> Option<Error> {
        self.level += 1;
        self.list_stack.push(Vec::new());
        None
    }
    fn rparen(&mut self) -> Option<Error> {
        if self.level > 0 {
            self.level -= 1;

            // I haven't implemented quoting lists yet so it's an error.
            assert_unqoted!(self);

            let current = self.list_stack.pop().unwrap();
            self.push_last(Expr::List(current));

            None
        } else {
            Some(Error::UnbalancedParens)
        }
    }
    fn quote(&mut self) -> Option<Error> {
        self.quote_levels.insert(self.level);
        None
    }

    // Other token methods -------------------------------------------------- //
    fn ident(&mut self, str: String) -> Option<Error> {
        let value = self.quote_if_needed(Expr::Ident(str));
        self.push_last(value);
        None
    }
    fn int(&mut self, str: String) -> Option<Error> {
        assert_unqoted!(self);
        // Only valid ints should end up here.
        let int = str.parse().unwrap();
        self.push_last(Expr::Int(int));
        None
    }
    fn str(&mut self, str: String) -> Option<Error> {
        assert_unqoted!(self);
        self.push_last(Expr::Str(str));
        None
    }
    fn bool(&mut self, val: bool) -> Option<Error> {
        assert_unqoted!(self);
        self.push_last(Expr::Bool(val));
        None
    }

    // ---------------------------------------------------------------------- //
    // Helper methods                                                         //
    // ---------------------------------------------------------------------- //

    // List stack access ---------------------------------------------------- //
    fn push_last(&mut self, expr: Expr) {
        self.list_stack.last_mut().unwrap().push(expr);
    }

    // Quote realed methods ------------------------------------------------- //
    fn should_quote(&mut self) -> bool {
        let level = &self.level;
        if self.quote_levels.contains(level) {
            self.quote_levels.remove(level);
            true
        } else {
            false
        }
    }
    fn quote_if_needed(&mut self, expr: Expr) -> Expr {
        if self.should_quote() {
            Expr::Quoted(Box::new(expr))
        } else {
            expr
        }
    }
}

// -------------------------------------------------------------------------- //
// Tests                                                                      //
// -------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::token::test_macros::*;
    use crate::frontend::ast::{Expr::*, TopLevel};

    macro_rules! parser_tests {
        ($($name:ident {[$($item:expr),*], $output:expr}),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(
                        Parser::new(vec![$(Ok($item)),*].into_iter()).parse(),
                        $output
                    )
                }
            )*
        }
    }

    #[rustfmt::skip]
    parser_tests! {
        test_ok_1 { [int!("1")], Ok(TopLevel(vec![Int(1.into())]))},
        test_ok_2 { [lp!(), ident!("+"), int!("1"), int!("9"), rp!()], Ok(
            TopLevel(vec![
                List(vec![
                    Ident("+".into()),
                    Int(1.into()),
                    Int(9.into())
                ])
            ])
        )},
        test_ok_3 { [int!("1"), int!("2")], Ok(
            TopLevel(vec![
                Int(1.into()),
                Int(2.into())
            ])
        )},
        test_ok_4 {
            [
                lp!(), ident!("+"), lp!(), ident!("*"), int!("2"), int!("5"), rp!(),
                                    int!("9"), rp!(),
                int!("1")
            ],
            Ok(
                TopLevel(vec![
                    List(vec![
                        Ident("+".into()),
                        List(vec![
                            Ident("*".into()),
                            Int(2.into()),
                            Int(5.into())
                        ]),
                        Int(9.into())
                    ]),
                    Int(1.into())
                ])
            )
        },
        test_ok_5_quoting_1 {
            [
                lp!(), ident!("def"), q!(), ident!("a"), int!("10"), rp!()
            ],
            Ok(
                TopLevel(vec![
                    List(vec![
                        Ident("def".into()),
                        Quoted(Box::new(Ident("a".into()))),
                        Int(10.into())
                    ])
                ])
            )
        },
        test_ok_6_quoting_2 {
            [
                lp!(), ident!("def"), q!(), ident!("a"),
                                      q!(), lp!(), int!("1"), int!("2"), int!("3"), rp!(), rp!()
            ],
            Err(Error::QuotingNotSupported)
        },
        test_err_1 { [lp!(), int!("2")], Err(Error::UnbalancedParens)},
        test_err_2 { [int!("2"), rp!()], Err(Error::UnbalancedParens)},
    }
}
