use crate::expression::Expr;

pub struct Print {
    pub expression: Box::<Expr>,
}

impl Print {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

pub struct Expression {
    pub expression: Box<Expr>,
}

impl Expression {
    pub fn new (expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

pub enum Statement {
    Print(Print),
    Expression(Expression),
}

impl Statement {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Statement::Print(print) => visitor.visit_print(print),
            Statement::Expression(expression) => visitor.visit_expression(expression),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_print(&mut self, print: &Print) -> Self::Output;
    fn visit_expression(&mut self, expression: &Expression) -> Self::Output;
}