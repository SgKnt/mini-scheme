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
        // list
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
        // symbol
        vars.insert("symbol?".to_string(), Rc::new(RefCell::new(Object::Subroutine(Subroutine{
            is_variadic: false,
            required: 1,
            fun: function::symbol::is_symbol
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
