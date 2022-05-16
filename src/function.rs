pub mod number;
pub mod bool;
// pub mod list;
// pub mod symbol;
// pub mod cmp;
// pub mod string;
// pub mod procedure;

use crate::data::Object;

use std::collections::VecDeque;

use anyhow::Result;

pub fn make_lib() -> Vec<(String, bool, usize, fn(VecDeque<Object>) -> Result<Object>)> {
    let mut lib: Vec<(String, bool, usize, fn(VecDeque<Object>) -> Result<Object>)> = Vec::new();
    // number
    lib.push(("number?".to_string(), false, 1, number::is_number));
    lib.push(("+".to_string(), true, 0, number::add));
    lib.push(("-".to_string(), true, 1, number::minus));
    lib.push(("*".to_string(), true, 0, number::mul));
    lib.push(("/".to_string(), true, 1, number::div));
    lib.push(("=".to_string(), true, 2, number::eq));
    lib.push(("<".to_string(), true, 2, number::lt));
    lib.push(("<=".to_string(), true, 2, number::le));
    lib.push((">".to_string(), true, 2, number::gt));
    lib.push((">=".to_string(), true, 2, number::ge));
    // boolean
    lib.push(("boolean?".to_string(), false, 1, bool::is_bool));
    lib.push(("not".to_string(), false, 1, bool::not));

    lib
}
