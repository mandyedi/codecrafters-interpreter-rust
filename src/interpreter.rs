use crate::{expression::{self, Variable}, runtime_error, statement, token::{LiteralType, Token, TokenType}, environment::Environment};

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> Self {
        return RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        };
    }
}

pub struct Interpreter {
    environment: Option<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Some(Environment::new(None)),
        }
    }

    pub fn interpret_expression(&mut self, expression: &expression::Expr) {
        let result = self.evaluate(expression);
        if result.is_ok() {
            println!("{}", self.stringify(&result.as_ref().unwrap()));
            return;
        }

        runtime_error(result.unwrap_err());
    }

    pub fn interpret(&mut self, statements: Vec<statement::Statement>) {
        let mut error: Option<RuntimeError> = None;
        for statement in statements {
            let result = self.execute(&statement);
            if result.is_err() {
                error = result.err();
                break;
            }
        }

        if error.is_some() {
            runtime_error(error.unwrap());
        }
    }

    fn execute(&mut self, statement: &statement::Statement) -> Result<(), RuntimeError> {
        statement.accept(self)?;
        return Ok(());
    }

    fn execute_block(&mut self, statements: &Vec<statement::Statement>) -> Result<(), RuntimeError> {
        self.environment = Some(Environment::new(self.environment.take()));

        let mut error: Result<(), RuntimeError> = Ok(());

        for statement in statements {
            let result = self.execute(statement);
            if result.is_err() {
                error = result;
                break;
            }
        }

        self.environment = self.environment.as_mut().unwrap().enclosing.take().map(|e| *e);

        return error;
    }

    fn evaluate(&mut self, expression: &expression::Expr) -> Result<Option<LiteralType>, RuntimeError> {
        expression.accept(self)
    }

    fn stringify(&self, value: &Option<LiteralType>) -> String {
        match value {
            Some(result) => {
                match result {
                    LiteralType::Number(_) => { return value.as_ref().unwrap().to_string().trim_end_matches(".0").to_string(); },
                    _ => { return result.to_string(); }
                }
            }
            None => { return "nil".to_string(); }
        }
    }

    fn is_truthy(&self, value: &Option<LiteralType>) -> bool {
        if value.is_none() {
            return false;
        }

        match value.as_ref().unwrap() {
            LiteralType::Boolean(value) => return *value,
            _ => return true,
        }
    }

    fn check_number_operand(&self, operator: &Token, operand: &Option<LiteralType>) -> Result<f64, RuntimeError> {
        match operand {
            Some(LiteralType::Number(value)) => return Ok(*value),
            _ => return Err(RuntimeError::new(operator, "Operand must be a number.")),
        }
    }

    fn check_number_operands(&self, operator: &Token, left: &Option<LiteralType>, right: &Option<LiteralType>) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (Some(LiteralType::Number(x)), Some(LiteralType::Number(y))) => {
                return Ok((*x, *y));
            }
            _ => { return Err(RuntimeError::new(operator, "Operand must be a number.")); }
        }
    }
}

impl expression::Visitor for Interpreter {
    type Output = Result<Option<LiteralType>, RuntimeError>;

    fn visit_literal(&mut self, literal: &expression::Literal) -> Self::Output {
        return Ok(literal.value.to_owned());
    }

    fn visit_grouping(&mut self, grouping: &expression::Grouping) -> Self::Output {
        return self.evaluate(&grouping.expression);
    }
    
    fn visit_unary(&mut self, unary: &expression::Unary) -> Self::Output {
        let right = self.evaluate(&unary.right)?;
        
        match unary.operator.token_type {
            TokenType::Minus => {
                let number = self.check_number_operand(&unary.operator, &right)?;
                return Ok(Some(LiteralType::Number(-number)));
            }
            TokenType::Bang => {
                return Ok(Some(LiteralType::Boolean(!self.is_truthy(&right))));
            }
            _ => return Ok(None),
        }
    }

