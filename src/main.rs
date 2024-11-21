mod token;
mod scanner;
mod parser;
mod expression;
mod ast_printer;
mod interpreter;
mod statement;
mod environment;
mod lox_callable;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use scanner::Scanner;
use parser::Parser;
use ast_printer::AstPrinter;
use statement::Statement;
use token::Token;
use token::TokenType;
use expression::Expr;
use interpreter::Interpreter;
use interpreter::RuntimeError;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

pub fn error(line: usize, message: String) {
    report(line, "", message);
}

pub fn error_token(token: &token::Token, message: String) {
    if token.token_type == TokenType::EOF {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

pub fn runtime_error(error: RuntimeError) {
    eprintln!("{}\n[line {}]", error.message, error.token.line);
    unsafe { HAD_RUNTIME_ERROR = true };
}

fn report(line: usize, location: &str, message: String) {
    unsafe { HAD_ERROR = true; }
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

struct Lox {
}

impl Lox {
    fn run_file(&self, args: &Vec<String>) {
        let command = &args[1];
        let filename = &args[2];

        match command.as_str() {
            "tokenize" => {
                let tokens = self.tokenize(filename);

                for token in tokens {
                    println!("{}", token);
                }

                if unsafe { HAD_ERROR } {
                    exit(65);
                }
            }
            "parse" => {
                let expr = self.parse_expression(filename);

                if unsafe { HAD_ERROR } {
                    exit(65);
                }

                let mut ast_printer = AstPrinter::new();
                println!("{}", ast_printer.print(expr.as_ref().unwrap()));
            }
            "evaluate" => {
                let expr = self.parse_expression(filename);

                if unsafe { HAD_ERROR } {
                    exit(65);
                }

                let mut interpreter = Interpreter::new();
                interpreter.interpret_expression(expr.as_ref().unwrap());

                if unsafe { HAD_RUNTIME_ERROR } {
                    exit(70);
                }
            }
            "run" => {
                let statements = self.parse(filename);

                if unsafe { HAD_ERROR } {
                    exit(65);
                }

                let mut interpreter = Interpreter::new();
                interpreter.interpret(statements);

                if unsafe { HAD_RUNTIME_ERROR } {
                    exit(70);
                }
            }
            _ => {
                writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
                return;
            }
        }
    }

    fn read_file(&self, filename: &str) -> String {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
            String::new()
        });
        return file_contents;
    }

    fn tokenize(&self, filename: &str) -> Vec<Token> {
        let source = self.read_file(filename);

        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        return scanner.tokens;
    }

    fn parse_expression(&self, filename: &str) -> Option<Expr> {
        let tokens = self.tokenize(filename);
        let tokens = tokens.into_boxed_slice();
        let mut parser = Parser::new(tokens);

        return parser.parse_expression();
    }

    fn parse(&self, filename: &str) -> Vec<Statement> {
        let tokens = self.tokenize(filename);
        let tokens = tokens.into_boxed_slice();
        let mut parser = Parser::new(tokens);
        
        return parser.parse();
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
