use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::Object;

#[derive(Debug, Default, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.vars.get(key) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(key).clone()),
        }
    }

    pub fn set(&mut self, key: &str, value: Object) {
        self.vars.insert(key.to_string(), value);
    }
}
