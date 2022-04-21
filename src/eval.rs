use std::rc::Rc;
use std::cell::RefCell;

use anyhow::{Context, Result, anyhow, bail};

use super::object::*;
use super::token::*;
use super::env::*;

pub fn eval(token: &Token, env: &RefCell<Rc<Environment>>) -> Result<Object> {
    // Exp, Define, (load String)
    todo!()
}

fn eval_exp(token: &Token, env: &Rc<Environment>) -> Result<Rc<Object>> {
    match token {
        &Token::Int(i) => Ok(Rc::new(Object{kind: ObjectKind::Number(NumberKind::Int(i))})),
        &Token::Float(f) => Ok(Rc::new(Object{kind: ObjectKind::Number(NumberKind::Float(f))})),
        &Token::Boolean(b) => Ok(Rc::new(Object{kind: ObjectKind::Boolean(b)})),
        Token::String(s) => Ok(Rc::new(Object{kind: ObjectKind::String(s.clone())})),
        &Token::Empty => Ok(Rc::new(Object{kind: ObjectKind::Empty})),
        Token::Symbol(s) => eval_quote(&*s),
        Token::Id(id) => if let Some(var) = env.lookup(id) {
            Ok(Rc::clone(&var))
        } else {
            Err(anyhow!("unbound variable: {}", id))
        },
        Token::Pair{car, cdr} => match &**car {
            Token::Id(id) => {
                if env.variables.borrow().get(id).is_some() {
                    eval_app(car, cdr, env)
                } else {
                    match id.as_str() {
                        "lambda" => {
                            todo!()
                        },
                        "quote" => {
                            eval_quote(cdr)
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
                            // (if exp1 exp2 exp3)
                            let exp1 = cdr.elem()
                                .with_context(|| format!("error: proper list required for function application or macro use: {}", token))?;
                            let exp2 = cdr.next()
                                .map(|t| t.elem().with_context(|| format!("syntax error: malformed if: {}", token))) // (if foo)
                                .with_context(|| format!("error: proper list required for function application or macro use: {}", token))??; // (if foo . bar)
                            let exp3 = cdr.next().unwrap();
                            let cond = eval_exp(exp1, env)?;
                            match exp3 {
                                Token::Pair{car: exp3_car, cdr: exp3_cdr} => {
                                    if !exp3_cdr.is_empty() {
                                        Err(anyhow!("syntax error: malformed if: {}", token))
                                    } else if !cond.is_falsy() {
                                        eval_exp(exp2, env)
                                    } else {
                                        eval_exp(exp3_car, env)
                                    }
                                }
                                Token::Empty => {
                                    if !cond.is_falsy() {
                                        eval_exp(exp2, env)
                                    } else {
                                        Ok(Rc::new(Object{kind: ObjectKind::Undefined}))
                                    }
                                }
                                _ => {
                                    Err(anyhow!("error: proper list required for function application or macro use: {}", token))
                                }
                            }
                        }
                        "cond" => {
                            if !cdr.is_list() {
                                bail!("error: proper list required for function application or macro use: {}", token);
                            }
                            if cdr.is_empty() {
                                bail!("syntax error: at least one clause is required for cond: {}", token);
                            }

                            let mut res = Err(anyhow!("interpreter error at {}", line!()));
                            for clause in &**cdr {
                                if let Token::Pair{car: test, cdr: exps} = clause.elem()
                                        .with_context(|| format!("syntax error: bad clause in cond: {}", token))? {
                                    if exps.is_empty() || !exps.is_list() {
                                        bail!("syntax error: bad clause in cond: {}", token);
                                    } else {
                                        match &**test {
                                            Token::Id(s) if &*s == "else" => {
                                                for exp in &**exps {
                                                    res = Ok(eval_exp(exp, env)?);
                                                }
                                            }
                                            _ => {
                                                if !eval_exp(test, env)?.is_falsy() {
                                                    for exp in &**exps {
                                                        res = Ok(eval_exp(exp, env)?);
                                                    }
                                                } else {
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            res
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

fn eval_quote(token: &Token) -> Result<Rc<Object>> {
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
            car: Ref::Rc(RefCell::new(eval_quote(&**car)?)), 
            cdr: Ref::Rc(RefCell::new(eval_quote(&**cdr)?))
        }})),
    }
}

fn eval_app(proc: &Token, args: &Token, env: &Rc<Environment>) -> Result<Rc<Object>> {
    todo!()
}