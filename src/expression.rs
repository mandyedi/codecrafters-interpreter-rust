use crate::token::{LiteralType, Token};

pub struct Literal {
    pub value: Option<LiteralType>,
}

impl Literal {
    pub fn new(value: Option<LiteralType>) -> Self {
        Self {
            value,
        }
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
}

impl Expr {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;
}