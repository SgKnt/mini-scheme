use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result};

use crate::env::Environment;
use crate::object::*;

pub fn eq(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let lhs = args.pop_front().unwrap()?;
    let lhs = lhs.borrow();
    let rhs = args.pop_front().unwrap()?;
    let rhs = rhs.borrow();
    Ok(Rc::new(RefCell::new(Object::Boolean(lhs.eq_scm(&*rhs)))))
}

pub fn neq(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let lhs = args.pop_front().unwrap()?;
    let lhs = lhs.borrow();
    let rhs = args.pop_front().unwrap()?;
    let rhs = rhs.borrow();
    Ok(Rc::new(RefCell::new(Object::Boolean(!lhs.eq_scm(&*rhs)))))
}

pub fn equal(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let lhs = args.pop_front().unwrap()?;
    let lhs = lhs.borrow();
    let rhs = args.pop_front().unwrap()?;
    let rhs = rhs.borrow();
    Ok(Rc::new(RefCell::new(Object::Boolean(lhs.equal_scm(&*rhs)))))
}
