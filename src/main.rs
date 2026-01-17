mod scanner;
mod token;
mod errors;
mod expr;
mod parser;
mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use crate::interpreter::Interpreter;
use crate::scanner::Scanner;
use crate::expr::{Expr, LiteralValue};
use crate::token::{Token, TokenType};
use crate::parser::Parser;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: shift [script]");
        process::exit(64);
    }else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str){
    let source = fs::read_to_string(path).expect("Failed to read file");
    let mut interpreter = Interpreter;

    run(&mut interpreter, &source);

    if errors::had_error() {
        process::exit(65)
    }

}

fn run_prompt() {
    let mut interpreter = Interpreter;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        run(&mut interpreter, &line);
        errors::reset_error();      
    }
}

fn run (interpreter: &mut Interpreter, source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens.clone());

    // DEBUG
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    match parser.parse() {
        Ok(expression) => {
            interpreter.interpret(&expression);
        },
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

