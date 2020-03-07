use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Shared<T> where T: Clone {
	data: Rc<RefCell<T>>,
}

impl<'a, K, T: IntoIterator<Item=K>> IntoIterator for &'a Shared<T> where T: Iterator<Item=K> + Clone {
	type Item = K;
	type IntoIter = T;

	fn into_iter(self) -> Self::IntoIter {
		let self_ref: &RefCell<T> = self.data.borrow();
		self_ref.clone().into_inner()
	}
}
