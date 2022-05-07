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
        // number
        vars.insert("number?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::number::is_number
        }))));
        vars.insert("+".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 0,
            fun: function::number::add
        }))));
        vars.insert("-".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 1,
            fun: function::number::minus
        }))));
        vars.insert("*".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 0,
            fun: function::number::mul
        }))));
        vars.insert("/".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 1,
            fun: function::number::div
        }))));
        vars.insert("=".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 2,
            fun: function::number::eq
        }))));
        vars.insert("<".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 2,
            fun: function::number::lt
        }))));
        vars.insert("<=".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 2,
            fun: function::number::le
        }))));
        vars.insert(">".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 2,
            fun: function::number::gt
        }))));
        vars.insert(">=".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 2,
            fun: function::number::ge
        }))));
        // boolean
        vars.insert("boolean?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::bool::is_bool
        }))));
        vars.insert("not".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::bool::not
        }))));
        // pair, list
        vars.insert("null?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::is_null
        }))));
        vars.insert("pair?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::is_pair
        }))));
        vars.insert("list?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::is_list
        }))));
        vars.insert("car".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::car
        }))));
        vars.insert("cdr".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::cdr
        }))));
        vars.insert("cons".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::list::cons
        }))));
        vars.insert("list".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 0,
            fun: function::list::list
        }))));
        vars.insert("length".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::length
        }))));
        vars.insert("memq".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::list::memq
        }))));
        vars.insert("last".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::list::last
        }))));
        vars.insert("append".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: true,
            required: 1,
            fun: function::list::append
        }))));
        vars.insert("set-car!".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::list::set_car
        }))));
        vars.insert("set-cdr!".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::list::set_cdr
        }))));
        // symbol
        vars.insert("symbol?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::symbol::is_symbol
        }))));
        // procedure
        vars.insert("procedure?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::procedure::is_procedure
        }))));
        // string
        vars.insert("string?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::string::is_string
        }))));
        vars.insert("string-append".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::string::string_append
        }))));
        vars.insert("symbol->string".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::string::symbol_to_string
        }))));
        vars.insert("string->symbol".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::string::string_to_symbol
        }))));
        vars.insert("string->number".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::string::string_to_number
        }))));
        vars.insert("number->string".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::string::number_to_string
        }))));
        // comparison
        vars.insert("eq?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::cmp::eq
        }))));
        vars.insert("neq?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::cmp::neq
        }))));
        vars.insert("equal?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 2,
            fun: function::cmp::equal
        }))));

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
