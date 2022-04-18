use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::object::{Object};

pub struct Environment {
    pub variables: RefCell<HashMap<String, RefCell<Rc<Object>>>>,
    pub parent: RefCell<Rc<Environment>>
}

impl Environment {
    pub fn lookup(&self, key: &str) -> Option<Rc<Object>> {
        if let Some(v) = self.variables.borrow().get(key) {
            Some(Rc::clone(&*v.borrow()))
        } else {
            self.parent.borrow().lookup(key)
        }
    }
}
