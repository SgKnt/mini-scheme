use std::rc::Rc;
use std::cell::RefCell;

use super::object::{Object};

struct Variable {
    pub id: String,
    pub value: RefCell<Rc<Object>>
}
