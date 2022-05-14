pub mod object;
pub mod env;
mod r#ref;
mod memory;
mod gc;

use self::r#ref::{ObjRef, EnvRef};
use self::object::*;
use self::gc::Marker;
use self::memory::Memory;
use crate::data::env::EnvBody;
use crate::token::Token;

use std::cell::Cell;
use std::collections::{VecDeque, HashMap};

use anyhow::Result;

// define Object and Environment

/**
 * Object: struct for scheme object, used in evaluating input
 * The entity is object::ObjBody. 
 * The interpreter accesses an object through this object.
 */

pub struct Object{
    re: ObjRef,
}

impl Object {
    /***** Constructor from here *****/
    pub fn new_int(i: i64, is_mutable: bool) -> Object {
        let body = ObjBody {
            is_mutable,
            kind: Kind::Number(Number::Int(i)),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    pub fn new_float(f: f64, is_mutable: bool) -> Object {
        let body = ObjBody {
            is_mutable,
            kind: Kind::Number(Number::Float(f)),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    pub fn new_string(s: String, is_mutable: bool) -> Object {
        let body = ObjBody {
            is_mutable,
            kind: Kind::String(s),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    pub fn new_symbol(s: String, is_mutable: bool) -> Object {
        let body = ObjBody {
            is_mutable,
            kind: Kind::Symbol(s),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    pub fn new_empty() -> Object {
        Object{re: Memory::get_empty()}
    }

    pub fn new_pair(car: Object, cdr: Object, is_mutable: bool) -> Object {
        let body = ObjBody {
            is_mutable,
            kind: Kind::Pair(Pair{
                car: car.re,
                cdr: cdr.re,
            }),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    pub fn new_procedure(env: Environment, args: Vec<String>, is_variadic: bool, require: usize, body: Token) -> Object {
        let body = ObjBody {
            is_mutable: false,
            kind: Kind::Procedure(Procedure::Proc{
                env: env.re, 
                args, is_variadic, require, body,
            }),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    fn new_subroutine(is_variadic: bool, require: usize, fun: fn(VecDeque<Result<Object>>) -> Result<Object> ) -> Object {
        let body = ObjBody {
            is_mutable: false,
            kind: Kind::Procedure(Procedure::Subr{
                is_variadic, require, fun,
            }),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    /***** Constructor to here *****/

    pub fn is_falsy(&self) -> bool {
        match self.re.borrow().kind {
            Kind::Boolean(b) if !b => true,
            _ => false,
        }
    }

    #[inline]
    pub fn kind(&self) -> &Kind {
        &self.re.borrow().kind
    }
}

impl Clone for Object {
    fn clone(&self) -> Object {
        self.re.borrow().inc_rc();
        Object{re: self.re}
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.re.borrow().dec_rc();
    }
}


/**
 * Environment: struct for scheme environment, used in evaluating input
 * The entity is env::EnvBody. 
 * The interpreter accesses an environment through this object.
 */

pub struct Environment{
    re: EnvRef,
}

impl Environment {
    pub fn new_global(subrs: Vec<(String, bool, usize, fn(VecDeque<Result<Object>>) -> Result<Object>)>) -> Self {
        // lib[i].0: function name in scheme
        // lib[i].1: is variadic function? 
        // lib[i].2: number of required argument
        // lib[i].3: function
        let vars: HashMap<String, ObjRef> = HashMap::new();
        for subr in subrs {
            let name = subr.0;
            let subr = Object::new_subroutine(subr.1, subr.2, subr.3);
            vars.insert(name, subr.re);
        }
        let body = EnvBody {
            vars,
            parent: None,
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_env(body);
        Environment{re}
    }

    pub fn new(parent: Environment) -> Self {
        let body = EnvBody {
            vars: HashMap::new(),
            parent: Some(parent.re),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_env(body);
        Environment{re}
    }

    pub fn lookup(&self, id: &String) -> Option<Object> {
        self.re.lookup(id)
    }

    pub fn containt_at(&self, id: &String) -> Option<Environment> {
        self.re.contains_at(id)
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        let re = self.re;
        re.borrow().inc_rc();
        Environment{re}
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        self.re.borrow().dec_rc();
    }
}