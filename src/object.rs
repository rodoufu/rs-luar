use std::borrow::Borrow;
use std::cell::RefCell;

trait ObjectInterface {
	fn add_child(&self, child: &dyn ObjectInterface);
	fn remove_child(&self, child: &dyn ObjectInterface);
	fn world_translation(&self) -> (u32, u32);
}

//Hash,
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Object {
	x: u32,
	y: u32,
	children: RefCell<Vec<Box<Object>>>,
	parent: RefCell<Option<Box<Object>>>,
}

impl Object {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y,
			children: RefCell::new(Vec::new()),
			parent: RefCell::new(None),
		}
	}
}

impl ObjectInterface for Object {
	fn add_child(&self, child: &dyn ObjectInterface) {
//		child.parent = Some(Box::new(*self));
//		self.children.push(Box::new(child));
	}

	fn remove_child(&self, child: &dyn ObjectInterface) {}

	fn world_translation(&self) -> (u32, u32) {
		match self.parent.borrow().as_ref() {
			None => (self.x, self.y),
			Some(parent) => (parent.x + self.x, parent.y + self.y),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::object::{
		Object,
		ObjectInterface,
	};

	#[test]
	fn world_translation_no_parent() {
		assert_eq!((0, 0), Object::new(0, 0).world_translation());
		assert_eq!((0, 1), Object::new(0, 1).world_translation());
		assert_eq!((3, 1), Object::new(3, 1).world_translation());
	}
}
