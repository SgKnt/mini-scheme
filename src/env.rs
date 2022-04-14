use std::collections::HashMap;
use std::cell::RefCell;

use crate::val::{Variable};

pub struct Environment {
    pub variables: RefCell<HashMap<String, Variable>>
}
