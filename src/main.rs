use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

mod error;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

use expr::{Expr, Literal};
use parser::Parser;
use scanner::Scanner;
use token::Token;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() > 2 {
        println!("Usage: rslox [script]");
        process::exit(64);
    }

    init();

    if args.len() == 2 {
        let file_path = &args[1];
        run_file(file_path);
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn init() {
    scanner::init();
}

fn run_file(file_path: &String) {
    let Ok(mut file) = File::open(file_path) else {
        println!("File not found {}", file_path);
        process::exit(64);
    };
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    run(source);
}

fn run_prompt() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    loop {
        print!("> ");
        stdout.flush()?;
        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line)?;

        if bytes_read == 0 {
            break;
        }

        run(line);
    }

    println!();

    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
}
