use std::collections::VecDeque;

use anyhow::{Result, bail, anyhow};

use crate::data::{*, object::*};

pub fn is_number(mut args: VecDeque<Object>) -> Result<Object> {
    match args.pop_front().unwrap().kind() {
        Kind::Number(_) => Ok(Object::new_boolean(true, true)),
        _ => Ok(Object::new_boolean(false, true)),
    }
}

pub fn add(mut args: VecDeque<Object>) -> Result<Object> {
    let mut acc = 0;
    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => acc += i,
                &Number::Float(f) => return add_float(args, (acc as f64) + f)
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Object::new_int(acc, true))
}

fn add_float(mut args: VecDeque<Object>, mut acc: f64) -> Result<Object> {
    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => acc += i as f64,
                &Number::Float(f) => acc += f,
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Object::new_float(acc, true))
}

pub fn minus(mut args: VecDeque<Object>) -> Result<Object> {
    let mut acc;

    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => acc = i,
            &Number::Float(f) => return minus_float(args, f),
        };
    } else {
        bail!("number required, but got {}", first)
    }

    if args.len() == 0 {
        return Ok(Object::new_int(-acc, true));
    }

    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => acc -= i,
                &Number::Float(f) => return minus_float(args, (acc as f64) - f),
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    
    Ok(Object::new_int(acc, true))
}

fn minus_float(mut args: VecDeque<Object>, mut acc: f64) -> Result<Object> {
    if args.len() == 0{
        return Ok(Object::new_float(-acc, true));
    }

    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => acc -= i as f64,
                &Number::Float(f) => acc -= f,
            }
            _ => bail!("number required, but got {}", obj)
        }
    }

    Ok(Object::new_float(acc, true))
} 

pub fn mul(mut args: VecDeque<Object>) -> Result<Object> {
    let mut acc = 1;
    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => acc *= i,
                &Number::Float(f) => return mul_float(args, (acc as f64) * f)
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Object::new_int(acc, true))
}

fn mul_float(mut args: VecDeque<Object>, mut acc: f64) -> Result<Object> {
    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => acc *= i as f64,
                &Number::Float(f) => acc *= f,
            }
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Object::new_float(acc, true))
}

pub fn div(mut args: VecDeque<Object>) -> Result<Object> {
    let mut acc ;
    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => acc = i as f64,
            &Number::Float(f) => acc = f,
        };
    } else {
        bail!("number required, but got {}", first)
    }

    if args.len() == 0 {
        // unary div
        if acc == 0f64 {
            bail!("zero division error");
        } else {
            return Ok(Object::new_float((1 as f64) / acc, true));
        }
    }

    while let Some(obj) = args.pop_front() {
        match obj.kind() {
            Kind::Number(num) => match num {
                &Number::Int(i) => {
                    if i == 0 {
                        bail!("zero division error");
                    }
                    acc /= i as f64;
                }
                &Number::Float(f) => {
                    if f == 0.0 {
                        bail!("zero division error");
                    }
                    acc /= f;
                }
            }
            
            _ => bail!("number required, but got {}", obj)
        }
    }
    Ok(Object::new_float(acc, true))
}

pub fn eq(mut args: VecDeque<Object>) -> Result<Object> {
    // binary op "=" 
    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => eq_int(args, i),
            &Number::Float(f) => eq_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first))
    }
}

fn eq_int(mut args: VecDeque<Object>, op1: i64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if op1 != op2 {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        eq_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if (op1 as f64) != op2 {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        eq_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

fn eq_float(mut args: VecDeque<Object>, op1: f64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if op1 != (op2 as f64) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        eq_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if op1 != op2 {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        eq_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

pub fn lt(mut args: VecDeque<Object>) -> Result<Object> {
    // binary op "=" 
    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => lt_int(args, i),
            &Number::Float(f) => lt_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first))
    }
}

fn lt_int(mut args: VecDeque<Object>, op1: i64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 < op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        lt_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !((op1 as f64) < op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        lt_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

fn lt_float(mut args: VecDeque<Object>, op1: f64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 < (op2 as f64)) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        lt_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !(op1 < op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        lt_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

pub fn le(mut args: VecDeque<Object>) -> Result<Object> {
    // binary op "=" 
    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => le_int(args, i),
            &Number::Float(f) => le_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first))
    }
}

fn le_int(mut args: VecDeque<Object>, op1: i64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 <= op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        le_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !((op1 as f64) <= op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        le_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

fn le_float(mut args: VecDeque<Object>, op1: f64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 <= (op2 as f64)) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        le_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !(op1 <= op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        le_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

pub fn gt(mut args: VecDeque<Object>) -> Result<Object> {
    // binary op "=" 
    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => gt_int(args, i),
            &Number::Float(f) => gt_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first))
    }
}

fn gt_int(mut args: VecDeque<Object>, op1: i64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 > op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        gt_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !((op1 as f64) > op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        gt_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

fn gt_float(mut args: VecDeque<Object>, op1: f64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 > (op2 as f64)) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        gt_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !(op1 > op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        gt_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

pub fn ge(mut args: VecDeque<Object>) -> Result<Object> {
    // binary op "=" 
    let first = args.pop_front().unwrap();
    if let Kind::Number(num) = first.kind() {
        match num {
            &Number::Int(i) => ge_int(args, i),
            &Number::Float(f) => ge_float(args, f),
        }
    } else {
        Err(anyhow!("number required, but got {}", first))
    }
}

fn ge_int(mut args: VecDeque<Object>, op1: i64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 >= op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        ge_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !((op1 as f64) >= op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        ge_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

fn ge_float(mut args: VecDeque<Object>, op1: f64) -> Result<Object> {
    if let Some(op2) = args.pop_front() {
        match op2.kind() {
            Kind::Number(num) => match num {
                &Number::Int(op2) => {
                    if !(op1 >= (op2 as f64)) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        ge_int(args, op2)
                    }
                }
                &Number::Float(op2) => {
                    if !(op1 >= op2) {
                        Ok(Object::new_boolean(false, true))
                    } else {
                        ge_float(args, op2)
                    }
                }
            }
            _ => Err(anyhow!("number required, but got {}", op2))
        }
    } else {
        Ok(Object::new_boolean(true, true))
    }
}

