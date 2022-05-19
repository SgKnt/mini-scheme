use crate::data::*;

use std::collections::VecDeque;

use anyhow::Result;

pub fn display(mut args: VecDeque<Object>) -> Result<Object> {
    println!("{}", args.pop_front().unwrap());
    Ok(Object::new_undefined())
}
