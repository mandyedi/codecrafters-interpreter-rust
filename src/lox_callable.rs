use std::{rc::Rc, fmt::Display};
use crate::interpreter::{Interpreter, RuntimeException};
use crate::token::LiteralType;
use crate::statement::Function;
use crate::environment::Environment;

pub trait LoxCallable {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<LiteralType>>,
    ) -> Result<Option<LiteralType>, RuntimeException>;
    fn arity(&self) -> usize;
}

#[derive(Clone, PartialEq, Debug)]
pub enum LoxCallables {
    LoxFunction(Box<LoxFunction>),
    LoxAnonymous(Box<LoxAnonymous>),
}

impl Display for LoxCallables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxCallables::LoxAnonymous(_) => write!(f, "<anonymous function>"),
            LoxCallables::LoxFunction(function) => {
                write!(f, "<fn {}>", function.declaration.name.lexeme)
            }
        }
    }
}

impl LoxCallable for LoxCallables {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<LiteralType>>,
    ) -> Result<Option<LiteralType>, RuntimeException> {
        match self {
            LoxCallables::LoxFunction(value) => value.call(interpreter, arguments),
            LoxCallables::LoxAnonymous(value) => value.call(interpreter, arguments),
        }
    }

    fn arity(&self) -> usize {
        match self {
            LoxCallables::LoxFunction(value) => value.arity(),
            LoxCallables::LoxAnonymous(value) => value.arity(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LoxAnonymous {
    call_ref: fn(
        &mut Interpreter,
        Vec<Option<LiteralType>>,
    ) -> Result<Option<LiteralType>, RuntimeException>,
    arity_ref: fn() -> usize,
}

impl LoxAnonymous {
    pub fn new(
        call: fn(
            &mut Interpreter,
            Vec<Option<LiteralType>>,
        ) -> Result<Option<LiteralType>, RuntimeException>,
        arity: fn() -> usize,
    ) -> LoxAnonymous {
        LoxAnonymous {
            call_ref: call,
            arity_ref: arity,
        }
    }
}

impl LoxCallable for LoxAnonymous {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<LiteralType>>,
    ) -> Result<Option<LiteralType>, RuntimeException> {
        (self.call_ref)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        (self.arity_ref)()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LoxFunction {
    declaration: Function,
    closure: Rc<Environment>,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: Rc<Environment>) -> LoxFunction {
        LoxFunction {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<LiteralType>>,
    ) -> Result<Option<LiteralType>, RuntimeException> {
        let environment = Rc::new(Environment::new(Some(&self.closure)));
        for i in 0..self.declaration.params.len() {
            environment.define(
                self.declaration.params.get(i).unwrap().lexeme.clone(),
                arguments.get(i).unwrap().clone(),
            );
        }

        return interpreter
            .execute_block(&self.declaration.body, environment)
            .map(|_| None);
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
