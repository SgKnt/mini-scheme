
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use anyhow::Result;

use crate::env::Environment;
use crate::token::Token;

pub enum Object {
    Number(NumberKind),
    Boolean(bool),
    Pair{
        car: Rc<RefCell<Object>>,
        cdr: Rc<RefCell<Object>>,
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
    pub required: usize,
    pub is_variadic: bool,
    pub fun: fn(VecDeque<Result<Rc<RefCell<Object>>>>, &Rc<Environment>) -> Result<Rc<RefCell<Object>>>
}

impl Object {
    pub fn is_falsy(&self) -> bool {
        if let Object::Boolean(false) = self {
            true
        } else {
            false
        }
    }

    pub fn eq_scm(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(lhs), Object::Number(rhs)) => {
                match (lhs, rhs) {
                    (NumberKind::Int(lhs), NumberKind::Int(rhs)) => lhs == rhs,
                    (NumberKind::Float(lhs), NumberKind::Float(rhs)) => lhs == rhs,
                    (_, _) => false
                }
            }
            (Object::Boolean(lhs), Object::Boolean(rhs)) => lhs == rhs,
            (Object::Symbol(lhs), Object::Symbol(rhs)) => lhs == rhs,
            (Object::Empty, Object::Empty) => true,
            (lhs, rhs) => (lhs as *const Self) == (rhs as *const Self),
        }
    }

    pub fn equal_scm(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(lhs), Object::Number(rhs)) => {
                match (lhs, rhs) {
                    (NumberKind::Int(lhs), NumberKind::Int(rhs)) => lhs == rhs,
                    (NumberKind::Float(lhs), NumberKind::Float(rhs)) => lhs == rhs,
                    (_, _) => false
                }
            }
            (Object::Boolean(lhs), Object::Boolean(rhs)) => lhs == rhs,
            (Object::Symbol(lhs), Object::Symbol(rhs)) => lhs == rhs,
            (Object::String(lhs), Object::String(rhs)) => lhs == rhs,
            (Object::Empty, Object::Empty) => true,
            (Object::Pair{car: lcar, cdr: lcdr}, 
                Object::Pair{car: rcar, cdr: rcdr})
                => lcar.borrow().equal_scm(&*rcar.borrow()) && lcdr.borrow().equal_scm(&*rcdr.borrow()),
            (lhs, rhs) => (lhs as *const Self) == (rhs as *const Self),
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
            Object::Pair{car, cdr} => match &*(**cdr).borrow(){
                Object::Pair{..} => {
                    let cdr = format!("{}", (**cdr).borrow());
                    write!(f, "({} {}", (**car).borrow(), cdr.split_at(1).1)
                }
                Object::Empty => {
                    write!(f, "({})", (**car).borrow())
                }
                _ => write!(f, "({} . {})", (**car).borrow(), (**cdr).borrow())
            }
        }
    }
}
