use crate::expression::{Expr, ExprVisitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal{ value } => { value.to_string() }
            _ => { panic!("Unknown expression type") }
        }
    }
}
