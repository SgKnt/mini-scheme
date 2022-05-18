use super::{Object, Environment};
use super::{object::*, env::EnvBody};

use std::collections::{HashMap, HashSet};
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
        // Floyd's cycle-finding algorithm
        let mut fast = self;
        let mut slow = self;
        loop {
            match &fast.borrow().kind {
                Kind::Pair(pair) => {
                    fast = &pair.cdr;
                    if fast == slow {
                        break false;
                    }
                    match &fast.borrow().kind {
                        Kind::Pair(pair) => fast = &pair.cdr,
                        Kind::Empty => break true,
                        _ => break false,
                    }
                    match &slow.borrow().kind {
                        Kind::Pair(pair) => slow = &pair.cdr,
                        _ => unreachable!(),
                    }
                }
                Kind::Empty => break true,
                _ => break false,
            }
        }
    }

    pub(crate) fn length(&self) -> Option<usize> {
        // Floyd's cycle-finding algorithm
        let mut len: usize = 0;
        let mut fast = self;
        let mut slow = self;
        loop {
            match &fast.borrow().kind {
                Kind::Pair(pair) => {
                    len += 1;
                    fast = &pair.cdr;
                    if fast == slow {
                        break None;
                    }
                    match &fast.borrow().kind {
                        Kind::Pair(pair) => {
                            len += 1;
                            fast = &pair.cdr;
                        }
                        Kind::Empty => break Some(len),
                        _ => break None,
                    }
                    match &slow.borrow().kind {
                        Kind::Pair(pair) => slow = &pair.cdr,
                        _ => unreachable!(),
                    }
                }
                Kind::Empty => break Some(len),
                _ => break None,
            }
        }
    }

    pub(crate) fn scm_eq(&self, other: &ObjRef) -> bool {
        match (&self.borrow().kind, &other.borrow().kind) {
            (Kind::Number(lhs), Kind::Number(rhs)) => match (lhs, rhs) {
                (Number::Int(lhs), Number::Int(rhs)) => lhs == rhs,
                (Number::Float(lhs), Number::Float(rhs)) => lhs == rhs,
                (_, _) => false,
            }
            (Kind::Boolean(lhs), Kind::Boolean(rhs)) => lhs == rhs,
            (Kind::Symbol(lhs), Kind::Symbol(rhs)) => lhs == rhs,
            (Kind::Empty, Kind::Empty) => true,
            (_, _) => self == other, 
        }
    }

    pub(crate) fn scm_equal(&self, other: &ObjRef) -> bool {
        match (&self.borrow().kind, &other.borrow().kind) {
            (Kind::Number(lhs), Kind::Number(rhs)) => match (lhs, rhs) {
                (Number::Int(lhs), Number::Int(rhs)) => lhs == rhs,
                (Number::Float(lhs), Number::Float(rhs)) => lhs == rhs,
                (_, _) => false,
            }
            (Kind::Boolean(lhs), Kind::Boolean(rhs)) => lhs == rhs,
            (Kind::Symbol(lhs), Kind::Symbol(rhs)) => lhs == rhs,
            (Kind::String(lhs), Kind::String(rhs)) => lhs == rhs,
            (Kind::Empty, Kind::Empty) => true,
            (Kind::Pair(lhs), Kind::Pair(rhs))
                => ObjRef::scm_equal(&lhs.car, &rhs.car) && ObjRef::scm_equal(&lhs.cdr, &rhs.cdr),
            (_, _) => self == other,
        }
    }
}

struct ObjRefDisplayState {
    obj_tag: HashMap<ObjRef, usize>,
    exists: HashSet<ObjRef>, 
}

impl ObjRef {
    pub fn to_string(&self) -> String {
        
        let mut state = ObjRefDisplayState{obj_tag: HashMap::new(), exists: HashSet::new()};
        self._to_string(&mut state)
    }

    fn _to_string(&self, state: &mut ObjRefDisplayState) -> String {
        match &self.borrow().kind {
            Kind::Number(num) => match num {
                Number::Int(i) => i.to_string(),
                Number::Float(f) => f.to_string(),
            }
            Kind::Boolean(b) if *b => "#t".to_string(),
            Kind::Boolean(_) => "#f".to_string(),
            Kind::String(s) => format!(r#""{}""#, s),
            Kind::Symbol(s) => s.clone(),
            Kind::Empty => "()".to_string(),
            Kind::Procedure(proc) => match proc {
                Procedure::Proc(_) => "#<procedure>".to_string(),
                Procedure::Subr(_) => "#<subroutine>".to_string(),
            }
            Kind::Undefined => "#<undef>".to_string(),
            Kind::Pair(pair) => {
                if let Some(tag) = state.obj_tag.get(self) {
                    format!("#{}#", tag)
                } else if let Some(_) = state.exists.take(self) {
                    let tag = state.obj_tag.len();
                    state.obj_tag.insert(*self, tag);
                    format!("#{}#", tag)
                } else {
                    state.exists.insert(*self);
                    let car = pair.car._to_string(state);
                    let cdr = pair.cdr._to_string(state);
                    if let Some(tag) = state.obj_tag.get(self) {
                        if cdr == "()" {
                            format!("#{}=({})", tag, car)
                        } else if cdr.starts_with('(') {
                            format!("#{}=({} {}", tag, car, cdr.split_at(1).1)
                        } else {
                            format!("#{}=({} . {})", tag, car, cdr)
                        }
                    } else {
                        if cdr == "()" {
                            format!("({})", car)
                        } else if cdr.starts_with('(') {
                            format!("({} {}", car, cdr.split_at(1).1)
                        } else {
                            format!("({} . {})", car, cdr)
                        }
                    }
                }
            }
        }
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
