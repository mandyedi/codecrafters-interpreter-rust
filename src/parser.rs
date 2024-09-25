use crate::{error_token, expression::*, token::*};

pub struct ParseError {}

impl ParseError {
    pub fn new() -> Self {
        Self {}
    }
}

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

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        
        while self.match_many(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_many(&vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_many(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary(Binary::new(expr, operator, right));
        }
        
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_many(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_many(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary::new(operator, right)));
        }
        
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_single(&TokenType::True) {
            return Ok(Expr::Literal(Literal::new(Some(LiteralType::Boolean(true)))));
        }
        
        if self.match_single(&TokenType::False) {
            return Ok(Expr::Literal(Literal::new(Some(LiteralType::Boolean(false)))));
        }

        if self.match_single(&TokenType::Nil) {
            return Ok(Expr::Literal(Literal::new(None)));
        }

        if self.match_many(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(Literal::new(self.previous().literal.clone())));
        }
        
        if self.match_single(&TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping::new(expr)));
        }

        return Err(self.error(self.peek(), "Expect expression.".to_string()));
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {
        // TODO: Implement error handling
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message.to_string()))
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

    fn error(&self, token: &Token, message: String) -> ParseError {
        error_token(token, message);
        return ParseError::new();
    }
}
