use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // TODO: is it ok to return &Vec<Token>?
    pub fn scan_tokens(&mut self, had_error: & mut bool) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(had_error);
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_owned(), None, self.line));
        return &self.tokens;
    }

    fn scan_token(&mut self, had_error: &mut bool) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            '*' => self.add_token(TokenType::Star, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '=' => {
                let token_type = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token_type, None);
            }
            '!' => {
                let token_type = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_char('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => { self.line += 1 }
            '"' => { self.string(had_error);  }
            _ => {
                *had_error = true;
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
            },
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap_or('\0') != expected {
            return false;
        }

        self.current += 1;

        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap_or('\0');
    }

    fn string(&mut self, had_error: &mut bool) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // TODO: make had_error a member of Scanner
            *had_error = true;
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return;
        }

        self.advance();

        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token(TokenType::String, Some(value));
    }
    
}