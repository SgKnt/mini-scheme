use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result};

use crate::env::Environment;
use crate::object::*;

pub fn is_procedure(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Procedure(_) | Object::Subroutine(_) => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}