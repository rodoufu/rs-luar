use std::borrow::Borrow;

trait ObjectInterface {
	fn add_child(&self, child: &dyn ObjectInterface);
	fn remove_child(&self, child: &dyn ObjectInterface);
	fn world_translation(&self) -> (u32, u32);
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Object {
	x: u32,
	y: u32,
	children: Vec<Box<Object>>,
	parent: Option<Box<Object>>,
}

impl Object {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y,
			children: Vec::new(),
			parent: None,
		}
	}

	pub fn world_translation(&self) -> (u32, u32) {
		match &self.parent {
			None => (self.x, self.y),
			Some(parent) => (parent.x + self.x, parent.y + self.y),
		}
	}

	pub fn add_child(&mut self, child: Object) {
//		child.parent = Some(Box::new(*self));
		self.children.push(Box::new(child));
	}

	pub fn remove_child(&self, child: Object) {}
}

#[cfg(test)]
mod tests {
	use crate::object::Object;

	#[test]
	fn world_translation_no_parent() {
		assert_eq!((0, 0), Object::new(0, 0).world_translation());
		assert_eq!((0, 1), Object::new(0, 1).world_translation());
		assert_eq!((3, 1), Object::new(3, 1).world_translation());
	}
}
