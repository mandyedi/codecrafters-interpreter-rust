use crate::{runtime_error, expression, token::{LiteralType, TokenType, Token}};

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

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_expression(&mut self, expression: &expression::Expr) {
        let result = self.evaluate(expression);
        if result.is_ok() {
            println!("{}", self.stringify(&result.as_ref().unwrap()));
            return;
        }

        runtime_error(result.unwrap_err());
    }

    pub fn interpret(&mut self) {}

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

    fn is_truthy(&self, value: Option<LiteralType>) -> bool {
        if value.is_none() {
            return false;
        }

        match value.unwrap() {
            LiteralType::Boolean(value) => return value,
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
                return Ok(Some(LiteralType::Boolean(!self.is_truthy(right))));
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
            _ => return Ok(None),
        }
    }
}