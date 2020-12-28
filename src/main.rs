#![allow(dead_code)]

mod ast;
mod elf;
mod error;
mod generator;
mod jit;
mod lexer;
mod parser;
mod types;

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

        let assembly = generator::gen_assembly(ast);
        if assembly.is_err() {
            eprintln!("Failed to generate assembly: {}", assembly.err().unwrap());
            continue;
        }
        let assembly = assembly.unwrap();

        if args.contains(&"--cg".to_string()) {
            println!("\nCode gen ({} bytes): ", assembly.len());
            println!("{:02X?}\n", assembly);
        }

        let jit = jit::Jit::new(&assembly);
        let result = jit.run();
        println!("=> {}", result);
    }
}
