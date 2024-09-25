use crate::{expression, token::{LiteralType, TokenType}};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expression: &expression::Expr) {
        let result = self.evaluate(expression);
        match result {
            Some(result) => {
                match result {
                    LiteralType::Number(_) => println!("{}", result.to_string().trim_end_matches(".0")),
                    _ => println!("{}", result)
                }
            }
            None => println!("nil"),
        }
    }

    fn evaluate(&mut self, expression: &expression::Expr) -> Option<LiteralType> {
        expression.accept(self)
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
}

impl expression::Visitor for Interpreter {
    type Output = Option<LiteralType>;

    fn visit_literal(&mut self, literal: &expression::Literal) -> Option<LiteralType> {
        return literal.value.to_owned();
    }

    fn visit_grouping(&mut self, grouping: &expression::Grouping) -> Option<LiteralType> {
        return self.evaluate(&grouping.expression);
    }
    
    fn visit_unary(&mut self, unary: &expression::Unary) -> Option<LiteralType> {
        let right = self.evaluate(&unary.right);
        
        match unary.operator.token_type {
            TokenType::Minus => {
                let value = right.unwrap().to_string().parse::<f64>().unwrap();
                return Some(LiteralType::Number(-value))
            }
            TokenType::Bang => {
                return Some(LiteralType::Boolean(!self.is_truthy(right)));
            }
            _ => return None,
        }
    }

    fn visit_binary(&mut self, binary: &expression::Binary) -> Option<LiteralType> {
        let left = self.evaluate(&binary.left);
        let right = self.evaluate(&binary.right);

        match binary.operator.token_type {
            TokenType::Star => {
                let x: f64 = left.unwrap().to_string().parse::<f64>().unwrap();
                let y: f64 = right.unwrap().to_string().parse::<f64>().unwrap();
                return Some(LiteralType::Number(x * y));
            }
            TokenType::Slash => {
                let x: f64 = left.unwrap().to_string().parse::<f64>().unwrap();
                let y: f64 = right.unwrap().to_string().parse::<f64>().unwrap();
                return Some(LiteralType::Number(x / y));
            }
            TokenType::Plus => {
                match (left, right) {
                    (Some(LiteralType::Number(x)), Some(LiteralType::Number(y))) => {
                        return Some(LiteralType::Number(x + y));
                    },
                    (Some(LiteralType::String(x)), Some(LiteralType::String(y))) => {
                        return Some(LiteralType::String(format!("{}{}", x, y)));
                    },
                    _ => return None,
                }
            }
            TokenType::Minus => {
                let x: f64 = left.unwrap().to_string().parse::<f64>().unwrap();
                let y: f64 = right.unwrap().to_string().parse::<f64>().unwrap();
                return Some(LiteralType::Number(x - y));
            }
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                let x: f64 = left.unwrap().to_string().parse::<f64>().unwrap();
                let y: f64 = right.unwrap().to_string().parse::<f64>().unwrap();
                match binary.operator.token_type {
                    TokenType::Greater => return Some(LiteralType::Boolean(x > y)),
                    TokenType::GreaterEqual => return Some(LiteralType::Boolean(x >= y)),
                    TokenType::Less => return Some(LiteralType::Boolean(x < y)),
                    TokenType::LessEqual => return Some(LiteralType::Boolean(x <= y)),
                    _ => return None,
                }
            }
            _ => return None,
        }
    }
}