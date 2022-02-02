use lisp_rs::frontend::{lexer::Lexer, parser::Parser};
use lisp_rs::frontend::ast::{Expr::*, TopLevel};

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
    test1 { "1", Ok(TopLevel(vec![Int(1.into())]))},
    test2 { "(+ 1 9)", Ok(
        TopLevel(vec![
            List(vec![
                Ident("+".into()),
                Int(1.into()),
                Int(9.into())
            ])
        ])
    )},
    test3 { 
        "(+ (* 2 5)
            9)
         1",
        Ok(TopLevel(vec![
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
    )},
}
