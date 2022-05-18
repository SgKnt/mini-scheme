use crate::data::{*, object::*};

use std::collections::VecDeque;

use anyhow::Result;

pub fn is_symbol(mut args: VecDeque<Object>) -> Result<Object> {
    match args.pop_front().unwrap().kind() {
        Kind::Symbol(_) => Ok(Object::new_boolean(true, true)),
        _               => Ok(Object::new_boolean(false, true)), 
    }
}
