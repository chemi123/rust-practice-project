use std::{rc::Rc, cell::RefCell, collections::HashMap};

use crate::lisp_expr::LispExpr;

#[derive(Debug, Default, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, LispExpr>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            parent: None,
            vars: HashMap::new(),
        }
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Self {
        Env {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<LispExpr> {
        match self.vars.get(key) {
            Some(value) => Some(value.clone()),
            _ => self.parent
                .as_ref()
                .and_then(|r| r.borrow().get(key)),
        }
    }

    pub fn set(&mut self, key: &str, value: LispExpr) {
        self.vars.insert(key.to_string(), value);
    }
}