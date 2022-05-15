use super::object::{ObjBody, Kind};
use super::env::EnvBody;
use super::r#ref::{ObjRef, EnvRef};
use super::gc::Marker;

use std::cell::Cell;

pub struct Memory {
    obj_mem: Vec<ObjBody>,
    env_mem: Vec<EnvBody>,
    obj_size: usize,
    env_size: usize,
    max_size: usize,
    initialized: bool,
}

pub static mut MEMORY: Memory = Memory{
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
            MEMORY.obj_mem.push(ObjBody{
                is_mutable: false,
                kind: Kind::Empty,
                mark: Marker::Black,
                rc: Cell::new(1),
            })
        }
    }

    fn ensure_initialized() {
        unsafe {
            if !MEMORY.initialized {
                panic!("Memory has not yet initialized. Call crate::data::memory::Memory::init(usize). ");
            }
        }
    }

    pub(crate) fn push_obj(mut obj: ObjBody) -> ObjRef {
        Self::ensure_initialized();
        let re = ObjRef::new(&mut obj);
        unsafe {
            if MEMORY.obj_size-1 == MEMORY.max_size {
                // gc
                todo!();
            }
            MEMORY.obj_size += 1;
            MEMORY.obj_mem.push(obj);
        }
        re
    }

    pub(crate) fn push_env(mut env: EnvBody) -> EnvRef {
        Self::ensure_initialized();
        let re = EnvRef::new(&mut env);
        unsafe {
            if MEMORY.env_size-1 == MEMORY.max_size {
                // gc
                todo!();
            }
            MEMORY.env_size += 1;
            MEMORY.env_mem.push(env);
        }
        re
    }

    pub fn get_empty() -> ObjRef {
        Self::ensure_initialized();
        unsafe {
            let mut empty = &mut MEMORY.obj_mem[0];
            empty.inc_rc();
            ObjRef::new(&mut empty)
        }
    }
}
