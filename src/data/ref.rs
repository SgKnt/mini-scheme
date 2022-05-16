use super::{Object, Environment};
use super::{object::*, env::EnvBody};

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    pub(crate) fn is_list(&self) -> bool {
        match &self.borrow().kind {
            Kind::Pair(pair) => pair.car.is_list(),
            Kind::Empty => true,
            _ =>false,
        }
    }
}

struct ObjRefDisplayState {
    obj_tag: HashMap<ObjRef, usize>,
    exists: HashSet<ObjRef>, 
}

impl ObjRef {
    fn display(&self, f: &mut Formatter<'_>, state: &mut ObjRefDisplayState) -> std::fmt::Result {
        match &self.borrow().kind {
            Kind::Number(num) => match num {
                Number::Int(i) => write!(f, "{}", i),
                Number::Float(fl) => write!(f, "{}", fl),
            }
            Kind::Boolean(b) if *b => write!(f, "#t"),
            Kind::Boolean(_) => write!(f, "#f"),
            Kind::String(s) => write!(f, r#""{}""#, s),
            Kind::Symbol(s) => write!(f, "{}", s),
            Kind::Empty => write!(f, "()"),
            Kind::Procedure(proc) => match proc {
                Procedure::Proc(_) => write!(f, "#<procedure>"),
                Procedure::Subr(_) => write!(f, "#<subroutine>"),
            }
            Kind::Undefined => write!(f, "#<undef>"),
            Kind::Pair(pair) => {
                if let Some(tag) = state.obj_tag.get(self) {
                    write!(f, "#{}#", tag)
                } else if let Some(_) = state.exists.take(self) {
                    let tag = state.obj_tag.len();
                    state.obj_tag.insert(*self, tag);
                    write!(f, "#{}#", tag)
                } else {
                    state.exists.insert(*self);
                    let car = format!("{}", pair.car);
                    let cdr = format!("{}", pair.cdr);
                    if let Some(tag) = state.obj_tag.get(self) {
                        if cdr == "()" {
                            write!(f, "#{}=({})", tag, car)
                        } else if cdr.starts_with('(') {
                            write!(f, "#{}=({} {}", tag, car, cdr.split_at(1).1)
                        } else {
                            write!(f, "#{}=({} . {})", tag, car, cdr)
                        }
                    } else {
                        if cdr == "()" {
                            write!(f, "({})", car)
                        } else if cdr.starts_with('(') {
                            write!(f, "({} {}", car, cdr.split_at(1).1)
                        } else {
                            write!(f, "({} . {})", car, cdr)
                        }
                    }
                }
            }
        }
    }
}

impl Display for ObjRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = ObjRefDisplayState{obj_tag: HashMap::new(), exists: HashSet::new()};
        self.display(f, &mut state)
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
