use std::iter::from_fn;
use std::str::FromStr;

use itertools::Itertools;
use num_bigint::BigInt;

use crate::ast::*;
use crate::lexer::Error;
use crate::token::Token;

type Item = Result<Token, Error>;
type AstResult = Result<Ast, ()>;
type ExprResult = Result<Expr, ()>;
type ErrResult = Result<(), ()>;

struct Parser<I: Iterator<Item = Item>> {
    tokens: I,
    list_stack: Vec<Vec<Expr>>,
    level: u64,
}

impl<I: Iterator<Item = Item>> Parser<I> {
    fn new(tokens: I) -> Self {
        Parser {
            tokens,
            list_stack: Vec::new(),
            level: 0,
        }
    }

    fn parse(&mut self) -> AstResult {
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
                        Token::Identifier(str) => self.ident(str),
                        Token::Integer(str) => self.int(str),
                        Token::String(str) => self.str(str),
                    } {
                        return Err(err);
                    }
                }
                Err(error) => {
                    return Err(());
                }
            }
        }


        let root_list = self.list_stack.pop().unwrap();
        match root_list.len() {
            0 => Err(()),
            1 => Ok(Ast::Expr(root_list.into_iter().next().unwrap())),
            _ => Ok(Ast::ExprList(root_list)),
        }
    }

    fn lparen(&mut self) -> Option<()> {
        self.level += 1;
        self.list_stack.push(Vec::new());
        None
    }
    fn rparen(&mut self) -> Option<()> {
        if self.level > 0 {
            self.level -= 1;

            let current = self.list_stack.pop().unwrap();
            let parent = self.list_stack.last_mut().unwrap();
            parent.push(Expr::List(current));

            None
        } else {
            Some(())
        }
    }
    fn ident(&mut self, str: String) -> Option<()> {
        self.list_stack.last_mut().unwrap().push(Expr::Ident(str));
        None
    }
    fn int(&mut self, str: String) -> Option<()> {
        let int = BigInt::from_str(&str).unwrap();
        self.list_stack.last_mut().unwrap().push(Expr::Int(int));
        None
    }
    fn str(&mut self, str: String) -> Option<()> {
        self.list_stack.last_mut().unwrap().push(Expr::Str(str));
        None
    }
}
