use super::{Object, Environment};
use super::{object::ObjBody, env::EnvBody};

use std::ptr::NonNull;

#[derive(Clone, Copy)]
pub struct ObjRef(NonNull<ObjBody>);

impl ObjRef {
    pub(crate) fn new(obj: &mut ObjBody) -> Self {
        ObjRef(
            unsafe {NonNull::new_unchecked(obj as *mut _)}
        )
    }

    #[inline]
    pub(crate) fn borrow(&self) -> &ObjBody {
        unsafe {self.0.as_ref()}
    }

    #[inline]
    pub(crate) unsafe fn borrow_mut(&self) -> &mut ObjBody {
        &mut *self.0.as_ptr()
    }
}

#[derive(Clone, Copy)]
pub struct EnvRef(NonNull<EnvBody>);

impl EnvRef {
    pub(crate) fn new(env: &mut EnvBody) -> Self {
        EnvRef(
            unsafe {NonNull::new_unchecked(env as *mut _)}
        )
    }

    pub(crate) fn lookup(&self, id: &String) -> Option<Object> {
        if let Some(re) = self.borrow().vars.get(id) {
            re.borrow().inc_rc();
            Some(Object{re: *re})
        } else if let Some(parent) = self.borrow().parent {
            parent.lookup(id)
        } else {
            None
        }
    }

    pub fn contains_at(&self, id: &String) -> Option<Environment> {
        if let Some(_) = self.borrow().vars.get(id) {
            self.borrow().inc_rc();
            Some(Environment{re: *self})
        } else if let Some(parent) = self.borrow().parent {
            parent.contains_at(id)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn borrow(&self) -> &EnvBody {
        unsafe {self.0.as_ref()}
    }

    #[inline]
    pub(crate) unsafe fn borrow_mut(&self) -> &mut EnvBody {
        &mut *self.0.as_ptr()
    }
}
