use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::env::Environment;

pub struct Object {
    pub kind: ObjectKind
}

pub enum ObjectKind {
    Number(NumberKind),
    Boolean(bool),
    Pair{
        car: Ref,
        cdr: Ref
    },
    Empty,
    Procedure(Procedure),
    Symbol(String),
    String(String),
    Undefined,
}

impl Object {
    pub fn is_falsy(&self) -> bool {
        if let ObjectKind::Boolean(false) = self.kind {
            true
        } else {
            false
        }
    }
}

pub enum Ref {
    Rc(RefCell<Rc<Object>>),
    Weak(RefCell<Weak<Object>>)
}

pub enum NumberKind {
    Int(i64),
    Float(f64)
}

pub struct Procedure {
    pub env: RefCell<Rc<Environment>>
}
