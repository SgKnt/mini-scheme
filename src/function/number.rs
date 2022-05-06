use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result, bail, anyhow};

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
                    &NumberKind::Int(i) => acc += i,
                    &NumberKind::Float(f) => 
                        return add_float(args, _env, (acc as f64) + f)
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
    let mut acc;

    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => acc = i,
            &NumberKind::Float(f) => return minus_float(args, _env, f),
        };
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
                    &NumberKind::Int(i) => acc -= i,
                    &NumberKind::Float(f) => 
                        return minus_float(args, _env, (acc as f64) - f)
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
                    &NumberKind::Int(i) => acc *= i,
                    &NumberKind::Float(f) => 
                        return mul_float(args, _env, (acc as f64) * f)
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

pub fn div(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut acc ;
    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => acc = i as f64,
            &NumberKind::Float(f) => acc = f,
        };
    } else {
        bail!("number required, but got {}", first.borrow())
    }

    if args.len() == 0 {
        // unary div
        if acc == 0f64 {
            bail!("zero division error");
        } else {
            return Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float((1 as f64) / acc)))))
        }
    }

    while let Some(obj) = args.pop_front() {
        let obj = obj?;
        let obj = (*obj).borrow();
        match &*obj {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(i) => {
                        if i == 0 {
                            bail!("zero division error");
                        }
                        acc /= i as f64;
                    }
                    &NumberKind::Float(f) => {
                        if f == 0.0 {
                            bail!("zero division error");
                        }
                        acc /= f;
                    }
                }
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Float(acc)))))
}

pub fn eq(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // binary op "=" 
    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => eq_int(args, i),
            &NumberKind::Float(f) => eq_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first.borrow()))
    }
}

fn eq_int(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: i64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if op1 != op2 {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            eq_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if (op1 as f64) != op2 {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            eq_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

fn eq_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: f64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if op1 != (op2 as f64) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            eq_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if op1 != op2 {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            eq_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

pub fn lt(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // binary op "<" 
    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => lt_int(args, i),
            &NumberKind::Float(f) => lt_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first.borrow()))
    }
}

fn lt_int(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: i64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 < op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            lt_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !((op1 as f64) < op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            lt_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

fn lt_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: f64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 < (op2 as f64)) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            lt_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !(op1 < op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            lt_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

pub fn le(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // binary op "<" 
    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => le_int(args, i),
            &NumberKind::Float(f) => le_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first.borrow()))
    }
}

fn le_int(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: i64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 <= op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            le_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !((op1 as f64) <= op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            le_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

fn le_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: f64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 <= (op2 as f64)) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            le_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !(op1 <= op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            le_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

pub fn gt(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // binary op "<" 
    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => gt_int(args, i),
            &NumberKind::Float(f) => gt_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first.borrow()))
    }
}

fn gt_int(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: i64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 > op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            gt_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !((op1 as f64) > op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            gt_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

fn gt_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: f64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 > (op2 as f64)) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            gt_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !(op1 > op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            gt_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

pub fn ge(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // binary op "<" 
    let first = args.pop_front().unwrap()?;
    let first = (*first).borrow();
    if let Object::Number(num) = &*first {
        match num {
            &NumberKind::Int(i) => ge_int(args, i),
            &NumberKind::Float(f) => ge_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first.borrow()))
    }
}

fn ge_int(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: i64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 >= op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            ge_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !((op1 as f64) >= op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            ge_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

fn ge_float(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, op1: f64) -> Result<Rc<RefCell<Object>>> {
    if let Some(op2) = args.pop_front() {
        let op2 = op2?;
        let op2 = (*op2).borrow();
        match &*op2 {
            Object::Number(num) => {
                match num {
                    &NumberKind::Int(op2) => {
                        if !(op1 >= (op2 as f64)) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            ge_int(args, op2)
                        }
                    }
                    &NumberKind::Float(op2) => {
                        if !(op1 >= op2) {
                            Ok(Rc::new(RefCell::new(Object::Boolean(false))))
                        } else {
                            ge_float(args, op2)
                        }
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Rc::new(RefCell::new(Object::Boolean(true))))
    }
}

