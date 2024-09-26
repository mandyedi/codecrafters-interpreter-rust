use std::collections::HashMap;
use crate::{interpreter::RuntimeError, token::{LiteralType, Token}};

pub struct Environment {
    values: HashMap<String, Option<LiteralType>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<LiteralType>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Option<LiteralType>, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap());
        }
        
        return Err(RuntimeError::new(name, format!("Undefined variable '{}'", name.lexeme).as_str()));
    }
}