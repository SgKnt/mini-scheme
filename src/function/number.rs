use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Context, Result, bail};

use crate::env::Environment;
use crate::object::*;

pub fn is_number(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*(*args.pop_front().unwrap()?).borrow() {
        Object::Number(_) => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn add(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut acc = 0;
    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    NumberKind::Int(i) => acc += i,
                    NumberKind::Float(f) => 
                        return add_float(args, _env, f + (acc as f64))
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Int(acc)))))
}

fn add_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>, mut acc: f64) -> Result<Rc<RefCell<Object>>> {
    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(i) => acc += i as f64,
                    &NumberKind::Float(f) => acc += f,
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float(acc)))))
}

pub fn minus(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut acc = 0;

    let first = args.pop_front().unwrap()?;
    if let Object::Number(num) = &*first.borrow() {
        match num {
            &NumberKind::Int(i) => acc = i,
            &NumberKind::Float(f) => return minus_float(args, _env, f),
        }
    } else {
        bail!("number required, but got {}", first.borrow())
    }

    if args.len() == 0 {
        // unary minus
        return Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Int(-acc)))));
    }

    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    NumberKind::Int(i) => acc -= i,
                    NumberKind::Float(f) => 
                        return minus_float(args, _env, f + (acc as f64))
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Int(acc)))))
}

fn minus_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>, mut acc: f64) -> Result<Rc<RefCell<Object>>> {
    if args.len() == 0 {
        // unary minus
        return Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float(-acc)))));
    }

    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(i) => acc -= i as f64,
                    &NumberKind::Float(f) => acc -= f,
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float(acc)))))
}

pub fn mul(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut acc = 1;
    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    NumberKind::Int(i) => acc *= i,
                    NumberKind::Float(f) => 
                        return add_float(args, _env, f * (acc as f64))
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Int(acc)))))
}

fn mul_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>, mut acc: f64) -> Result<Rc<RefCell<Object>>> {
    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(i) => acc *= i as f64,
                    &NumberKind::Float(f) => acc *= f,
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float(acc)))))
}
