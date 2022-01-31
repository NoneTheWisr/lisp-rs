use lisp_rs::{lexer::Lexer, parser::Parser};
use lisp_rs::ast::{Expr::*, TopLevel::*};

macro_rules! str_to_ast_tests {
    ($($name:ident {$input:expr, $output:expr}),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                assert_eq!(
                    Parser::new(Lexer::from($input)).parse(),
                    $output
                );
            }
        )*
    }
}

#[rustfmt::skip]
str_to_ast_tests! {
    test1 { "1", Ok(Single(Int(1.into())))},
    test2 { "(+ 1 9)", Ok(
        Single(List(vec![Ident("+".into()),
                         Int(1.into()),
                         Int(9.into())]))
    )},
    test3 { "(+ (* 2 5) 9)\n1", Ok(
        Multi(vec![List(vec![Ident("+".into()),
                             List(vec![Ident("*".into()),
                                       Int(2.into()),
                                       Int(5.into())]),
                             Int(9.into())]),
                   Int(1.into())])
    )},
}