use crate::{error_token, statement::{Statement, Print, Expression, Var, Block, If}, expression::*, token::*};

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

    pub fn parse_expression(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let statement = self.declaration();
            if statement.is_ok() {
                statements.push(statement.ok().unwrap());
            } else {
                self.synchronize();
            }
        }
        return statements;
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if self.match_single(&TokenType::Var) {
            return Ok(self.var_declaration()?);
        }

        return Ok(self.statement()?);
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?.clone();

        let mut initializer: Option<Expr> = None;
        if self.match_single(&TokenType::Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration.")?;

        return Ok(Statement::Var(Var::new(name, initializer)));
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_single(&TokenType::If) {
            return Ok(self.if_statement()?);
        }

        if self.match_single(&TokenType::Print) {
            return Ok(self.print_statement()?);
        }

        if self.match_single(&TokenType::LeftBrace) {
            return Ok(Statement::Block(Block::new(self.block()?)));
        }
        
        return Ok(self.expression_statement()?);
    }

    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;
        
        let then_branch = self.statement()?;
        let mut else_branch: Option<Statement> = None;
        if self.match_single(&TokenType::Else) {
            else_branch = Some(self.statement()?);
        }

        return Ok(Statement::If(If::new(condition, then_branch, else_branch)));
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        return Ok(Statement::Print(Print::new(value)));
    }

    fn block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements: Vec<Statement> = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        return Ok(statements);
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        return Ok(Statement::Expression(Expression::new(expression)));
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_single(&TokenType::Equal) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(variable) => {
                    return Ok(Expr::Assign(Assign::new(variable.name, value)));
                }
                _ => {}
            }

            self.error(&equals, "Invalid assignment target.".to_string());
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        
        while self.match_single(&TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Logical::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        
        while self.match_single(&TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Logical::new(expr, operator, right));
        }

        return Ok(expr);
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

        if self.match_single(&TokenType::Identifier) {
            return Ok(Expr::Variable(Variable::new(self.previous().clone())));
        }

        return Err(self.error(self.peek(), "Expect expression.".to_string()));
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {
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
        self.peek().token_type == TokenType::EOF
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => { self.advance(); }
            }
        }
    }
}
