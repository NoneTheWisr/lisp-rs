use std::collections::HashSet;

use crate::ast::*;
use crate::lexer::Error as LError;
use crate::token::Token;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    LexerError(LError),
    UnbalancedParens,
    DisallowedQuoting,
}

type Item = Result<Token, LError>;
type AstResult = Result<TopLevel, Error>;

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

    pub fn parse(&mut self) -> AstResult {
        // This vec (list) will contain all top-level expressions
        self.list_stack.push(Vec::new());
        
        // I must be doing something really wrong if I have to use this instead
        // of `for token in self.tokens`. I've tried multiple things but this
        // seemed to be the only reasonable one that worked 
        while let Some(token) = self.tokens.next() {
            match token {
                Ok(token) => {
                    if let Some(err) = match token {
                        Token::LParen => self.lparen(),
                        Token::RParen => self.rparen(),
                        Token::Quote => self.quote(),
                        Token::Identifier(str) => self.ident(str),
                        Token::Integer(str) => self.int(str),
                        Token::String(str) => self.str(str),
                    } {
                        return Err(err);
                    }
                }
                Err(error) => {
                    return Err(Error::LexerError(error));
                }
            }
        }
        
        if self.level != 0 {
            return Err(Error::UnbalancedParens);
        }

        // NOTE: Emtpy top-level expressions are supported.
        Ok(TopLevel(self.list_stack.pop().unwrap()))
    }

    fn lparen(&mut self) -> Option<Error> {
        self.level += 1;
        self.list_stack.push(Vec::new());
        None
    }
    fn rparen(&mut self) -> Option<Error> {
        if self.level > 0 {
            self.level -= 1;

            let current = self.list_stack.pop().unwrap();
            let value = self.quote_if_needed(Expr::List(current));
            let parent = self.list_stack.last_mut().unwrap();
            parent.push(value);

            None
        } else {
            Some(Error::UnbalancedParens)
        }
    }
    fn quote(&mut self) -> Option<Error> {
        self.quote_levels.insert(self.level);
        None
    }

    fn ident(&mut self, str: String) -> Option<Error> {
        let value = self.quote_if_needed(Expr::Ident(str));
        self.list_stack.last_mut().unwrap().push(value);
        None
    }
    fn int(&mut self, str: String) -> Option<Error> {
        if self.should_quote() {
            Some(Error::DisallowedQuoting)
        } else {
            let int = str.parse().unwrap();
            self.list_stack.last_mut().unwrap().push(Expr::Int(int));
            None
        }
    }
    fn str(&mut self, str: String) -> Option<Error> {
        if self.should_quote() {
            Some(Error::DisallowedQuoting)
        } else {
            self.list_stack.last_mut().unwrap().push(Expr::Str(str));
            None
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::test_macros::*;
    use crate::ast::{Expr::*, TopLevel};

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
            Ok(
                TopLevel(vec![
                    List(vec![
                        Ident("def".into()),
                        Quoted(Box::new(Ident("a".into()))),
                        Quoted(Box::new(List(vec![
                            Int(1.into()),
                            Int(2.into()),
                            Int(3.into()),
                        ])))
                    ])
                ])
            )
        },
        test_err_1 { [lp!(), int!("2")], Err(Error::UnbalancedParens)},
        test_err_2 { [int!("2"), rp!()], Err(Error::UnbalancedParens)},
    }
}
