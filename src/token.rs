use std::fmt::Display;

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
    String, Number,

    // Keywords

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
            TokenType::EOF => "EOF",
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub enum LiteralType {
    String(String),
    Number(f64),
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Number(n) => write!(f, "{}", n),
        }
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    #[allow(dead_code)]
    line: usize,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, 
            match &self.literal {
                Some(LiteralType::String(literal)) => format!("{}", literal),
                Some(LiteralType::Number(literal)) => format!("{:?}", literal),
                None => "null".to_owned(),
            }
        )
    }
}