use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::object::*;
use crate::function;

pub struct Environment {
    pub vars: Mutex<HashMap<String, usize>>,
    pub parent: Option<Arc<Environment>>
}

impl Environment {
    pub fn new_global() -> Self {
        let mut vars = HashMap::new();
        Environment{
            vars: Mutex::new(vars),
            parent: None
        }
    }

    pub fn new(parent: &Arc<Environment>) -> Self {
        Environment{
            vars: Mutex::new(HashMap::new()),
            parent: Some(parent.clone())
        }
    }

    pub fn lookup(&self, key: &str) -> Option<Object> {
        if let Some(pointer) = self.vars.lock().unwrap().get(key) {
            Some(memory::get_object(*pointer).expect("this environment has null"))
        } else if let Some(p) = &self.parent {
            p.lookup(key)
        } else {
            None
        }
    }

    pub fn contains_at(self: &Arc<Environment>, key: &str) -> Option<Arc<Environment>> {
        if let Some(pointer) = self.vars.lock().unwrap().get(key) {
            Some(Arc::clone(self))
        } else if let Some(p) = &self.parent {
            p.contains_at(key)
        } else {
            None
        }
    }
}
