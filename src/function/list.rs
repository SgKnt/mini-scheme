use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use anyhow::{Result, anyhow, bail};

use crate::env::Environment;
use crate::object::*;

pub fn is_null(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Empty => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn is_pair(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    match &*args.pop_front().unwrap()?.borrow() {
        Object::Pair{..} => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn is_list(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let arg = args.pop_front().unwrap()?;

    is_list_inner(&arg)
}

fn is_list_inner(obj: &Rc<RefCell<Object>>) -> Result<Rc<RefCell<Object>>> {
    match &*obj.borrow() {
        Object::Pair{car:_, cdr} => is_list_inner(cdr),
        Object::Empty => Ok(Rc::new(RefCell::new(Object::Boolean(true)))),
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false)))),
    }
}

pub fn car(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let arg = args.pop_front().unwrap()?;
    let arg = arg.borrow();
    match &*arg {
        Object::Pair{car, ..} => Ok(Rc::clone(car)),
        obj => Err(anyhow!("pair required, but got {}", obj))
    }
}

pub fn cdr(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let arg = args.pop_front().unwrap()?;
    let arg = arg.borrow();
    match &*arg {
        Object::Pair{car:_, cdr} => Ok(Rc::clone(cdr)),
        obj => Err(anyhow!("pair required, but got {}", obj))
    }
}

pub fn cons(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let car = args.pop_front().unwrap()?;
    let cdr = args.pop_front().unwrap()?;
    Ok(Rc::new(RefCell::new(Object::Pair{car, cdr})))
}

pub fn list(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut res = Rc::new(RefCell::new(Object::Empty));
    while let Some(arg) = args.pop_back() {
        res = Rc::new(RefCell::new(Object::Pair{
            car: arg?,
            cdr: res
        }));
    }
    Ok(res)
}

pub fn length(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let obj = args.pop_front().unwrap()?;
    match length_inner(&obj, 0) {
        Ok(len) => Ok(Rc::new(RefCell::new(Object::Number(NumberKind::Int(len))))),
        Err(_) => Err(anyhow!("proper list required, but got {}", obj.borrow()))
    }
}

fn length_inner(obj: &Rc<RefCell<Object>>, len: i64) -> core::result::Result<i64, ()> {
    match &*obj.borrow() {
        Object::Pair{car:_, cdr} => length_inner(cdr, len + 1),
        Object::Empty => Ok(len),
        _ => Err(())
    }
}

pub fn memq(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let obj = args.pop_front().unwrap()?;
    let list = args.pop_front().unwrap()?;
    memq_inner(&obj, &list)
}

fn memq_inner(obj: &Rc<RefCell<Object>>, list: &Rc<RefCell<Object>>) -> Result<Rc<RefCell<Object>>> {
    match &*list.borrow() {
        Object::Pair{car, cdr} => {
            if obj.borrow().eq_scm(&*car.borrow()) {
                Ok(Rc::clone(list))
            } else {
                memq_inner(obj, cdr)
            }
        }
        _ => Ok(Rc::new(RefCell::new(Object::Boolean(false))))
    }
}

pub fn last(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let pair = args.pop_front().unwrap()?;
    last_inner(&pair)
}

fn last_inner(pair: &Rc<RefCell<Object>>) -> Result<Rc<RefCell<Object>>> {
    match &*pair.borrow() {
        Object::Pair{car, cdr} => match &*cdr.borrow() {
            Object::Pair{..} => last_inner(cdr),
            _ => Ok(Rc::clone(car)),
        }
        obj => Err(anyhow!("pair required, but got {}", obj))
    }
}

pub fn append(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let mut res = args.pop_back().unwrap()?;
    while let Some(list) = args.pop_back() {
        res = append_inner(&list?, res)?
    }
    Ok(res)
}

fn append_inner(list1: &Rc<RefCell<Object>>, list2: Rc<RefCell<Object>>) -> Result<Rc<RefCell<Object>>> {
    match &*list1.borrow() {
        Object::Pair{car, cdr} => {
            Ok(Rc::new(RefCell::new(Object::Pair{
                car: Rc::clone(car),
                cdr: append_inner(cdr, list2)?
            })))
        }
        Object::Empty => Ok(list2),
        _ => Err(anyhow!("proper list required, but got {}", list1.borrow()))
    }
}

pub fn set_car(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let pair = args.pop_front().unwrap()?;
    let obj = args.pop_front().unwrap()?;
    match &mut*pair.borrow_mut() {
        Object::Pair{car, ..} => {
            *car = obj;
        }
        obj => bail!("pair required, but got {}", obj)
    };
    Ok(Rc::new(RefCell::new(Object::Undefined)))
}

pub fn set_cdr(mut args: VecDeque<Result<Rc<RefCell<Object>>>>, _env: &Rc<Environment>) -> Result<Rc<RefCell<Object>>> {
    let pair = args.pop_front().unwrap()?;
    let obj = args.pop_front().unwrap()?;
    match &mut*pair.borrow_mut() {
        Object::Pair{car:_, cdr} => {
            *cdr = obj;
        }
        obj => bail!("pair required, but got {}", obj)
    };
    Ok(Rc::new(RefCell::new(Object::Undefined)))
}
