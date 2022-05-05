use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

use anyhow::{Context, Result, anyhow, bail};

use super::object::*;
use super::token::*;
use super::env::*;

pub fn eval(token: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // Exp, Define, (load String)
    match token {
        Token::Pair{car, cdr} => match &**car {
            Token::Id(id) if id == "define" => {
                eval_define(token, env)
            }
            Token::Id(id) if id == "load" => {
                eval_load(&**cdr, env)
            }
            _ => eval_exp(token, env)
        }
        _ => eval_exp(token, env)
    }
}

pub fn eval_define(token: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let def_token = token;
    let token = token.next().unwrap();
    ensure_proper_list(token)?;

    let ids = token.elem().with_context(|| format!("syntax error: {}", def_token))?;
    match ids {
        Token::Id(id) => {
            let obj = token
                .next()
                .unwrap()
                .elem()
                .map(|t| eval_exp(t, env))
                .unwrap_or_else(|| Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Undefined}))))?;
            env.variables.borrow_mut().insert(id.clone(), obj);
            Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Symbol(id.clone())})))
        }
        Token::Pair{car: id, cdr: args} => {
            if let Token::Id(id) = &**id {
                let body = token.next().unwrap();
                let obj = eval_lambda(token, args, body, env)?;
                env.variables.borrow_mut().insert(id.clone(), obj);
                Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Symbol(id.clone())})))
            } else {
                Err(anyhow!("syntax error: {}", def_token))
            }
        }
        _ => Err(anyhow!("syntax error: {}", def_token))
    }
}

pub fn eval_load(token: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    todo!()
}

