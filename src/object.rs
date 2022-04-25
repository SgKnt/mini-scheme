use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;

use crate::env::Environment;

pub struct Object {
    pub kind: ObjectKind
}

pub enum ObjectKind {
    Number(NumberKind),
    Boolean(bool),
    Pair{
        car: Ref,
        cdr: Ref
    },
    Empty,
    Procedure(Procedure),
    Symbol(String),
    String(String),
    Undefined,
}

impl Object {
    pub fn is_falsy(&self) -> bool {
        if let ObjectKind::Boolean(false) = self.kind {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ObjectKind::Number(num) => match num {
                NumberKind::Int(i) => write!(f, "{}", i),
                NumberKind::Float(fl) => write!(f, "{}", fl),
            }
            ObjectKind::Boolean(b) => {
                if *b {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            ObjectKind::Procedure(_) => write!(f, "#<procedure>"),
            ObjectKind::String(s) => write!(f, "\"{}\"", s),
            ObjectKind::Symbol(s) => write!(f, "{}", s),
            ObjectKind::Undefined => write!(f, "#<undef>"),
            ObjectKind::Empty => write!(f, "()"),
            ObjectKind::Pair{car, cdr} => match car {
                Ref::Rc(car) => match cdr {
                    Ref::Rc(cdr) => match cdr.borrow().kind {
                        ObjectKind::Pair{..} => {
                            let cdr = format!("{}", cdr.borrow());
                            write!(f, "({} {}", car.borrow(), cdr.split_at(1).1)
                        }
                        ObjectKind::Empty => write!(f, "({})", car.borrow()),
                        _ => write!(f, "({} {})", car.borrow(), cdr.borrow())
                    }
                    Ref::Weak(cdr) => match cdr.borrow().upgrade().unwrap().kind {
                        ObjectKind::Pair{..} => {
                            let cdr = format!("{}", cdr.borrow().upgrade().unwrap());
                            write!(f, "({} {}", car.borrow(), cdr.split_at(1).1)
                        }
                        ObjectKind::Empty => write!(f, "({})", car.borrow()),
                        _ => write!(f, "({} {})", car.borrow(), cdr.borrow().upgrade().unwrap())
                    }
                }
                Ref::Weak(car) => match cdr {
                    Ref::Rc(cdr) => match cdr.borrow().kind {
                        ObjectKind::Pair{..} => {
                            let cdr = format!("{}", cdr.borrow());
                            write!(f, "({} {}", car.borrow().upgrade().unwrap(), cdr.split_at(1).1)
                        }
                        ObjectKind::Empty => write!(f, "({})", car.borrow().upgrade().unwrap()),
                        _ => write!(f, "({} {})", car.borrow().upgrade().unwrap(), cdr.borrow())
                    }
                    Ref::Weak(cdr) => match cdr.borrow().upgrade().unwrap().kind {
                        ObjectKind::Pair{..} => {
                            let cdr = format!("{}", cdr.borrow().upgrade().unwrap());
                            write!(f, "({} {}", car.borrow().upgrade().unwrap(), cdr.split_at(1).1)
                        }
                        ObjectKind::Empty => write!(f, "({})", car.borrow().upgrade().unwrap()),
                        _ => write!(f, "({} {})", car.borrow().upgrade().unwrap(), cdr.borrow().upgrade().unwrap())
                    }
                }
            }
        }
    }
}

pub enum Ref {
    Rc(RefCell<Rc<Object>>),
    Weak(RefCell<Weak<Object>>)
}


pub enum NumberKind {
    Int(i64),
    Float(f64)
}

pub struct Procedure {
    pub env: RefCell<Rc<Environment>>
}
