use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::object::{Object};

pub struct Environment {
    pub variables: RefCell<HashMap<String, RefCell<Rc<Object>>>>,
    pub parent: Option<RefCell<Rc<Environment>>>
}

impl Environment {
    pub fn new_global() -> Self {
        Environment{
            variables: RefCell::New(HashMap::new()),
            parent: None
        }
    }

    pub fn new(parent: &Rc<Environment>) -> Self {
        Environment{
            variables: RefCell::new(HashMap::new()),
            parent: Some(RefCell::new(parent.clone()))
        }
    }

    pub fn lookup(&self, key: &str) -> Option<Rc<Object>> {
        if let Some(v) = self.variables.borrow().get(key) {
            Some(Rc::clone(&*v.borrow()))
        } else if let Some(p) = self.parent {
            p.borrow().lookup(key)
        } else {
            None
        }
    }
}
