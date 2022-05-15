use super::r#ref::{ObjRef, EnvRef};
use super::memory::Marker;

use std::cell::Cell;
use std::collections::HashMap;

pub(crate) struct EnvBody {
    pub vars: HashMap<String, ObjRef>,
    pub parent: Option<EnvRef>,
    pub mark: Marker,
    pub rc: Cell<u32>,
}

impl EnvBody {
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
            panic!("An EnvBody which is already dead has been borrowed.");
        }
        self.rc.set(rc - 1);
    }
}
