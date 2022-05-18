use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};

use crate::data::{*, object::*};
use crate::token::*;
use crate::parse::Parser;

pub fn eval(token: &Token, env: Environment) -> Result<Object> {
    // Exp, Define, (load String)
    match token {
        Token::Pair{car, cdr} => match &**car {
            Token::Id(id) if id == "define" => {
                eval_define(token, env)
            }
            Token::Id(id) if id == "load" => {
                eval_load(token, &**cdr, env)
            }
            _ => eval_exp(token, env)
        }
        _ => eval_exp(token, env)
    }
}

pub fn eval_define(token: &Token, env: Environment) -> Result<Object> {
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
                .map(|t| eval_exp(t, env.clone()))
                .unwrap_or_else(|| Ok(Object::new_undefined()))?;
            env.insert(id.clone(), obj);
            Ok(Object::new_symbol(id.clone(), false))
        }
        Token::Pair{car: id, cdr: args} => {
            if let Token::Id(id) = &**id {
                let body = token.next().unwrap();
                let obj = eval_lambda(args, body, env.clone())?;
                env.insert(id.clone(), obj);
                Ok(Object::new_symbol(id.clone(), false))
            } else {
                Err(anyhow!("syntax error: {}", def_token))
            }
        }
        _ => Err(anyhow!("syntax error: {}", def_token))
    }
}

pub fn eval_load(token: &Token, path: &Token, env: Environment) -> Result<Object> {
    // argument "token" is used for error messages
    let (mut file, path_display) = match path {
        Token::Pair{car, cdr} => {
            if let Token::Empty = &**cdr {
                if let Token::String(path) = &**car {
                    let path = Path::new(&*path);
                    match File::open(&path) {
                        Ok(file) => (file, path.display()),
                        Err(reason) => bail!("could't open {}: {}", path.display(), reason)
                    }
                } else {
                    bail!("string required, but got {}", car);
                }
            } else {
                bail!("syntax error: malformed load: {}", token)
            }
        }
        _ => bail!("proper list required for function application or macro use: {}", token)
    };

    let mut buf = String::new();
    let parser;
    let tokens;
    if let Err(reason) = file.read_to_string(&mut buf) {
        bail!("could't read {}: {}", path_display, reason)
    }
    parser = Parser::new(buf);
    tokens = parser.build_tokens();
    for token in tokens {
        match eval(&token?, env.clone()) {
            Ok(res) => println!("{}", res),
            Err(reason) => println!("{}", reason),
        }
    }
    Ok(Object::new_boolean(true, true))
}

