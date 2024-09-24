use crate::{expression::*, token::*};
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
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_many(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();

            expr = Expr::Binary(Binary::new(expr, operator, right));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_many(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();

            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_many(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(Unary::new(operator, right));
        }
        
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
        
        if self.match_single(&TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(&TokenType::RightParen);
            return Expr::Grouping(Grouping::new(expr));
        }

        // TODO: Implement error handling
        return Expr::Literal(Literal::new(None));
    }

    fn consume(&mut self, token_type: &TokenType) {
        // TODO: Implement error handling
        if self.check(&token_type) {
            self.advance();
        }
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