fn eval_exp(token: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match token {
        &Token::Int(i) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Number(NumberKind::Int(i))}))),
        &Token::Float(f) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Number(NumberKind::Float(f))}))),
        &Token::Boolean(b) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Boolean(b)}))),
        Token::String(s) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::String(s.clone())}))),
        Token::Empty => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Empty}))),
        Token::Symbol(s) => eval_quote(&*s),
        Token::Id(id) => if let Some(var) = env.lookup(id) {
            Ok(Rc::clone(&var))
        } else {
            Err(anyhow!("unbound variable: {}", id))
        },
        Token::Pair{car, cdr} => match &**car {
            Token::Id(id) => {
                if env.variables.borrow().get(id).is_some() {
                    eval_app(token, car, cdr, env)
                } else {
                    match id.as_str() {
                        "lambda" => {
                            ensure_proper_list(cdr)?;
                            let arg = cdr.elem().with_context(|| format!("syntax error: malformed lambda: {}", token))?;
                            let body = cdr.next().unwrap();
                            eval_lambda(token, arg, body, env)
                        },
                        "quote" => {
                            eval_quote(cdr)
                        },
                        "set!" => {
                            ensure_proper_list(token)?;
                            let id = token
                                .nth(1)
                                .with_context(|| format!("syntax error: malformed set!: {}", token))?;
                            if let Token::Id(id) = id {
                                let exp = eval_exp(
                                    token
                                    .nth(2)
                                    .with_context(|| format!("syntax error: malformed set!: {}", token))?,
                                    env)?;
                                let mut env = env;
                                loop {
                                    let mut vals = env.variables.borrow_mut();
                                    if let Some(_) = vals.get(&id.to_string()) {
                                        vals.insert(id.to_string(), exp).unwrap();
                                        break Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Undefined})));
                                    }
                                    if let Some(e) = &env.parent {
                                        env = e;
                                    } else {
                                        break Err(anyhow!("symbal not defined: {}", id));
                                    }
                                }
                            } else {
                                Err(anyhow!(""))
                            }
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
                                .with_context(|| format!("error: proper list required for function application or macro use: {}", token))?
                                .elem()
                                .with_context(|| format!("syntax error: malformed if: {}", token))?;
                            let exp3 = cdr.next()
                                .unwrap()
                                .next()
                                .with_context(|| format!("error: proper list required for function application or macro use: {}", token))?;
                            let cond = eval_exp(exp1, env)?;
                            match exp3 {
                                Token::Pair{car: exp3_car, cdr: exp3_cdr} => {
                                    if !exp3_cdr.is_empty() {
                                        Err(anyhow!("syntax error: malformed if: {}", token))
                                    } else if !cond.borrow().is_falsy() {
                                        eval_exp(exp2, env)
                                    } else {
                                        eval_exp(exp3_car, env)
                                    }
                                }
                                Token::Empty => {
                                    if !cond.borrow().is_falsy() {
                                        eval_exp(exp2, env)
                                    } else {
                                        Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Undefined})))
                                    }
                                }
                                _ => {
                                    Err(anyhow!("error: proper list required for function application or macro use: {}", token))
                                }
                            }
                        }
                        "cond" => {
                            ensure_proper_list(cdr)?;
                            if cdr.is_empty() {
                                bail!("syntax error: at least one clause is required for cond: {}", token);
                            }

                            let mut res = Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Undefined})));
                            for clause in &**cdr {
                                println!("debug: clause = {}", clause);
                                if let Token::Pair{car: test, cdr: exps} = clause {
                                    if exps.is_empty() || !exps.is_list() {
                                        bail!("syntax error: bad clause in cond: {}", token);
                                    } else {
                                        match &**test {
                                            Token::Id(s) if s == "else" => {
                                                for exp in &**exps {
                                                    res = Ok(eval_exp(exp, env)?);
                                                }
                                            }
                                            _ => {
                                                if !eval_exp(test, env)?.borrow().is_falsy() {
                                                    for exp in &**exps {
                                                        res = Ok(eval_exp(exp, env)?);
                                                    }
                                                } else {
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    bail!("syntax error: bad clause in cond: {}", token);
                                }
                            }
                            res
                        }
                        "and" => {
                            ensure_proper_list(cdr)?;
                            let mut res = Rc::new(RefCell::new(Object{kind: ObjectKind::Boolean(true)}));
                            for test in &**cdr {
                                res = eval_exp(test, env)?;
                                if res.borrow().is_falsy() {
                                    break;
                                }
                            }
                            Ok(res)
                        }
                        "or" => {
                            ensure_proper_list(cdr)?;
                            let mut res = Rc::new(RefCell::new(Object{kind: ObjectKind::Boolean(false)}));
                            for test in &**cdr {
                                res = eval_exp(test, env)?;
                                if !res.borrow().is_falsy() {
                                    break;
                                }
                            }
                            Ok(res)
                        }
                        "begin" => {
                            ensure_proper_list(cdr)?;
                            let mut res = Rc::new(RefCell::new(Object{kind: ObjectKind::Number(NumberKind::Int(0))}));
                            for exp in &**cdr {
                                res = eval_exp(exp, env)?;
                            }
                            Ok(res)
                        }
                        "do" => {
                            // (do (val_init_steps) (test exps) body)
                            ensure_proper_list(cdr)?;
                            let do_env = Rc::new(Environment::new(env));

                            let val_init_steps = cdr.elem().with_context(|| format!("syntax error: malformed do: {}", token))?;
                            if !val_init_steps.is_list() {
                                bail!("syntax error: malformed do: {}", token);
                            }
                            let mut vals: Vec<&str> = Vec::new();
                            let mut steps: Vec<&Token> = Vec::new();
                            for val_init_step in val_init_steps {
                                let val = val_init_step.nth(0).with_context(|| format!("syntax error: malformed do: {}", token))?;
                                let init = val_init_step.nth(1).with_context(|| format!("syntax error: malformed do: {}", token))?;
                                let step = val_init_step.nth(2).with_context(|| format!("syntax error: malformed do: {}", token))?;
                                if let Token::Id(id) = val {
                                    vals.push(id.as_ref());
                                    do_env.variables.borrow_mut().insert(id.clone(), eval_exp(init, env)?);
                                } else {
                                    bail!("syntax error: malformed do: {}", token);
                                }
                                steps.push(step);
                                if let Some(_) = val_init_step.nth(3) {
                                    bail!("syntax error: malformed do: {}", token);
                                }
                            }

                            let test_exp = cdr
                                .next()
                                .with_context(|| format!("syntax error: malformed do: {}", token))?
                                .elem()
                                .with_context(|| format!("syntax error: malformed do: {}", token))?;
                            if !test_exp.is_list() {
                                bail!("syntax error: malformed do: {}", token);
                            }
                            let test = test_exp.elem().with_context(|| format!("syntax error: malformed do: {}", token))?;

                            let cmds = cdr
                                .next()
                                .unwrap()
                                .next()
                                .with_context(|| format!("syntax error: malformed do: {}", token))?;
                            
                            while !eval_exp(test, &do_env)?.borrow().is_falsy() {
                                eval_body(cmds, &do_env)?;
                            }

                            let mut res = Rc::new(RefCell::new(Object{kind: ObjectKind::Boolean(true)}));
                            for exp in test_exp.next() {
                                res = eval_exp(exp, &do_env)?;
                            }
                            Ok(res)
                        }
                        _ => Err(anyhow!("unbound variable: {}", id))
                    }
                }
            }, 
            Token::Pair{..} => {
                eval_app(token, car, cdr, env)
            }
            _ => {
                Err(anyhow!("invalid application: {}", token))
            }
        }
    }
}

