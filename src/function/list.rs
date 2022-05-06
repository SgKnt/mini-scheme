use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result};

use crate::env::Environment;
use crate::object::*;

pub fn is_null(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Empty => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn is_pair(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Pair{..} => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn is_list(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let arg = args.pop_front().unwrap()?;

    is_list_inner(&arg)
}

fn is_list_inner(obj: &Rc<RefCell<Object>>) -> Result<Rc<RefCell<Object>>> {
    match &*obj.borrow() {
        Object::Pair{car:_, cdr} => is_list_inner(cdr),
        Object::Empty => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}