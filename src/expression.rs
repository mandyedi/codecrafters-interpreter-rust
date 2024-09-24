use crate::token::LiteralType;

pub enum Expr {
    Literal {
        value: LiteralType,
    },
}

impl Expr {
    pub fn wrap(self) -> Self {
        self
    }

    pub fn accept<R>(&self, visitor: &mut impl ExprVisitor<R>) -> R {
        visitor.visit(self)
    }
}

pub trait ExprVisitor<R> {
    fn visit(&mut self, expr: &Expr) -> R;
}