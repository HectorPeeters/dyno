use crate::error::*;
use std::collections::HashMap;

pub struct Scope<T> {
    items: Vec<HashMap<String, T>>,
}

impl<T> Scope<T> where T: Copy{
    pub fn new() -> Self {
        Self {
            items: vec![HashMap::new()],
        }
    }

    pub fn insert(&mut self, name: &str, data: T) -> DynoResult<()> {
        let scope_count = self.items.len();
        let last_scope = &mut self.items[scope_count - 1];

        if last_scope.contains_key(name) {
            return Err(DynoError::IdentifierError(format!(
                "Identifier already defined in scope: {}",
                name,
            )));
        }

        last_scope.insert(name.to_owned(), data);
        Ok(())
    }

    pub fn push(&mut self) {
        self.items.push(HashMap::new());
    }

    pub fn pop(&mut self) -> DynoResult<()> {
        match self.items.pop() {
            Some(_) => Ok(()),
            //TODO: Replace this with a better error type
            None => Err(DynoError::IdentifierError(
                "Tried popping while scope stack was empty".to_owned(),
            )),
        }
    }

    pub fn find(&mut self, name: &str) -> DynoResult<T> {
        for scope in self.items.iter().rev() {
            match scope.get(name) {
                Some(x) => return Ok(*x),
                None => continue,
            }
        }

        Err(DynoError::IdentifierError(format!(
            "Identifier `{}` not found in scope",
            name
        )))
    }
}

impl<T> Default for Scope<T> where T: Copy {
    fn default() -> Self {
        Self {
            items: vec![HashMap::new()],
        }
    } 
}