fn eval_exp(token: &Token, env: Environment) -> Result<Object> {
    match token {
        &Token::Int(i) => Ok(Object::new_int(i, false)),
        &Token::Float(f) => Ok(Object::new_float(f, false)),
        &Token::Boolean(b) => Ok(Object::new_boolean(b, false)),
        Token::String(s) => Ok(Object::new_string(s.clone(), false)),
        Token::Empty => Ok(Object::new_empty()),
        Token::Symbol(s) => eval_quote(&*s),
        Token::Id(id) => if let Some(var) = env.lookup(id) {
            Ok(var)
        } else {
            Err(anyhow!("unbound variable: {}", id))
        },
        Token::Pair{car, cdr} => match &**car {
            Token::Id(id) => {
                if env.lookup(id).is_some() {
                    eval_app(token, car, cdr, env)
                } else {
                    match id.as_str() {
                        "lambda" => {
                            ensure_proper_list(cdr)?;
                            let arg = cdr.elem().with_context(|| format!("syntax error: malformed lambda: {}", token))?;
                            let body = cdr.next().unwrap();
                            eval_lambda(arg, body, env)
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
                                    env.clone()
                                )?;
                                if let Some(env) = env.contains_at(id) {
                                    env.insert(id.clone(), exp);
                                    Ok(Object::new_undefined())
                                } else {
                                    Err(anyhow!("symbal not defined: {}", id))
                                }
                            } else {
                                Err(anyhow!(""))
                            }
                        }
                        "let" => {
                            ensure_proper_list(cdr)?;
                            match cdr.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                Token::Id(id) => {
                                    let new_env = Environment::new(env.clone());
                                    let name = id.clone();
                                    let bindings = cdr.nth(1).with_context(|| format!("syntax error: malformed let: {}", token))?;
                                    let body = cdr.next().unwrap().next().with_context(|| format!("syntax error: malformed let: {}", token))?;
                                    ensure_proper_list(bindings)?;

                                    let mut args = Vec::new();
                                    let mut inits = VecDeque::new();
                                    for binding in bindings {
                                        match binding.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                            Token::Id(id) => args.push(id.clone()),
                                            t => bail!("syntax error: identifier required, but got {}", t),
                                        };
                                        inits.push_back(eval_exp(binding.nth(1).with_context(|| format!("syntax error: malformed let: {}", token))?, env.clone())?);
                                        if binding.nth(2).is_some() {
                                            bail!("syntax error: malformed let: {}", token)
                                        }
                                    }
                                    let proc = Object::new_procedure(env.clone(), args.clone(), false, args.len(), body.clone());
                                    new_env.insert(name, proc);
                                    for arg in args {
                                        new_env.insert(arg, inits.pop_front().unwrap());
                                    }
                                    eval_body(body, new_env)
                                }
                                Token::Pair{..} => {
                                    let bindings = cdr.nth(0).unwrap();
                                    let body = cdr.next().with_context(|| format!("syntax error: malformed let: {}", token))?;
                                    ensure_proper_list(bindings)?;
                                    let new_env = Environment::new(env.clone());
                                    for binding in bindings {
                                        let id = match binding.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                            Token::Id(id) => id.clone(),
                                            t => bail!("syntax error: identifier required, but got {}", t),
                                        };
                                        new_env.insert(id, eval_exp(binding.nth(1).with_context(|| format!("syntax error: malformed let: {}", token))?, env.clone())?);
                                    }
                                    eval_body(body, new_env)
                                }
                                _ => Err(anyhow!("syntax error: malformed let: {}", token))
                            }
                        }
                        "let*" => {
                            ensure_proper_list(cdr)?;
                            match cdr.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                Token::Pair{..} => {
                                    let bindings = cdr.nth(0).unwrap();
                                    let body = cdr.next().with_context(|| format!("syntax error: malformed let: {}", token))?;
                                    ensure_proper_list(bindings)?;
                                    let mut env = env;
                                    for binding in bindings {
                                        let id = match binding.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                            Token::Id(id) => id.clone(),
                                            t => bail!("syntax error: identifier required, but got {}", t),
                                        };
                                        let init = eval_exp(binding.nth(1).with_context(|| format!("syntax error: malformed let: {}", token))?, env.clone())?;
                                        let new_env = Environment::new(env);
                                        new_env.insert(id, init);
                                        env = new_env;
                                    }
                                    eval_body(body, env)
                                }
                                _ => Err(anyhow!("syntax error: malformed let: {}", token))
                            }
                        }
                        "letrec" => {
                            ensure_proper_list(cdr)?;
                            match cdr.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                Token::Pair{..} => {
                                    let bindings = cdr.nth(0).unwrap();
                                    let body = cdr.next().with_context(|| format!("syntax error: malformed let: {}", token))?;
                                    ensure_proper_list(bindings)?;
                                    let new_env = Environment::new(env);
                                    for binding in bindings {
                                        let id = match binding.nth(0).with_context(|| format!("syntax error: malformed let: {}", token))? {
                                            Token::Id(id) => id.clone(),
                                            t => bail!("syntax error: identifier required, but got {}", t),
                                        };
                                        let init = eval_exp(binding.nth(1).with_context(|| format!("syntax error: malformed let: {}", token))?, new_env.clone())?;
                                        new_env.insert(id, init);
                                    }
                                    eval_body(body, new_env)
                                }
                                _ => Err(anyhow!("syntax error: malformed let: {}", token))
                            }
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
                            let cond = eval_exp(exp1, env.clone())?;
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
                                        Ok(Object::new_undefined())
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

                            let mut res = Ok(Object::new_undefined());
                            for clause in &**cdr {
                                println!("debug: clause = {}", clause);
                                if let Token::Pair{car: test, cdr: exps} = clause {
                                    if exps.is_empty() || !exps.is_list() {
                                        bail!("syntax error: bad clause in cond: {}", token);
                                    } else {
                                        match &**test {
                                            Token::Id(s) if s == "else" => {
                                                for exp in &**exps {
                                                    res = Ok(eval_exp(exp, env.clone())?);
                                                }
                                            }
                                            _ => {
                                                if !eval_exp(test, env.clone())?.is_falsy() {
                                                    for exp in &**exps {
                                                        res = Ok(eval_exp(exp, env.clone())?);
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
                            let mut res = Object::new_boolean(true, true);
                            for test in &**cdr {
                                res = eval_exp(test, env.clone())?;
                                if res.is_falsy() {
                                    break;
                                }
                            }
                            Ok(res)
                        }
                        "or" => {
                            ensure_proper_list(cdr)?;
                            let mut res = Object::new_boolean(false, true);
                            for test in &**cdr {
                                res = eval_exp(test, env.clone())?;
                                if !res.is_falsy() {
                                    break;
                                }
                            }
                            Ok(res)
                        }
                        "begin" => {
                            ensure_proper_list(cdr)?;
                            let mut res = Object::new_undefined();
                            for exp in &**cdr {
                                res = eval_exp(exp, env.clone())?;
                            }
                            Ok(res)
                        }
                        "do" => {
                            // (do (val_init_steps) (test exps) body)
                            ensure_proper_list(cdr)?;
                            let do_env = Environment::new(env.clone());

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
                                    do_env.insert(id.clone(), eval_exp(init, env.clone())?);
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
                            
                            while !eval_exp(test, do_env.clone())?.is_falsy() {
                                eval_body(cmds, do_env.clone())?;
                            }

                            let mut res = Object::new_undefined();
                            for exp in test_exp.next() {
                                res = eval_exp(exp, do_env.clone())?;
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

fn eval_quote(token: &Token) -> Result<Object> {
    // "token" must be elements of Token::Symbol
    match token {
        &Token::Int(i) => Ok(Object::new_int(i, false)),
        &Token::Float(f) => Ok(Object::new_float(f, false)),
        &Token::Boolean(b) => Ok(Object::new_boolean(b, false)),
        Token::String(s) => Ok(Object::new_string(s.clone(), false)),
        &Token::Empty => Ok(Object::new_empty()),
        Token::Symbol(_) => Ok(Object::new_symbol(format!("{}", token), false)),
        Token::Id(id) => Ok(Object::new_symbol(format!("{}", id), false)),
        Token::Pair{car, cdr} => Ok(Object::new_pair(
            eval_quote(&**car)?, 
            eval_quote(&**cdr)?, 
            false
        )),
    }
}

fn eval_app(token: &Token, proc: &Token, args: &Token, env: Environment) -> Result<Object> {
    // argument "token" is for error messages
    ensure_proper_list(args)?;
    let proc = eval_exp(proc, env.clone())?;
    let mut args: VecDeque<Object> = args
        .into_iter()
        .map(|t| -> Result<Object> {eval_exp(t, env.clone())})
        .collect::<Result<VecDeque<Object>>>()?;
    
    match proc.kind() {
        Kind::Procedure(proc) => {
            match proc {
                Procedure::Proc(proc) => {
                    let new_env = Environment::new(proc.env());
                    if !proc.is_variadic && proc.require != args.len() {
                        bail!("wrong number of arguments (required {}, got {})", proc.require, args.len());  
                    } 
                    if proc.require > args.len() {
                        bail!("wrong number of arguments (required {}, got {})", proc.require, args.len());
                    }

                    for i in 0..proc.require {
                        new_env.insert(proc.args.get(i).unwrap().clone(), args.pop_front().unwrap());
                    }
                    if proc.is_variadic {
                        let mut variadic = Object::new_empty();
                        for _ in 0..args.len() {
                            variadic = Object::new_pair(
                                args.pop_back().unwrap(),
                                variadic,
                                true,
                            );
                        }
                        new_env.insert(proc.args.get(proc.require).unwrap().clone(), variadic);
                    }

                    eval_body(&proc.body, new_env)
                }
                Procedure::Subr(subr) => {
                    if !subr.is_variadic && subr.require != args.len() {
                        bail!("wrong number of arguments (required {}, got {})", subr.require, args.len());
                    }
                    if subr.require > args.len() {
                        bail!("wrong number of arguments (required {}, got {})", subr.require, args.len());
                    }

                    (subr.fun)(args)
                }
            }
        }
        _ => Err(anyhow!("invalid application: {}", token))
    }
}

fn eval_lambda(mut arg: &Token, body: &Token, env: Environment) -> Result<Object> {
    let mut args = Vec::new();
    while let Some(id) = arg.elem() {
        match id {
            Token::Id(id) => args.push(id.clone()),
            _ => bail!("syntax error: identifier required, but got {}", id),
        }
        arg = arg.cdr().unwrap();
    }
    let require = args.len();
    match arg {
        Token::Id(id) => {
            args.push(id.clone());
            Ok(Object::new_procedure(env, args, true, require, body.clone()))
        }
        Token::Empty => {
            Ok(Object::new_procedure(env, args, false, require, body.clone()))
        }
        _ => Err(anyhow!("syntax error: identifier required, but got {}", arg))
    }
}

fn eval_body(mut token: &Token, env: Environment) -> Result<Object> {
    ensure_proper_list(token)?;
    // Define
    let mut def = token.elem().context("syntax error: at least one expression is necessary")?;
    loop {
        if let Token::Pair{car, cdr: _} = def {
            match &**car {
                Token::Id(s) if s == "define" => {
                    eval_define(def, env.clone())?;
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
        eval_exp(exp, env.clone())?;
        token = token.next().unwrap();
    }
}

pub fn ensure_proper_list(token: &Token) -> Result<()> {
    if !token.is_list() {
        Err(anyhow!("proper list required for function application or macro use: {}", token))
    } else {
        Ok(())
    }
}
