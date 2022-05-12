mod content;
pub mod memory;

use std::collections::VecDeque;
use std::sync::Arc;
use std::fmt;

use anyhow::Result;

use crate::env::Environment;
use crate::token::Token;
use content::{Content, ContentKind};

pub struct Object {
    pointer: usize,
    pub mutable: bool,
    pub kind: ObjectKind,
}

#[derive(Clone)]
pub enum ObjectKind {
    Number(Number),
    Boolean(bool),
    String(String),
    Symbol(String),
    Empty,
    Pair,
    Procedure(Procedure),
    Undefined,
}

#[derive(Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Clone)]
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

impl Object {
    // Constructor
    fn new_int(v: i64, mutable: bool) -> Self {
        let new = Content{
            mutable,
            kind: ContentKind::Number(content::Number::Int(v)),
            rc: 1,
        };
        let pointer = memory::push(new);
        Object { 
            pointer,
            mutable,
            kind: ObjectKind::Number(Number::Int(v))
        }
    }

    fn new_float(v: f64, mutable: bool) -> Self {
        let new = Content{
            mutable,
            kind: ContentKind::Number(content::Number::Float(v)),
            rc: 1,
        };
        let pointer = memory::push(new);
        Object {
            pointer,
            mutable,
            kind: ObjectKind::Number(Number::Float(v))
        }
    }

    fn new_boolean(v: bool, mutable: bool) -> Self {
        let new = Content {
            mutable,
            kind: ContentKind::Boolean(v),
            rc: 1,
        };
        let pointer = memory::push(new);
        Object { 
            pointer, 
            mutable,
            kind: ObjectKind::Boolean(v),
        }
    }

    fn new_string(v: String, mutable: bool) -> Self {
        let new = Content {
            mutable,
            kind: ContentKind::String(v),
            rc: 1,
        };
        let pointer = memory::push(new);
        Object { 
            pointer, 
            mutable,
            kind: ObjectKind::String(v),
        }
    }

    fn new_symbol(v: String, mutable: bool) -> Self {
        let new = Content {
            mutable,
            kind: ContentKind::Symbol(v),
            rc: 1,
        };
        let pointer = memory::push(new);
        Object { 
            pointer, 
            mutable,
            kind: ObjectKind::Symbol(v),
        }
    }

    fn new_empty() -> Self {
        {
            let c_empty = memory::MEMORY
            .get()
            .unwrap()
            .mem[0]
            .lock()
            .unwrap();
            c_empty.unwrap().0.rc += 1;
            c_empty.unwrap().1 = memory::Marker::Black;
        }
        Object {
            pointer: 0,
            mutable: false,
            kind: ObjectKind::Empty,
        }
    }

    fn new_pair(car: Object, cdr: Object, mutable: bool) -> Self {
        let new = Content{
            mutable,
            kind: ContentKind::Pair{car: car.pointer, cdr: cdr.pointer},
            rc: 1,
        };
        let pointer = memory::push(new);
        {
            memory::MEMORY
                .get()
                .unwrap()
                .mem[car.pointer]
                .lock()
                .unwrap()
                .expect("pair points None")
                .1 = memory::Marker::Black;
        }
        {
            memory::MEMORY
                .get()
                .unwrap()
                .mem[cdr.pointer]
                .lock()
                .unwrap()
                .expect("pair points None")
                .1 = memory::Marker::Black;
        }
        Object {
            mutable,
            pointer,
            kind: ObjectKind::Pair,
        }
    }

    fn new_procedure(env: Arc<Environment>, args: Vec<String>, is_variadic: bool, require: usize, body: Token, mutable: bool) -> Self {
        let new = Content {
            mutable,
            kind: ContentKind::Procedure(content::Procedure::Proc{
                env: Arc::clone(&env),
                args: args.clone(),
                is_variadic,
                require,
                body: body.clone(),
            }),
            rc: 1,
        };
        let pointer = memory::push(new);
        Object { 
            pointer, 
            mutable,
            kind: ObjectKind::Procedure(Procedure::Proc{
                env, args, is_variadic, require, body,
            }),
        }
    }

    fn new_undefined(mutable: bool) -> Self {
        let new = Content {
            mutable,
            kind: ContentKind::Undefined,
            rc: 1,
        };
        let pointer = memory::push(new);
        Object { 
            pointer, 
            mutable,
            kind: ObjectKind::Undefined,
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        memory::MEMORY
            .get()
            .unwrap()
            .mem[self.pointer]
            .lock()
            .unwrap()
            .expect("exists object of None")
            .0.rc += 1;
        Object {
            pointer: self.pointer,
            mutable: self.mutable,
            kind: self.kind.clone(),
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        memory::MEMORY
            .get()
            .unwrap()
            .mem[self.pointer]
            .lock()
            .unwrap()
            .expect("exists object of None")
            .0.rc -= 1;
    }
}
