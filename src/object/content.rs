use std::collections::VecDeque;
use std::sync::Arc;

use anyhow::{Result};

use crate::env::Environment;
use crate::token::Token;
use super::Object;

pub struct Content {
    pub mutable: bool,
    pub kind: ContentKind,
}

pub enum ContentKind {
    Number(Number),
    Boolean(bool),
    String(String),
    Symbol(String),
    Empty,
    Pair{car: usize, cdr: usize},
    Procedure(Procedure),
    Undefined,
}

pub enum Number {
    Int(i64),
    Float(f64),
}

pub enum Procedure {
    Proc{
        env: Arc<Environment>,
        args: Vec<String>,
        is_variadic: bool,
        require: usize,
        body: Token,
    },
    Subr{
        is_variadic: bool,
        require: usize,
        fun: fn(VecDeque<Result<Object>>) -> Result<Object>,
    }
}
