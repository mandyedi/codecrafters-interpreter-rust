use crate::{expression::Expr, token::Token};

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Statement, else_branch: Option<Statement>) -> Self {
        Self { 
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|e| Box::new(e)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Statement>,
}

impl While {
    pub fn new(condition: Expr, body: Statement) -> Self {
        Self {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Statement>) -> Self {
        Self { name, params, body }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Expr>,
}
impl Return {
    pub fn new(keyword: Token, value: Option<Expr>) -> Self {
        Self {
            keyword,
            value: value,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Print(Print),
    Expression(Expression),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
    Function(Function),
    Return(Return),
}

impl Statement {
    pub fn accept<T: Visitor>(&self, visitor: &mut T) -> T::Output {
        return match self {
            Statement::Print(print) => visitor.visit_print(print),
            Statement::Expression(expression) => visitor.visit_expression(expression),
            Statement::Var(var) => visitor.visit_var(var),
            Statement::Block(block) => visitor.visit_block(block),
            Statement::If(if_statement) => visitor.visit_if(if_statement),
            Statement::While(while_statement) => visitor.visit_while(while_statement),
            Statement::Function(function_statement) => visitor.visit_function(function_statement),
            Statement::Return(return_statement) => visitor.visit_return(return_statement),
        };
    }
}

pub trait Visitor {
    type Output;
    fn visit_print(&mut self, print: &Print) -> Self::Output;
    fn visit_expression(&mut self, expression: &Expression) -> Self::Output;
    fn visit_var(&mut self, var: &Var) -> Self::Output;
    fn visit_block(&mut self, block: &Block) -> Self::Output;
    fn visit_if(&mut self, if_statement: &If) -> Self::Output;
    fn visit_while(&mut self, while_statement: &While) -> Self::Output;
    fn visit_function(&mut self, function_statement: &Function) -> Self::Output;
    fn visit_return(&mut self, return_statement: &Return) -> Self::Output;
}