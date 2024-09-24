use crate::expression::{self, Expr, Literal};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl expression::Visitor for AstPrinter {
    type Output = String;

    fn visit_literal(&mut self, literal: &Literal) -> String {
        literal.value.to_string()
    }
}

