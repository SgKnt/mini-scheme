use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::object::{Object};

pub struct Environment {
    pub variables: RefCell<HashMap<String, Rc<RefCell<Object>>>>,
    pub parent: Option<Rc<Environment>>
}

impl Environment {
    pub fn new_global() -> Self {
        Environment{
            variables: RefCell::new(HashMap::new()),
            parent: None
        }
    }

    pub fn new(parent: &Rc<Environment>) -> Self {
        Environment{
            variables: RefCell::new(HashMap::new()),
            parent: Some(parent.clone())
        }
    }

    pub fn lookup(&self, key: &str) -> Option<Rc<RefCell<Object>>> {
        if let Some(v) = self.variables.borrow().get(key) {
            Some(Rc::clone(v))
        } else if let Some(p) = &self.parent {
            p.lookup(key)
        } else {
            None
        }
    }
}
