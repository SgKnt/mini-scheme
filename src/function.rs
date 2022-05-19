pub mod number;
pub mod bool;
pub mod list;
pub mod symbol;
pub mod cmp;
pub mod string;
pub mod procedure;
pub mod display;

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
    // pair, list
    lib.push(("null?".to_string(), false, 1, list::is_null));
    lib.push(("pair?".to_string(), false, 1, list::is_pair));
    lib.push(("list?".to_string(), false, 1, list::is_list));
    lib.push(("car".to_string(), false, 1, list::car));
    lib.push(("cdr".to_string(), false, 1, list::cdr));
    lib.push(("cons".to_string(), false, 2, list::cons));
    lib.push(("list".to_string(), true, 0, list::list));
    lib.push(("length".to_string(), false, 1, list::length));
    lib.push(("memq".to_string(), false, 2, list::memq));
    lib.push(("last".to_string(), false, 1, list::last));
    lib.push(("append".to_string(), true, 1, list::append));
    lib.push(("set-car!".to_string(), false, 2, list::set_car));
    lib.push(("set-cdr!".to_string(), false, 2, list::set_cdr));
    // string
    lib.push(("string?".to_string(), false, 1, string::is_string));
    lib.push(("string-append".to_string(), false, 2, string::string_append));
    lib.push(("symbol->string".to_string(), false, 1, string::symbol_to_string));
    lib.push(("string->symbol".to_string(), false, 1, string::string_to_symbol));
    lib.push(("string->number".to_string(), false, 1, string::string_to_number));
    lib.push(("number->string".to_string(), false, 1, string::number_to_string));
    // symbol
    lib.push(("symbol?".to_string(), false, 1, symbol::is_symbol));
    // procedure
    lib.push(("procedure?".to_string(), false, 1, procedure::is_procedure));
    // comparison
    lib.push(("eq?".to_string(), false, 2, cmp::eq));
    lib.push(("neq?".to_string(), false, 2, cmp::neq));
    lib.push(("equal?".to_string(), false, 2, cmp::equal));

    // additional
    lib.push(("display".to_string(), false, 1, display::display));

    lib
}
