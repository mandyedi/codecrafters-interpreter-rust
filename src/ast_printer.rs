use crate::expression::{self, Expr, Literal, Grouping};

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
        if literal.value.is_none() {
            return "nil".to_string();
        }

        return literal.value.as_ref().unwrap().to_string();
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        return format!("(group {})", grouping.expression.accept(self));
    }
}

