use super::r#ref::{ObjRef, EnvRef};
use super::memory::Marker;
use super::{Object, Environment};
use crate::token::Token;

use std::collections::VecDeque;
use std::cell::Cell;

use anyhow::{Result};

pub(crate) struct ObjBody {
    pub is_mutable: bool,
    pub kind: Kind,
    pub mark: Marker,
    pub rc: Cell<u32>,
}

pub enum Kind {
    Number(Number),
    Boolean(bool),
    String(String),
    Symbol(String),
    Empty,
    Pair(Pair),
    Procedure(Procedure),
    Undefined,
}

pub enum Number {
    Int(i64),
    Float(f64),
}

pub struct Pair {
    pub(crate) car: ObjRef,
    pub(crate) cdr: ObjRef,
}

pub enum Procedure {
    Proc(Proc),
    Subr(Subr),
}

pub struct Proc {
    pub(crate) env: EnvRef,
    pub args: Vec<String>,
    pub is_variadic: bool,
    pub require: usize,
    pub body: Token,
}

pub struct Subr {
    pub is_variadic: bool,
    pub require: usize,
    pub fun: fn(VecDeque<Object>) -> Result<Object>,
}

impl ObjBody {
    #[inline]
    fn rc(&self) -> u32 {
        self.rc.get()
    }

    #[inline]
    pub fn inc_rc(&self) {
        let rc = self.rc();
        self.rc.set(rc + 1);
    }

    #[inline]
    pub fn dec_rc(&self) {
        let rc = self.rc();
        if rc == 0 {
            // こうなると手遅れ unsafeを使いこなせていない
            panic!("An ObjBody which is already dead has been borrowed.");
        }
        self.rc.set(rc - 1);
    }
}

impl Pair {
    pub fn car(&self) -> Object {
        let re = self.car;
        re.borrow().inc_rc();
        Object{re}
    }

    pub fn cdr(&self) -> Object {
        let re = self.cdr;
        re.borrow().inc_rc();
        Object{re}
    }
}

impl Proc {
    pub fn env(&self) -> Environment {
        let env = self.env;
        env.borrow().inc_rc();
        Environment{re: env}
    }
}
