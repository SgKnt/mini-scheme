use crate::data::{*, object::*};

use std::collections::VecDeque;

use anyhow::{Result, anyhow, Context};

pub fn is_null(mut args: VecDeque<Object>) -> Result<Object> {
    match args.pop_front().unwrap().kind() {
        Kind::Empty => Ok(Object::new_boolean(true, true)),
        _           => Ok(Object::new_boolean(false, true)), 
    }
}

pub fn is_pair(mut args: VecDeque<Object>) -> Result<Object> {
    match args.pop_front().unwrap().kind() {
        Kind::Pair(_) => Ok(Object::new_boolean(true, true)),
        _             => Ok(Object::new_boolean(false, true)),
    }
}

pub fn is_list(mut args: VecDeque<Object>) -> Result<Object> {
    Ok(Object::new_boolean(args.pop_front().unwrap().is_list(), true))
}

pub fn car(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    match obj.kind() {
        Kind::Pair(pair) => Ok(pair.car()),
        _ => Err(anyhow!("pair required, but got {}", obj))
    }
}

pub fn cdr(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    match obj.kind() {
        Kind::Pair(pair) => Ok(pair.cdr()),
        _ => Err(anyhow!("pair required, but got {}", obj))
    }
}

pub fn cons(mut args: VecDeque<Object>) -> Result<Object> {
    let car = args.pop_front().unwrap();
    let cdr = args.pop_front().unwrap();
    Ok(Object::new_pair(car, cdr, true))
}

pub fn list(mut args: VecDeque<Object>) -> Result<Object> {
    let mut res = Object::new_empty();
    while let Some(obj) = args.pop_back() {
        res = Object::new_pair(obj, res, true);
    }
    Ok(res)
}

pub fn length(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    let len = obj.length().with_context(|| format!("proper list required, but got {}", obj))?;
    Ok(Object::new_int(len as i64, true))
}

pub fn memq(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    let mut list = args.pop_front().unwrap();
    loop {
        match list.kind() {
            Kind::Pair(pair) => {
                if Object::scm_eq(&obj, &pair.car()) {
                    break Ok(list)
                } else {
                    list = pair.cdr();
                }
            }
            _ => break Ok(Object::new_boolean(false, true))
        }
    }
}

pub fn last(mut args: VecDeque<Object>) -> Result<Object> {
    let mut list = args.pop_front().unwrap();
    loop {
        match list.kind() {
            Kind::Pair(pair) => {
                let cdr = pair.cdr();
                match cdr.kind() {
                    Kind::Pair(_) => list = cdr,
                    _ => break Ok(pair.car()),
                }
            }
            _ => break Err(anyhow!("pair required, but got {}", list))
        }
    }
}

pub fn append(mut args: VecDeque<Object>) -> Result<Object> {
    fn append_inner(list1: Object, list2: Object) -> Result<Object> {
        match list1.kind() {
            Kind::Pair(pair) => Ok(Object::new_pair(pair.car(), append_inner(pair.cdr(), list2)?, true)),
            Kind::Empty => Ok(list2),
            _ => Err(anyhow!("proper list required, but got {}", list1))
        }
    }

    let mut res = args.pop_back().unwrap();
    while let Some(list) = args.pop_back() {
        res = append_inner(list, res)?
    }
    Ok(res)
}

pub fn set_car(mut args: VecDeque<Object>) -> Result<Object> {
    let pair = args.pop_front().unwrap();
    let obj = args.pop_front().unwrap();
    pair.set_car(obj.clone())?;
    Ok(Object::new_undefined())
}

pub fn set_cdr(mut args: VecDeque<Object>) -> Result<Object> {
    let pair = args.pop_front().unwrap();
    let obj = args.pop_front().unwrap();
    pair.set_cdr(obj.clone())?;
    Ok(Object::new_undefined())
}
