use crate::data::{*, object::*};

use std::collections::VecDeque;

use anyhow::Result;

pub fn eq(mut args: VecDeque<Object>) -> Result<Object> {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();
    Ok(Object::new_boolean(Object::scm_eq(&lhs, &rhs), true))
}

pub fn neq(mut args: VecDeque<Object>) -> Result<Object> {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();
    Ok(Object::new_boolean(!Object::scm_eq(&lhs, &rhs), true))
}

pub fn equal(mut args: VecDeque<Object>) -> Result<Object> {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();
    Ok(Object::new_boolean(Object::scm_equal(&lhs, &rhs), true))
}
