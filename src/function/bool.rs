use std::collections::VecDeque;

use anyhow::Result;

use crate::data::*;
use crate::data::object::*;

pub fn is_bool(mut args: VecDeque<Object>) -> Result<Object> {
    match args.pop_front().unwrap().kind() {
        Kind::Boolean(_) => Ok(Object::new_boolean(true, true)),
        _ => Ok(Object::new_boolean(false, true)),
    }
}

pub fn not(mut args: VecDeque<Object>) -> Result<Object> {
    if args.pop_front().unwrap().is_falsy() {
        Ok(Object::new_boolean(true, true))
    } else {
        Ok(Object::new_boolean(false, true))
    }
}
