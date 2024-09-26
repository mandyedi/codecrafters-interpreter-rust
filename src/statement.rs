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

pub enum Statement {
    Print(Print)
}

impl Statement {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Statement::Print(print) => visitor.visit_print(print),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_print(&self, print: &Print) -> Self::Output;
}