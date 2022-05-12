use std::sync::{Mutex, Arc};

use once_cell::sync::OnceCell;

use super::content::{Content, ContentKind};
use crate::env::Environment;
use super::{Object, ObjectKind};
use super::content;

// if you get multiple locks,
// get ones in the order of vacancy.lock() -> mem[i].lock() -> mem[j].lock() (i < j)
pub(crate) struct Memory {
    size: usize,
    pub mem: Vec<Mutex<Option<(Content, Marker)>>>,
    pub vacancy: Mutex<Vec<usize>>,
}

pub(crate) enum Marker {
    White, // might dead
    Black, // alive
}

pub(crate) static MEMORY: OnceCell<Memory> = OnceCell::new();

pub fn init(size: usize, global_env: &Environment) {
    let mut mem = Vec::new();
    /* Add built-in function */
    let mut begin = 0;

    mem.push(Mutex::new(Some((
        Content{
            mutable: false,
            kind: ContentKind::Empty,
            rc: 0,
        }, 
        Marker::Black
    ))));
    begin += 1;

    // allocate 
    mem.resize_with(begin + size, || Mutex::new(None));
    MEMORY.set(Memory {
        size, 
        mem, 
        vacancy: Mutex ::new((begin..size).rev().collect()),
    });
}

pub(crate) fn push(new: Content) -> usize {
    let idx = loop {
        let idx = MEMORY.get()
            .unwrap()
            .vacancy
            .lock()
            .unwrap()
            .pop();
        if let None = idx {
            todo!(); // send message to make gc start
        } else {
            break idx.unwrap();
        }
    };
    let old = MEMORY.get()
        .unwrap()
        .mem[idx]
        .lock()
        .unwrap();
    if old.is_some() {
        panic!("cannot add a new content at where other content already exists");
    } else {
        *old = Some((new, Marker::Black));
        idx
    }
}

pub fn get_object(pointer: usize) -> Option<Object> {
    let (mutable, kind) = {
        let cont = MEMORY.get()
            .unwrap()
            .mem[pointer]
            .lock()
            .unwrap();
        if let Some((cont, _)) = cont.as_mut() {
            cont.rc += 1;
            let mutable = cont.mutable;
            let kind = match &cont.kind {
                ContentKind::Number(n) => match n {
                    content::Number::Int(i) => ObjectKind::Number(super::Number::Int(*i)),
                    content::Number::Float(f) => ObjectKind::Number(super::Number::Float(*f)),
                }
                ContentKind::Boolean(b) => ObjectKind::Boolean(*b),
                ContentKind::String(s) => ObjectKind::String(s.clone()),
                ContentKind::Symbol(s) => ObjectKind::Symbol(s.clone()),
                ContentKind::Empty => ObjectKind::Empty,
                ContentKind::Pair{..} => ObjectKind::Pair,
                ContentKind::Procedure(proc) => match proc {
                    content::Procedure::Proc{env, args, is_variadic, require, body} => 
                        ObjectKind::Procedure(super::Procedure::Proc { 
                            env: Arc::clone(env), 
                            args: args.clone(), 
                            is_variadic: *is_variadic, 
                            require: *require, 
                            body: body.clone()
                        }),
                    content::Procedure::Subr{is_variadic, require, fun} =>
                        ObjectKind::Procedure(super::Procedure::Subr {
                            is_variadic: *is_variadic,
                            require: *require,
                            fun: fun.clone()
                        })
                }
                ContentKind::Undefined => ObjectKind::Undefined,
            };
            (mutable, kind)
        } else {
            return None;
        }
    };
    Some(Object{pointer, mutable, kind})
}
