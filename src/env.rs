use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::object::*;
use crate::function;

pub struct Environment {
    pub vars: RefCell<HashMap<String, Rc<RefCell<Object>>>>,
    pub parent: Option<Rc<Environment>>
}

impl Environment {
    pub fn new_global() -> Self {
        // add functions
        let mut vars = HashMap::new();
        macro_rules! register_subroutine {
            ($fun_name:expr, $is_variadic:expr, $required:expr, $fun:expr) => {
                vars.insert($fun_name.to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
                    is_variadic: $is_variadic,
                    required: $required,
                    fun: $fun,
                }))));
            };
        }
        // number
        register_subroutine!("number?", false, 1, function::number::is_number);
        register_subroutine!("+", true, 0, function::number::add);
        register_subroutine!("-", true, 1, function::number::minus);
        register_subroutine!("*", true, 0, function::number::mul);
        register_subroutine!("/", true, 1, function::number::div);
        register_subroutine!("=", true, 2, function::number::eq);
        register_subroutine!("<", true, 2, function::number::lt);
        register_subroutine!("<=", true, 2, function::number::le);
        register_subroutine!(">", true, 2, function::number::gt);
        register_subroutine!(">=", true, 2, function::number::ge);
        // boolean
        register_subroutine!("boolean?", false, 1, function::bool::is_bool);
        register_subroutine!("not", false, 1, function::bool::not);
        // pair, list
        register_subroutine!("null?", false, 1, function::list::is_null);
        register_subroutine!("pair?", false, 1, function::list::is_pair);
        register_subroutine!("list?", false, 1, function::list::is_list);
        register_subroutine!("car", false, 1, function::list::car);
        register_subroutine!("cdr", false, 1, function::list::cdr);
        register_subroutine!("cons", false, 2, function::list::cons);
        register_subroutine!("list", true, 0, function::list::list);
        register_subroutine!("length", false, 1, function::list::length);
        register_subroutine!("memq", false, 2, function::list::memq);
        register_subroutine!("last", false, 1, function::list::last);
        register_subroutine!("append", true, 1, function::list::append);
        register_subroutine!("set-car!", false, 2, function::list::set_car);
        register_subroutine!("set-cdr!", false, 2, function::list::set_cdr);
        // symbol
        register_subroutine!("symbol?", false, 1, function::symbol::is_symbol);
        // procedure
        register_subroutine!("procedure?", false, 1, function::procedure::is_procedure);
        // string
        register_subroutine!("string?", false, 1, function::string::is_string);
        register_subroutine!("string-append", false, 2, function::string::string_append);
        register_subroutine!("symbol->string", false, 1, function::string::symbol_to_string);
        register_subroutine!("string->symbol", false, 1, function::string::string_to_symbol);
        register_subroutine!("string->number", false, 1, function::string::string_to_number);
        register_subroutine!("number->string", false, 1, function::string::number_to_string);
        // comparison
        register_subroutine!("eq?", false, 2, function::cmp::eq);
        register_subroutine!("neq?", false, 2, function::cmp::neq);
        register_subroutine!("equal?", false, 2, function::cmp::equal);

        Environment{
            vars: RefCell::new(vars),
            parent: None
        }
    }

    pub fn new(parent: &Rc<Environment>) -> Self {
        Environment{
            vars: RefCell::new(HashMap::new()),
            parent: Some(parent.clone())
        }
    }

    pub fn lookup(&self, key: &str) -> Option<Rc<RefCell<Object>>> {
        if let Some(v) = self.vars.borrow().get(key) {
            Some(Rc::clone(v))
        } else if let Some(p) = &self.parent {
            p.lookup(key)
        } else {
            None
        }
    }
}
