use super::r#ref::{ObjRef, EnvRef};
use super::gc::Marker;
use super::Object;
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
    Proc{
        env: EnvRef,
        args: Vec<String>,
        is_variadic: bool,
        require: usize,
        body: Token,
    },
    Subr{
        is_variadic: bool,
        require: usize,
        fun: fn(VecDeque<Result<Object>>) -> Result<Object>
    }
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
    pub fn dec_rc(self) {
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
