use crate::{expression::Expr, token::Token};

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

pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>
}

impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self {
            name,
            initializer: initializer.map(|i| Box::new(i)),
        }
    }
}

pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

pub enum Statement {
    Print(Print),
    Expression(Expression),
    Var(Var),
    Block(Block),
}

impl Statement {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Statement::Print(print) => visitor.visit_print(print),
            Statement::Expression(expression) => visitor.visit_expression(expression),
            Statement::Var(var) => visitor.visit_var(var),
            Statement::Block(block) => visitor.visit_block(block),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_print(&mut self, print: &Print) -> Self::Output;
    fn visit_expression(&mut self, expression: &Expression) -> Self::Output;
    fn visit_var(&mut self, var: &Var) -> Self::Output;
    fn visit_block(&mut self, block: &Block) -> Self::Output;
}