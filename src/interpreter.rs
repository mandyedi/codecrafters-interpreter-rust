use std::{rc::Rc, time::{SystemTime, UNIX_EPOCH}};
use crate::{environment::Environment, expression::{self, Variable}, runtime_error, statement::{self}, token::{LiteralType, Token, TokenType}};
use crate::lox_callable::{LoxFunction, LoxAnonymous, LoxCallables, LoxCallable};

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

pub struct Return {
    pub value: Option<LiteralType>,
}

impl Return {
    pub fn new(value: Option<LiteralType>) -> Self {
        Self { value }
    }
}

pub enum RuntimeException {
    RuntimeError(RuntimeError),
    Return(Return),
}

pub struct Interpreter {
    globals: Rc<Environment>,
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(Environment::new(None));

        globals.define("clock".to_owned(),
            Some(LiteralType::LoxCallable(LoxCallables::LoxAnonymous(
                Box::new(LoxAnonymous::new(|_interpreter, _arguments| {
                    Ok(Some(LiteralType::Number(
                        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
                    )))
                }, || 0,
                )),
            ))),
        );

        let environment = Rc::clone(&globals);

        Self {
            globals,
            environment,
        }
    }

    pub fn interpret_expression(&mut self, expression: &expression::Expr) {
        let result = self.evaluate(&Box::new(expression));
        if result.is_ok() {
            println!("{}", self.stringify(&result.ok().unwrap()));
            return;
        }

        match result.unwrap_err() {
            RuntimeException::RuntimeError(run_error) => runtime_error(run_error),
            _ => {}
        }
    }

    pub fn interpret(&mut self, statements: Vec<statement::Statement>) {
        let mut error: Option<RuntimeException> = None;
        for statement in statements {
            let result = self.execute(&statement);
            if result.is_err() {
                error = result.err();
                break;
            }
        }

        if error.is_some() {
            match error.unwrap() {
                RuntimeException::RuntimeError(run_error) => runtime_error(run_error),
                _ => {}
            }
        }
    }

    fn execute(&mut self, statement: &statement::Statement) -> Result<(), RuntimeException> {
        statement.accept(self)?;
        return Ok(());
    }

    pub fn execute_block(&mut self, statements: &Vec<statement::Statement>, environment: Rc<Environment>) -> Result<(), RuntimeException> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;

        let mut error: Result<(), RuntimeException> = Ok(());

        for statement in statements {
            let result = self.execute(statement);
            if result.is_err() {
                error = result;
                break;
            }
        }

        self.environment = previous;

        return error;
    }

    fn evaluate(&mut self, expression: &expression::Expr) -> Result<Option<LiteralType>, RuntimeException> {
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

    fn check_number_operand(&self, operator: &Token, operand: &Option<LiteralType>) -> Result<f64, RuntimeException> {
        match operand {
            Some(LiteralType::Number(value)) => return Ok(*value),
            _ => return Err(RuntimeException::RuntimeError(RuntimeError::new(operator, "Operand must be a number."))),
        }
    }

    fn check_number_operands(&self, operator: &Token, left: &Option<LiteralType>, right: &Option<LiteralType>) -> Result<(f64, f64), RuntimeException> {
        match (left, right) {
            (Some(LiteralType::Number(x)), Some(LiteralType::Number(y))) => {
                return Ok((*x, *y));
            }
            _ => { return Err(RuntimeException::RuntimeError(RuntimeError::new(operator, "Operand must be a number."))); }
        }
    }
}

impl expression::Visitor for Interpreter {
    type Output = Result<Option<LiteralType>, RuntimeException>;

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

                return Err(RuntimeException::RuntimeError(RuntimeError::new(&binary.operator, "Operands must be two numbers or two strings.")));
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
            _ => return Err(RuntimeException::RuntimeError(RuntimeError::new(&binary.operator, "Invalid operator when evaluating binary."))),
        }
    }

    fn visit_variable(&mut self, variable: &Variable) -> Self::Output {
        return Ok(self.environment.get(&variable.name)?);
    }

    fn visit_assign(&mut self, assign: &expression::Assign) -> Self::Output {
        let value = self.evaluate(&assign.value)?;
        self.environment.assign(&assign.name, value.clone())?;
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
                return Err(RuntimeException::RuntimeError(RuntimeError::new(&logical.operator, "Invalid operator when evaluating logical.")))
            }
        }

        return self.evaluate(&logical.right);
    }

    fn visit_call(&mut self, call: &expression::Call) -> Self::Output {
        let callee = self.evaluate(&call.callee)?;

        let mut arguments = Vec::new();
        for argument in &call.arguments {
            arguments.push(self.evaluate(&Box::new(argument))?);
        }

        let mut function = match callee {
            Some(LiteralType::LoxCallable(callable)) => Ok(callable),
            _ => Err(RuntimeException::RuntimeError(RuntimeError::new(&call.paren, "Can only call functions and classes.")))
        }?;

        if arguments.len() != function.arity() {
            return Err(RuntimeException::RuntimeError(RuntimeError::new(&call.paren, &format!("Expected {} arguments but got {}.", function.arity(), arguments.len()))));
        }

        let result = function.call(self, arguments);
        return match result {
            Err(RuntimeException::Return(r#return)) => Ok(r#return.value),
            _ => result,
        };
    }
}

impl statement::Visitor for Interpreter {
    type Output = Result<(), RuntimeException>;

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

        self.environment.define(var.name.lexeme.clone(), value);

        return Ok(());
    }

    fn visit_block(&mut self, block: &statement::Block) -> Self::Output {
        let result = self.execute_block(&block.statements, Rc::new(Environment::new(Some(&self.environment))));
        return result;
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

    fn visit_while(&mut self, while_statement: &statement::While) -> Self::Output {
        let mut value = self.evaluate(&while_statement.condition)?;
        while self.is_truthy(&value) {
            self.execute(&while_statement.body)?;
            value = self.evaluate(&while_statement.condition)?;
        }

        return Ok(());
    }

    fn visit_function(&mut self, function: &statement::Function) -> Self::Output {
        let value = Some(LiteralType::LoxCallable(LoxCallables::LoxFunction(
            Box::new(LoxFunction::new(function.clone(), Rc::clone(&self.environment),)),
        )));
        self.environment.define(function.name.lexeme.clone(), value);
        return Ok(());
    }
}