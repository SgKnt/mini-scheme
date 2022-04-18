use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::val::{Variable};

pub struct Environment {
    pub variables: RefCell<HashMap<String, Variable>>,
    pub parent: RefCell<Rc<Environment>>
}
