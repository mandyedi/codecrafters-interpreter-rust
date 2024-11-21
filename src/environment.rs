use std::{collections::HashMap, cell::RefCell, rc::Rc};
use crate::{interpreter::{RuntimeException, RuntimeError}, token::{LiteralType, Token}};

#[derive(Debug, PartialEq)]
pub struct Environment {
    values: RefCell<HashMap<String, Option<LiteralType>>>,
    pub enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<&Rc<Environment>>) -> Self {
        Self {
            values: RefCell::new(HashMap::new()),
            enclosing: enclosing.map(|e| Rc::clone(e)),
        }
    }

    pub fn define(&self, name: String, value: Option<LiteralType>) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn assign(&self, name: &Token, value: Option<LiteralType>) -> Result<(), RuntimeException> {
        let mut value_ref = self.values.borrow_mut();
        if value_ref.contains_key(&name.lexeme) {
            value_ref.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().assign(name, value);
        }

        return Err(RuntimeException::RuntimeError(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme).as_str())));
    }

    pub fn get(&self, name: &Token) -> Result<Option<LiteralType>, RuntimeException> {
        let value_ref = self.values.borrow();
        if value_ref.contains_key(&name.lexeme) {
            return Ok(value_ref.get(&name.lexeme).unwrap().clone());
        }

        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(name);
        }
        
        return Err(RuntimeException::RuntimeError(RuntimeError::new(name, format!("Undefined variable '{}'", name.lexeme).as_str())));
    }
}