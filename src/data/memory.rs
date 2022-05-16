use super::object::{ObjBody, Kind, Procedure};
use super::env::EnvBody;
use super::r#ref::{ObjRef, EnvRef};

use std::cell::Cell;

pub(crate) struct Memory {
    obj_mem: Vec<Box<ObjBody>>,
    env_mem: Vec<Box<EnvBody>>,
    obj_size: usize,
    env_size: usize,
    max_size: usize,
    initialized: bool,
}
pub(crate) enum Marker {
    Black,  // alive, finish
    Gray,   // alive, searching
    White,  // maybe dead
}

pub(crate) static mut MEMORY: Memory = Memory{
    obj_mem: Vec::new(),
    env_mem: Vec::new(),
    obj_size: 0,
    env_size: 0,
    max_size: 0,
    initialized: false,
};

impl Memory {
    pub fn init(max: usize) {
        unsafe {
            MEMORY.max_size = max;
            MEMORY.obj_mem.push(Box::new(ObjBody{
                is_mutable: false,
                kind: Kind::Empty,
                mark: Marker::Black,
                rc: Cell::new(1),
            }));
            MEMORY.initialized = true;
        }
    }

    fn ensure_initialized() {
        unsafe {
            if !MEMORY.initialized {
                panic!("Memory has not yet initialized. Call crate::data::memory::Memory::init(usize). ");
            }
        }
    }

    pub(crate) fn push_obj(obj: ObjBody) -> ObjRef {
        Self::ensure_initialized();
        let mut obj = Box::new(obj);
        let re = ObjRef::new(obj.as_mut());
        unsafe {
            if MEMORY.obj_size == MEMORY.max_size-1 {
                Self::gc();
            }
            MEMORY.obj_size += 1;
            MEMORY.obj_mem.push(obj);
        }
        re
    }

    pub(crate) fn push_env(env: EnvBody) -> EnvRef {
        Self::ensure_initialized();
        let mut env = Box::new(env);
        let re = EnvRef::new(env.as_mut());
        unsafe {
            if MEMORY.env_size == MEMORY.max_size-1 {
                Self::gc();
            }
            MEMORY.env_size += 1;
            MEMORY.env_mem.push(env);
        }
        re
    }

    pub(crate) fn get_empty() -> ObjRef {
        Self::ensure_initialized();
        unsafe {
            let mut empty = &mut MEMORY.obj_mem[0];
            empty.inc_rc();
            ObjRef::new(&mut empty)
        }
    }

    pub fn gc() {
        Memory::ensure_initialized();
        Self::mark();
        Self::sweep();
        unsafe {
            MEMORY.obj_size = MEMORY.obj_mem.len();
            MEMORY.env_size = MEMORY.env_mem.len();
            if MEMORY.obj_size == MEMORY.max_size {
                panic!("Memory overflow")
            }
        }
    }
    
    fn mark() {
        unsafe {
            // mark obj, env in stack
            for obj in &mut MEMORY.obj_mem {
                if obj.rc.get() > 0 {
                    obj.mark = Marker::Gray;
                } else {
                    obj.mark = Marker::White;
                }
            }
            for env in &mut MEMORY.env_mem {
                if env.rc.get() > 0 {
                    env.mark = Marker::Gray;
                } else {
                    env.mark = Marker::White;
                }
            }
    
            // mark obj, env which can be reached from obj, env in stack
            for obj in &mut MEMORY.obj_mem {
                if let Marker::Gray = obj.mark {
                    obj.mark = Marker::Black;
                    match &obj.kind {
                        Kind::Pair(pair) => {
                            Self::mark_obj(&pair.car);
                            Self::mark_obj(&pair.cdr);
                        }
                        Kind::Procedure(proc) => {
                            if let Procedure::Proc(proc) = proc {
                                Self::mark_env(&proc.env);
                            }
                        }
                        _ => {}
                    }
                }
            }
    
            for env in &mut MEMORY.env_mem {
                if let Marker::Gray = env.mark {
                    env.mark = Marker::Black;
                    for obj in env.vars.values() {
                        Self::mark_obj(obj);
                    }
                    if let Some(parent) = &env.parent {
                        Self::mark_env(parent);
                    }
                }
            }
        }
    }
    
    fn mark_obj(obj: &ObjRef) {
        if let Marker::Black = &obj.borrow().mark {
            return
        }
    
        unsafe {obj.borrow_mut().mark = Marker::Black};
        match &obj.borrow().kind {
            Kind::Pair(pair) => {
                Self::mark_obj(&pair.car);
                Self::mark_obj(&pair.cdr);
            }
            Kind::Procedure(proc) => {
                if let Procedure::Proc(proc) = proc {
                    Self::mark_env(&proc.env);
                }
            }
            _ => {}
        }
    }
    
    fn mark_env(env: &EnvRef) {
        if let Marker::Black = &env.borrow().mark {
            return;
        }
    
        unsafe {env.borrow_mut().mark = Marker::Black};
        for obj in env.borrow().vars.values() {
            Self::mark_obj(obj);
        }

        if let Some(parent) = &env.borrow().parent {
            Self::mark_env(parent);
        }
    }
    
    fn sweep() {
        unsafe {
            MEMORY.obj_mem.retain(|obj| matches!(obj.mark, Marker::Black));
            MEMORY.env_mem.retain(|env| matches!(env.mark, Marker::Black));
        }
    }
}
