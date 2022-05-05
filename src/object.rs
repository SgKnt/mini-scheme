use std::borrow::Borrow;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;

use anyhow::Result;

use crate::env::Environment;
use crate::token::Token;

pub enum Object {
    Number(NumberKind),
    Boolean(bool),
    Pair{
        car: Ref,
        cdr: Ref
    },
    Empty,
    Procedure(Procedure),
    Subroutine(Subroutine),
    Symbol(String),
    String(String),
    Undefined,
}

pub enum NumberKind {
    Int(i64),
    Float(f64)
}

pub enum Ref {
    Rc(Rc<RefCell<Object>>),
    Weak(Weak<RefCell<Object>>)
}

pub struct Procedure {
    pub env: Rc<Environment>,
    pub args: Args,
    pub body: Token
}

pub struct Args {
    pub ids: Vec<String>,
    pub is_variadic: bool,
    pub required: usize,
}

pub struct Subroutine {
    pub args: Args,
    pub fun: fn(VecDeque<Result<Rc<RefCell<Object>>>>, &Rc<Environment>) -> Rc<RefCell<Object>>
}

impl Object {
    pub fn is_falsy(&self) -> bool {
        if let Object::Boolean(false) = self {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Object::Number(num) => match num {
                NumberKind::Int(i) => write!(f, "{}", i),
                NumberKind::Float(fl) => write!(f, "{}", fl),
            }
            Object::Boolean(b) => {
                if *b {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            Object::Procedure(_) => write!(f, "#<procedure>"),
            Object::Subroutine(_) => write!(f, "#<subroutine>"),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Undefined => write!(f, "#<undef>"),
            Object::Empty => write!(f, "()"),
            Object::Pair{car, cdr} => match car {
                Ref::Rc(car) => match cdr {
                    Ref::Rc(cdr) =>  {
                        let cdr = (**cdr).borrow();
                        match &*cdr {
                            Object::Pair{..} => {
                                let cdr = format!("{}", cdr);
                                write!(f, "({} {}", (**car).borrow(), cdr.split_at(1).1)
                            }
                            Object::Empty => write!(f, "({})", (**car).borrow()),
                            _ => write!(f, "({} {})", (**car).borrow(), cdr)
                        }
                    }
                    Ref::Weak(cdr) =>  {
                        let cdr = cdr.upgrade().unwrap();
                        let cdr = (*cdr).borrow();
                        match &*cdr {
                            Object::Pair{..} => {
                                let cdr = format!("{}", cdr);
                                write!(f, "({} {}", (**car).borrow(), cdr.split_at(1).1)
                            }
                            Object::Empty => write!(f, "({})", (**car).borrow()),
                            _ => write!(f, "({} {})", (**car).borrow(), cdr)
                        }
                    }
                }
                Ref::Weak(car) => match cdr {
                    Ref::Rc(cdr) => {
                        let cdr = (**cdr).borrow();
                        match &*cdr {
                            Object::Pair{..} => {
                                let cdr = format!("{}", cdr.borrow());
                                write!(f, "({} {}", (*car.upgrade().unwrap()).borrow(), cdr.split_at(1).1)
                            }
                            Object::Empty => write!(f, "({})", (*car.upgrade().unwrap()).borrow()),
                            _ => write!(f, "({} {})", (*car.upgrade().unwrap()).borrow(), cdr.borrow())
                        }
                    }
                    Ref::Weak(cdr) => {
                        let cdr = cdr.upgrade().unwrap();
                        let cdr = (*cdr).borrow();
                        match &*cdr {
                            Object::Pair{..} => {
                                let cdr = format!("{}", cdr);
                                write!(f, "({} {}", (*car.upgrade().unwrap()).borrow(), cdr.split_at(1).1)
                            }
                            Object::Empty => write!(f, "({})", (*car.upgrade().unwrap()).borrow()),
                            _ => write!(f, "({} {})", (*car.upgrade().unwrap()).borrow(), cdr)
                        }
                    }
                }
            }
        }
    }
}




