use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result};

use crate::env::Environment;
use crate::object::*;

pub fn is_bool(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*(*args.pop_front().unwrap()?).borrow() {
        Object::Boolean(_) => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn not(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    if (*args.pop_front().unwrap()?).borrow().is_falsy() {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(false))))
    }
}
