mod token;
mod scanner;
mod parser;
mod expression;
mod ast_printer;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use scanner::Scanner;
use parser::Parser;
use ast_printer::AstPrinter;

struct Lox {
}

impl Lox {
    fn run_file(&self, args: &Vec<String>) {
        let command = &args[1];
        let filename = &args[2];

        match command.as_str() {
            "tokenize" => {
                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });

                let mut scanner = Scanner::new(file_contents);
                let mut had_error = false;
                scanner.scan_tokens(&mut had_error);
                for token in scanner.tokens {
                    println!("{}", token);
                }

                if had_error {
                    exit(65);
                }
            }
            "parse" => {
                let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });

                let mut scanner = Scanner::new(file_contents);
                let mut had_error = false;
                scanner.scan_tokens(&mut had_error);

                let tokens = scanner.tokens.into_boxed_slice();
                let mut parser = Parser::new(tokens);
                let expr = parser.parse();

                if had_error {
                    exit(65);
                }

                let mut ast_printer = AstPrinter::new();
                println!("{}", ast_printer.print(&expr));
            }
            _ => {
                writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
                return;
            }
        }
    }

    fn _error(&mut self, line: u32, message: &str) {
        self._report(line, "", message);
    }

    fn _report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {}] Error: {}: {}", line, location, message);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let lox = Lox {};
    lox.run_file(&args);
}
