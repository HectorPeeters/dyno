#![allow(dead_code)]

use dyno::*;
use std::env;
use std::io::{stdin, stdout, Write};

fn read_input() -> String {
    let mut input = String::new();

    print!("> ");

    let _ = stdout().flush();

    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct input");

    input
}

fn main() {
    let args: Vec<String> = env::args().collect();

    loop {
        let input = read_input();

        // Lexing

        let tokens = lexer::lex(&input);
        if tokens.is_err() {
            eprintln!("Failed to tokenize input: {}", tokens.err().unwrap());
            continue;
        }
        let tokens = tokens.unwrap();

        if args.contains(&"--lex".to_string()) {
            println!("\nTokens:");
            println!("{:#?}", tokens);
        }

        // Parsing

        let ast = parser::parse(tokens);
        if ast.is_err() {
            eprintln!("Failed to create ast: {}", ast.err().unwrap());
            continue;
        }
        let ast = ast.unwrap();

        if args.contains(&"--ast".to_string()) {
            println!("\nAst:");
            println!("{:#?}", ast);
        }

        // Jit execution
        let result = backend::x86_backend::compile_and_run(&ast);
        if result.is_err() {
            eprintln!("Failed to compile and run ast: {}", result.err().unwrap());
            continue;
        }
        let result = result.unwrap();

        println!("=> {}", result);
    }
}
