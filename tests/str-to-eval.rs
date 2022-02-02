use lisp_rs::{lexer::Lexer, parser::Parser, eval::evaluator::evaluate_toplevel};
use lisp_rs::eval::value::{Value};
use num_bigint::BigInt;

#[test]
fn omg_it_works_for_ints() {
    let token_iter = Lexer::from("(+ 5 (+ 1 2 3 4 5))");
    let ast = Parser::new(token_iter).parse().unwrap();
    let value = evaluate_toplevel(ast);
    assert!(matches!(value, Ok(Value::Int(v)) if v == BigInt::from(20)));
}
#[test]
fn omg_it_works_for_strings() {
    let token_iter = Lexer::from(r#"(+ "5" (+ "1" "2" "3" "4" "5"))"#);
    let ast = Parser::new(token_iter).parse().unwrap();
    let value = evaluate_toplevel(ast);
    assert!(matches!(value, Ok(Value::Str(v)) if v == String::from("512345")));
}
#[test]
fn omg_errors_work_too() {
    let token_iter = Lexer::from(r#"(+ "5" (+ "1" "2" "3" "4" 5))"#);
    let ast = Parser::new(token_iter).parse().unwrap();
    let value = evaluate_toplevel(ast);
    assert!(matches!(value, Err(())));
}

#[test]
fn test_def() {
    let token_iter = Lexer::from(
        "(def 'a 0)
         (def 'a 10)
         (def 'b 5)
         (+ a b 5)"
    );
    let ast = Parser::new(token_iter).parse().unwrap();
    let value = evaluate_toplevel(ast);
    assert!(matches!(value, Ok(Value::Int(v)) if v == BigInt::from(20)));
}

#[test]
fn test_def_string_mul_and_cmp() {
    let token_iter = Lexer::from(
        r#"(def 'str "ha")
           (= (* str 2) "haha")"#
    );
    let ast = Parser::new(token_iter).parse().unwrap();
    let value = evaluate_toplevel(ast);
    assert!(matches!(value, Ok(Value::Bool(true))));
}