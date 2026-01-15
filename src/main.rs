mod scanner;
mod token;
mod errors;


use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use crate::scanner::Scanner;


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
    let source = fs::read_to_string(path)
        .expect("Failed to read file");
    run(&source);

    if errors::had_error() {
        process::exit(65)
    }

}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        run(&line);
        errors::reset_error();      
    }
}

fn run (source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

