use std::rc::Rc;
use std::cell::RefCell;

pub struct Shared<T> {
	data: Rc<RefCell<T>>,
}

// TODO Implement IntoIterator for &'a Shared<T>'
