use std::fmt::{self, Display};
use crate::lox_callable::LoxCallables;

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Star, Semicolon, Slash,

    // One or two character tokens
    Equal, EqualEqual,
    Bang, BangEqual,
    Less, LessEqual,
    Greater, GreaterEqual,

    // Literals
    String, Number, Identifier,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // End of file
    EOF
}

impl TokenType {
    fn as_str(&self) -> &'static str {
        match self {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Minus => "MINUS",
            TokenType::Plus => "PLUS",
            TokenType::Star => "STAR",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Slash => "SLASH",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::Fun => "FUN",
            TokenType::For => "FOR",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",
            TokenType::EOF => "EOF",
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
    LoxCallable(LoxCallables),
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Number(n) => write!(f, "{:?}", n),
            LiteralType::Boolean(b) => write!(f, "{}", b),
            LiteralType::LoxCallable(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        
        let literal = match literal {
            Some(literal) => Some(LiteralType::String(literal)),
            _ => None,
        };
        
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn new_number(token_type: TokenType, lexeme: String, literal: Option<f64>, line: usize) -> Self {
        
        let literal = match literal {
            Some(literal) => Some(LiteralType::Number(literal)),
            _ => None,
        };
        
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, 
            match &self.literal {
                Some(LiteralType::String(literal)) => format!("{}", literal),
                Some(LiteralType::Number(literal)) => format!("{:?}", literal),
                Some(LiteralType::Boolean(literal)) => format!("{}", literal),
                Some(LiteralType::LoxCallable(literal)) => format!("{}", literal),
                None => "null".to_owned(),
            }
        )
    }
}