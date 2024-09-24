use crate::token::LiteralType;

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

pub enum Expr {
    Literal(Literal),
}

impl Expr {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_literal(&mut self, expr: &Literal) -> Self::Output;
}