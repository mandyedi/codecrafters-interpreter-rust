use std::collections::HashMap;
use crate::{interpreter::RuntimeError, token::{LiteralType, Token}};

pub struct Environment {
    values: HashMap<String, Option<LiteralType>>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing.map(|e| Box::new(e)),
        }
    }

    pub fn define(&mut self, name: String, value: Option<LiteralType>) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Option<LiteralType>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if self.enclosing.is_some() {
            return self.enclosing.as_mut().unwrap().assign(name, value);
        }

        return Err(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme).as_str()));
    }

    pub fn get(&self, name: &Token) -> Result<&Option<LiteralType>, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap());
        }

        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(name);
        }
        
        return Err(RuntimeError::new(name, format!("Undefined variable '{}'", name.lexeme).as_str()));
    }
}