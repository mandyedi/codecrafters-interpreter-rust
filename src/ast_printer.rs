use crate::expression::*;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &Vec<&Expr>) -> String {
        let mut result = format!("({}", name);
        for expr in exprs.iter() {
            result.push_str(&format!(" {}", expr.accept(self)));
        }
        result.push_str(")");
        return result;
    }
}

impl Visitor for AstPrinter {
    type Output = String;

    fn visit_literal(&mut self, literal: &Literal) -> String {
        if literal.value.is_none() {
            return "nil".to_string();
        }

        return literal.value.as_ref().unwrap().to_string();
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        return self.parenthesize("group", &vec![&grouping.expression]);
    }

    fn visit_unary(&mut self, unary: &Unary) -> String {
        return self.parenthesize(&unary.operator.lexeme, &vec![&unary.right]);
    }

    fn visit_binary(&mut self, binary: &Binary) -> String {
        return self.parenthesize(&binary.operator.lexeme, &vec![&binary.left, &binary.right]);
    }

    fn visit_variable(&mut self, variable: &Variable) -> Self::Output {
        return variable.name.lexeme.clone();
    }
}

