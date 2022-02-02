use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use itertools::Itertools;
use lisp_rs::eval::evaluator::*;
use lisp_rs::eval::env::Env;
use lisp_rs::frontend::lexer::Lexer;
use lisp_rs::frontend::parser;
use parser::Parser;

fn main() {
    let args = env::args().skip(1).collect_vec();
    match args.len() {
        0 => repl_mode(),
        1 => file_mode(args),
        _ => {
            println!("Usage: <executable> <path to source>\n    - to evaluate a file");
            println!("Usage: <executable>\n    - to start REPL mode");
            process::exit(1);
        }
    }
}

fn repl_mode() {
    println!("Welcome to Lame Repl(TM). Multiline input is not supported.");
    let mut env = Env::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read the input from stdin");
            process::exit(1);
        }

        if matches!(input.trim_end(), "q" | "quit" | "exit") {
            process::exit(0);
        }

        eval_string(&input, false, Some(&mut env));
    }
}

fn file_mode(args: Vec<String>) {
    let path = args.into_iter().next().unwrap();
    let source = match fs::read_to_string(path.clone()) {
        Err(_) => {
            println!("Coulnd't read the file at \"{path}\"");
            process::exit(1);
        }
        Ok(contents) => contents,
    };

    eval_string(&source, true, None)
}

fn eval_string(source: &str, quit_on_err: bool, env: Option<&mut Env>) {
    let tokens = Lexer::from(source);
    let ast = match Parser::new(tokens).parse() {
        Err(error) => {
            match error {
                parser::Error::LexerError(error) => println!("Lexing error: {error:?}"),
                error => println!("Parsing error: {error:?}"),
            };
            
            if quit_on_err { process::exit(1); } else { return; }
        }
        Ok(ast) => ast,
    };

    let value_result = match env {
        Some(env) => evaluate_toplevel_with_env(ast, env),
        None => evaluate_toplevel(ast),
    };
    let value = match value_result {
        Err(error) => {
            println!("Evaluation error: {error:?}");
            if quit_on_err { process::exit(1); } else { return; }
        }
        Ok(value) => value,
    };

    println!(": {value:?}");
}
