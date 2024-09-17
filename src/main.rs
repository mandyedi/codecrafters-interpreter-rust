use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let result = tokenize(&file_contents);
            exit(result);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(input: &str) -> i32 {
    let mut result = 0;
    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        match char {
            ' ' | '\r' | '\t' | '\n' => {
                continue;
            }
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            '+' => println!("PLUS + null"),
            '*' => println!("STAR * null"),
            ';' => println!("SEMICOLON ; null"),
            '=' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("EQUAL_EQUAL == null");
                    chars.next();
                } else {
                    println!("EQUAL = null");
                }
            }
            '!' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("BANG_EQUAL != null");
                    chars.next();
                } else {
                    println!("BANG ! null");
                }
            }
            '<' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("LESS_EQUAL <= null");
                    chars.next();
                } else {
                    println!("LESS < null");
                }
            }
            '>' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("GREATER_EQUAL >= null");
                    chars.next();
                } else {
                    println!("GREATER > null");
                }
            }
            '/' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('/') {
                    break;
                } else {
                    println!("SLASH / null");
                }
            }
            _ => {
                    eprintln!("[line 1] Error: Unexpected character: {}", char);
                result = 65;
            }
        }
    }
    println!("EOF  null");
    return result;
}