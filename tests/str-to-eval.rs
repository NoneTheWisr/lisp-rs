use lisp_rs::{lexer::Lexer, parser::Parser, eval::evaluator::evaluate_toplevel};
use lisp_rs::eval::value::{self, Value};
use num_bigint::BigInt;

#[test]
fn test() {
    let token_iter = Lexer::from("(+ 5 (+ 1 2 3 4 5))");
    let ast = Parser::new(token_iter).parse().unwrap();
    let value = evaluate_toplevel(ast);
    assert!(matches!(value, Ok(Value::Int(v)) if v == BigInt::from(20)));
}