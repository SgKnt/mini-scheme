use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result, anyhow};

use crate::env::Environment;
use crate::object::*;

pub fn is_string(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::String(_) => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn string_append(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match (&*args.pop_front().unwrap()?.borrow(), &*args.pop_front().unwrap()?.borrow()) {
        (Object::String(s1), Object::String(s2)) => {
            let s = s1.clone() + s2;
            Ok(Rc::new(RefCell::new(Object::String(s))))
        }
        (Object::String(_), obj) => Err(anyhow!("string required, but got {}", obj)),
        (obj, _) => Err(anyhow!("string required, but got {}", obj)),
    }
}

pub fn symbol_to_string(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Symbol(s) => Ok(Rc::new(RefCell::new(Object::String(s.clone())))),
        obj => Err(anyhow!("symbol required, but got {}", obj)),
    }
}

pub fn string_to_symbol(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::String(s) => Ok(Rc::new(RefCell::new(Object::Symbol(s.clone())))),
        obj => Err(anyhow!("string required, but got {}", obj)),
    }
}

pub fn string_to_number(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::String(s) => {
            if let Ok(i) = s.parse::<i64>() {
                Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Int(i)))))
            } else if let Ok(f) = s.parse::<f64>() {
                Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float(f)))))
            } else {
                Ok(Rc::new(RefCell::new(Object::Boolean(false))))
            }
        }
        obj => Err(anyhow!("string required, but got {}", obj)),
    }
}

pub fn number_to_string(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Number(num) => match num {
            NumberKind::Int(i) => Ok(Rc::new(RefCell::new(Object::String(i.to_string())))),
            NumberKind::Float(f) => Ok(Rc::new(RefCell::new(Object::String(f.to_string())))),
        }
        obj => Err(anyhow!("symbol required, but got {}", obj)),
    }
}
