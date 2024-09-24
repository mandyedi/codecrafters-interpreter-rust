use crate::{expression::{Expr, Literal}, token::{Token, TokenType, LiteralType}};
pub struct Parser {
    tokens: Box<[Token]>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Box<[Token]>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_single(&TokenType::True) {
            return Expr::Literal(Literal::new(Some(LiteralType::Boolean(true))));
        }
        
        if self.match_single(&TokenType::False) {
            return Expr::Literal(Literal::new(Some(LiteralType::Boolean(false))));
        }

        if self.match_single(&TokenType::Nil) {
            return Expr::Literal(Literal::new(None));
        }

        if self.match_many(&vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal(Literal::new(self.previous().literal.clone()));
        }

        // TODO: Implement error handling
        return Expr::Literal(Literal::new(None));
    }

    fn match_many(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_single(&mut self, token_type: &TokenType) -> bool {
        if self.check(&token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() <= self.current
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(& self) -> &Token {
        &self.tokens[self.current - 1]
    }

}