fn eval_quote(token: &Token) -> Result<Rc<RefCell<Object>>> {
    // "token" must be elements of Token::Symbol
    match token {
        &Token::Int(i) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Number(NumberKind::Int(i))}))),
        &Token::Float(f) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Number(NumberKind::Float(f))}))),
        &Token::Boolean(b) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Boolean(b)}))),
        Token::String(s) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::String(s.clone())}))),
        &Token::Empty => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Empty}))),
        Token::Symbol(_) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Symbol(format!("{}", token))}))),
        Token::Id(id) => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Symbol(format!("{}", id))}))),
        Token::Pair{car, cdr} => Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Pair{
            car: Ref::Rc(eval_quote(&**car)?), 
            cdr: Ref::Rc(eval_quote(&**cdr)?)
        }}))),
    }
}

fn eval_app(token: &Token, proc: &Token, args: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    // argument "token" is for error messages
    ensure_proper_list(args)?;
    let proc = eval_exp(proc, env)?;
    let mut args: VecDeque<Result<Rc<RefCell<Object>>>> = args
        .into_iter()
        .map(|t| -> Result<Rc<RefCell<Object>>> {eval_exp(t, env)})
        .collect();
    
    let proc = proc.borrow();
    if let ObjectKind::Procedure(proc) = &proc.kind {
        let new_env = Environment::new(&proc.env);
        if !proc.args.is_variadic && proc.args.required != args.len() {
            bail!("wrong number of arguments (required {}, got {})", proc.args.required, args.len());
        }
        if proc.args.required < args.len() {
            bail!("wrong number of arguments (required {}, got {})", proc.args.required, args.len());
        }

        for i in 0..proc.args.required {
            new_env.variables.borrow_mut().insert(proc.args.ids.get(i).unwrap().clone(), args.pop_front().unwrap()?);
        }
        if proc.args.is_variadic {
            let mut variadic = Rc::new(RefCell::new(Object{kind: ObjectKind::Empty}));
            for _ in 0..args.len() {
                variadic = Rc::new(RefCell::new(Object{kind: ObjectKind::Pair{
                    car: Ref::Rc(args.pop_back().unwrap()?),
                    cdr: Ref::Rc(variadic)
                }}));
            }
            new_env.variables.borrow_mut().insert(proc.args.ids.get(proc.args.required).unwrap().clone(), variadic);
        }
        
        eval_body(&proc.body, &Rc::new(new_env))
    } else {
        Err(anyhow!("invalid application: {}", token))
    }
}

fn eval_lambda(token: &Token, mut arg: &Token, body: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut args = Vec::new();
    while let Some(id) = arg.elem() {
        match id {
            Token::Id(id) => args.push(id.clone()),
            _ => bail!("syntax error: argment must be identifier: {}", token),
        }
        arg = arg.cdr().unwrap();
    }
    let required = args.len();
    match arg {
        Token::Id(id) => {
            args.push(id.clone());
            Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Procedure(Procedure{
                env: Rc::clone(env), 
                args: Args{ids: args, is_variadic: true, required}, 
                body: body.clone()
            })})))
        }
        Token::Empty => {
            Ok(Rc::new(RefCell::new(Object{kind: ObjectKind::Procedure(Procedure{
                env: Rc::clone(env), 
                args: Args{ids: args, is_variadic: false, required}, 
                body: body.clone()
            })})))
        }
        _ => Err(anyhow!("syntax error: argment must be identifier: {}", token))
    }
}

fn eval_body(mut token: &Token, env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    ensure_proper_list(token)?;
    // Define
    let mut def = token.elem().context("syntax error: at least one expression is necessary")?;
    loop {
        if let Token::Pair{car, cdr: _} = def {
            match &**car {
                Token::Id(s) if s == "define" => {
                    eval_define(def, env)?;
                    token = token.next().unwrap();
                    def = token.elem().context("syntax error: at least one expression is necessary")?;
                }
                _ => break
            }
        } else {
            break;
        }
    };
    // Expression
    loop {
        let exp = token.elem().unwrap();
        if let Token::Empty = token.next().unwrap() {
            break eval_exp(exp, env)
        }
        eval_exp(exp, env)?;
        token = token.next().unwrap();
    }
}

fn ensure_proper_list(token: &Token) -> Result<()> {
    if !token.is_list() {
        Err(anyhow!("proper list required for function application or macro use: {}", token))
    } else {
        Ok(())
    }
}
