use crate::{expression, token::LiteralType};

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
        None
    }

    fn visit_binary(&mut self, binary: &expression::Binary) -> Option<LiteralType> {
        None
    }
}