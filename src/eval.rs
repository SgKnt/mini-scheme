use std::rc::Rc;
use std::cell::RefCell;

use anyhow::{Context, Result, anyhow, bail};

use super::object::*;
use super::token::*;
use super::env::*;

pub fn eval(token: &Token, env: &RefCell<Rc<Environment>>) -> Result<Object> {
    todo!()
}

fn eval_exp(token: &Token, env: &RefCell<Rc<Environment>>) -> Result<Rc<Object>> {
    match token {
        &Token::Int(i) => Ok(Rc::new(Object{kind: ObjectKind::Number(NumberKind::Int(i))})),
        &Token::Float(f) => Ok(Rc::new(Object{kind: ObjectKind::Number(NumberKind::Float(f))})),
        &Token::Boolean(b) => Ok(Rc::new(Object{kind: ObjectKind::Boolean(b)})),
        Token::String(s) => Ok(Rc::new(Object{kind: ObjectKind::String(s.clone())})),
        &Token::Empty => Ok(Rc::new(Object{kind: ObjectKind::Empty})),
        Token::Symbol(s) => eval_quote(&*s, env),
        Token::Id(id) => if let Some(var) = env.borrow().variables.borrow().get(id) {
            Ok(Rc::clone(&var.value.borrow()))
        } else {
            Err(anyhow!("unbound variable: {}", id))
        },
        Token::Pair{car, cdr} => match &**car {
            Token::Id(id) => {
                if env.borrow().variables.borrow().get(id).is_some() {
                    eval_app(car, cdr, env)
                } else {
                    match id.as_str() {
                        "lambda" => {
                            todo!()
                        },
                        "quote" => {
                            eval_quote(cdr, env)
                        },
                        "set!" => {
                            todo!()
                        }
                        "let" => {
                            todo!()
                        }
                        "let*" => {
                            todo!()
                        }
                        "letrec" => {
                            todo!()
                        }
                        "if" => {
                            todo!()
                        }
                        "cond" => {
                            todo!()
                        }
                        "and" => {
                            todo!()
                        }
                        "or" => {
                            todo!()
                        }
                        "begin" => {
                            todo!()
                        }
                        "do" => {
                            todo!()
                        }
                        _ => Err(anyhow!("unbound variable: {}", id))
                    }
                }
            }, 
            Token::Pair{..} => {
                eval_app(car, cdr, env)
            }
            _ => {
                Err(anyhow!("invalid application: {}", token))
            }
        }
    }
}

fn eval_quote(token: &Token, env: &RefCell<Rc<Environment>>) -> Result<Rc<Object>> {
    // the token must be the elements of Token::Symbol
    match token {
        &Token::Int(i) => Ok(Rc::new(Object{kind: ObjectKind::Number(NumberKind::Int(i))})),
        &Token::Float(f) => Ok(Rc::new(Object{kind: ObjectKind::Number(NumberKind::Float(f))})),
        &Token::Boolean(b) => Ok(Rc::new(Object{kind: ObjectKind::Boolean(b)})),
        Token::String(s) => Ok(Rc::new(Object{kind: ObjectKind::String(s.clone())})),
        &Token::Empty => Ok(Rc::new(Object{kind: ObjectKind::Empty})),
        Token::Symbol(_) => Ok(Rc::new(Object{kind: ObjectKind::Symbol(format!("{}", token))})),
        Token::Id(id) => Ok(Rc::new(Object{kind: ObjectKind::Symbol(format!("{}", id))})),
        Token::Pair{car, cdr} => Ok(Rc::new(Object{kind: ObjectKind::Pair{
            car: Ref::Rc(RefCell::new(eval_quote(&**car, env)?)), 
            cdr: Ref::Rc(RefCell::new(eval_quote(&**cdr, env)?))
        }})),
    }
}

fn eval_app(proc: &Token, args: &Token, env: &RefCell<Rc<Environment>>) -> Result<Rc<Object>> {
    todo!()
}