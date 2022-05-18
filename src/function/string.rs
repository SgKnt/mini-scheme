use crate::data::{*, object::*};

use std::collections::VecDeque;

use anyhow::{Result, anyhow};

pub fn is_string(mut args: VecDeque<Object>) -> Result<Object> {
    match args.pop_front().unwrap().kind() {
        Kind::String(_) => Ok(Object::new_boolean(true, true)),
        _               => Ok(Object::new_boolean(false, true)),
    }
}

pub fn string_append(mut args: VecDeque<Object>) -> Result<Object> {
    let obj1 = args.pop_front().unwrap();
    let obj2 = args.pop_front().unwrap();
    match (obj1.kind(), obj2.kind()) {
        (Kind::String(s1), Kind::String(s2)) => {
            let s = s1.clone() + s2;
            Ok(Object::new_string(s, true))
        }
        (Kind::String(_), _) => Err(anyhow!("string required, but got {}", obj2)),
        (_, _) => Err(anyhow!("string required, but got {}", obj1)),
    }
}

pub fn symbol_to_string(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    match obj.kind() {
        Kind::Symbol(s) => Ok(Object::new_string(s.clone(), true)),
        _ => Err(anyhow!("symbol required, but got {}", obj))
    }
}

pub fn string_to_symbol(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    match obj.kind() {
        Kind::String(s) => Ok(Object::new_symbol(s.clone(), true)),
        _ => Err(anyhow!("string required, but got {}", obj)),
    }
}

pub fn string_to_number(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    match obj.kind() {
        Kind::String(s) => {
            if let Ok(i) = s.parse::<i64>() {
                Ok(Object::new_int(i, true))
            } else if let Ok(f) = s.parse::<f64>() {
                Ok(Object::new_float(f, true))
            } else {
                Ok(Object::new_boolean(false, true))
            }
        }
        _ => Err(anyhow!("string required, but got {}", obj))
    }
}

pub fn number_to_string(mut args: VecDeque<Object>) -> Result<Object> {
    let obj = args.pop_front().unwrap();
    match obj.kind() {
        Kind::Number(num) => match num {
            &Number::Int(i) => Ok(Object::new_int(i, true)),
            &Number::Float(f) => Ok(Object::new_float(f, true)),
        }
        _ => Err(anyhow!("string required, but got {}", obj))
    }
}
