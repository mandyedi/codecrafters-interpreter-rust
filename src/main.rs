mod token;
mod scanner;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use scanner::Scanner;
// use token::Token;

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

                // let result = tokenize(&file_contents);
                // exit(result);

                let mut scanner = Scanner::new(file_contents);
                let mut had_error = false;
                let tokens = scanner.scan_tokens(&mut had_error);
                for token in tokens {
                    println!("{}", token);
                }

                if had_error {
                    exit(65);
                }
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
