use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result};

use crate::env::Environment;
use crate::object::*;

pub fn is_symbol(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.as_ref().borrow() {
        Object::Symbol(_) => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}
