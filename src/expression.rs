use crate::token::{LiteralType, Token};

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

impl Expr {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Variable(variable) => visitor.visit_variable(variable),
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Logical(logical) => visitor.visit_logical(logical),
            Expr::Call(call) => visitor.visit_call(call),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;
    fn visit_binary(&mut self, binary: &Binary) -> Self::Output;
    fn visit_variable(&mut self, variable: &Variable) -> Self::Output;
    fn visit_assign(&mut self, assign: &Assign) -> Self::Output;
    fn visit_logical(&mut self, logical: &Logical) -> Self::Output;
    fn visit_call(&mut self, call: &Call) -> Self::Output;
}