    fn visit_binary(&mut self, binary: &expression::Binary) -> Self::Output {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.token_type {
            TokenType::Star => {
                let (left_number, right_number) = 
                    self.check_number_operands(&binary.operator, &left, &right)?;
                return Ok(Some(LiteralType::Number(left_number * right_number)));
            }
            TokenType::Slash => {
                let (left_number, right_number) = 
                    self.check_number_operands(&binary.operator, &left, &right)?;
                return Ok(Some(LiteralType::Number(left_number / right_number)));
            }
            TokenType::Plus => {
                match (left, right) {
                    (Some(LiteralType::Number(x)), Some(LiteralType::Number(y))) => {
                        return Ok(Some(LiteralType::Number(x + y)));
                    },
                    (Some(LiteralType::String(x)), Some(LiteralType::String(y))) => {
                        return Ok(Some(LiteralType::String(format!("{}{}", x, y))));
                    },
                    _ => {},
                }

                return Err(RuntimeError::new(&binary.operator, "Operands must be two numbers or two strings."));
            }
            TokenType::Minus => {
                let (left_number, right_number) = 
                    self.check_number_operands(&binary.operator, &left, &right)?;
                return Ok(Some(LiteralType::Number(left_number - right_number)));
            }
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                let (left_number, right_number) = 
                    self.check_number_operands(&binary.operator, &left, &right)?;
                match binary.operator.token_type {
                    TokenType::Greater => return Ok(Some(LiteralType::Boolean(left_number > right_number))),
                    TokenType::GreaterEqual => return Ok(Some(LiteralType::Boolean(left_number >= right_number))),
                    TokenType::Less => return Ok(Some(LiteralType::Boolean(left_number < right_number))),
                    TokenType::LessEqual => return Ok(Some(LiteralType::Boolean(left_number <= right_number))),
                    _ => return Ok(None),
                }
            }
            TokenType::EqualEqual => {
                return Ok(Some(LiteralType::Boolean(left == right)));
            }
            TokenType::BangEqual => {
                return Ok(Some(LiteralType::Boolean(left != right)));
            }
            _ => return Err(RuntimeError::new(&binary.operator, "Invalid operator when evaluating binary.")),
        }
    }

    fn visit_variable(&mut self, variable: &Variable) -> Self::Output {
        return Ok(self.environment.as_ref().unwrap().get(&variable.name)?.clone());
    }

    fn visit_assign(&mut self, assign: &expression::Assign) -> Self::Output {
        let value = self.evaluate(&assign.value)?;
        self.environment.as_mut().unwrap().assign(&assign.name, value.clone())?;
        return Ok(value);
    }

    fn visit_logical(&mut self, logical: &expression::Logical) -> Self::Output {
        let left = self.evaluate(&logical.left)?;

        let left_truthy = self.is_truthy(&left);
        match logical.operator.token_type {
            TokenType::Or => {
                if left_truthy {
                    return Ok(left);
                }
            }
            TokenType::And => {
                if !left_truthy {
                    return Ok(left);
                }
            }
            _ => {
                return Err(RuntimeError::new(&logical.operator, "Invalid operator when evaluating logical."))
            }
        }

        return self.evaluate(&logical.right);
    }
}

impl statement::Visitor for Interpreter {
    type Output = Result<(), RuntimeError>;

    fn visit_print(&mut self, print: &statement::Print) -> Self::Output {
        let value = self.evaluate(&print.expression)?;
        println!("{}", self.stringify(&value));
        return Ok(());
    }

    fn visit_expression(&mut self, expression: &statement::Expression) -> Self::Output {
        self.evaluate(&expression.expression)?;
        return Ok(());
    }

    fn visit_var(&mut self, var: &statement::Var) -> Self::Output {
        let mut value: Option<LiteralType> = None;

        if var.initializer.is_some() {
            value = self.evaluate(var.initializer.as_ref().unwrap())?;
        }

        self.environment.as_mut().unwrap().define(var.name.lexeme.clone(), value);

        return Ok(());
    }

    fn visit_block(&mut self, block: &statement::Block) -> Self::Output {
        self.execute_block(&block.statements)?;
        return Ok(());
    }

    fn visit_if(&mut self, if_statement: &statement::If) -> Self::Output {
        let value = self.evaluate(&if_statement.condition)?;
        if self.is_truthy(&value) {
            self.execute(&if_statement.then_branch)?;
        } else if if_statement.else_branch.is_some() {
            self.execute(if_statement.else_branch.as_ref().unwrap())?;
        }

        return Ok(());
    }
}