use crate::token::Token;

pub struct Parser {
    #[allow(dead_code)]
    tokens: Box<[Token]>,
}

impl Parser {
    pub fn new(tokens: Box<[Token]>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse(&mut self) {
        // TODO: implement the parser
        println!("Parsing not implemented yet");
    }
}
