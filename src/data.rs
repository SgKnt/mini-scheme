pub mod object;
pub mod env;
mod r#ref;
pub mod memory;

use self::r#ref::{ObjRef, EnvRef};
use self::object::*;
use self::memory::Marker;
use self::memory::Memory;
use crate::data::env::EnvBody;
use crate::token::Token;

use std::cell::Cell;
use std::collections::{VecDeque, HashMap};
use std::fmt;
use std::iter::{Iterator, IntoIterator};

use anyhow::Result;

// define Object and Environment
// These structs follows Interior mutability pattern (擬き)

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

    pub fn new_boolean(b: bool, is_mutable: bool) -> Object {
        let body = ObjBody {
            is_mutable,
            kind: Kind::Boolean(b),
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
            kind: Kind::Procedure(Procedure::Proc(Proc{
                env: env.re, 
                args, is_variadic, require, body,
            })),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    fn new_subroutine(is_variadic: bool, require: usize, fun: fn(VecDeque<Object>) -> Result<Object> ) -> Object {
        let body = ObjBody {
            is_mutable: false,
            kind: Kind::Procedure(Procedure::Subr(Subr{
                is_variadic, require, fun,
            })),
            mark: Marker::Black,
            rc: Cell::new(1),
        };
        let re = Memory::push_obj(body);
        Object{re}
    }

    pub fn new_undefined() -> Object {
        let body = ObjBody {
            is_mutable: false,
            kind: Kind::Undefined,
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

    pub fn is_list(&self) -> bool {
        self.re.is_list()
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

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/* scheme List to Iterator */
pub struct SchemeListIter{
    obj: Object,
}

impl IntoIterator for &Object {
    type Item = Object;
    type IntoIter = SchemeListIter;
    fn into_iter(self) -> Self::IntoIter {
        SchemeListIter{obj: self.clone()}
    }
}

impl Iterator for SchemeListIter {
    type Item = Object;
    fn next(&mut self) -> Option<Self::Item> {
        let (car, cdr) = if let Kind::Pair(pair) = self.obj.kind() {
            (pair.car(), pair.cdr())
        } else {
            return None
        };
        self.obj = cdr;
        Some(car)
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
    pub fn new_global(subrs: Vec<(String, bool, usize, fn(VecDeque<Object>) -> Result<Object>)>) -> Self {
        // lib[i].0: function name in scheme
        // lib[i].1: is variadic function? 
        // lib[i].2: number of required argument
        // lib[i].3: function
        let mut vars: HashMap<String, ObjRef> = HashMap::new();
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

    pub fn parent(&self) -> Option<Environment> {
        if let Some(parent) = self.re.borrow().parent {
            parent.borrow().inc_rc();
            Some(Environment{re: parent})
        } else {
            None
        }
    }

    pub fn lookup(&self, id: &String) -> Option<Object> {
        self.re.lookup(id)
    }

    pub fn contains_at(&self, id: &String) -> Option<Environment> {
        self.re.contains_at(id)
    }

    pub fn insert(&self, id: String, obj: Object) {
        unsafe {
            self.re.borrow_mut().vars.insert(id, obj.re);
        }
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
