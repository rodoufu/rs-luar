use std::cell::RefCell;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Object<'a> {
	x: u32,
	y: u32,
	children: RefCell<Vec<Box<&'a Object<'a>>>>,
	parent: RefCell<Option<Box<&'a Object<'a>>>>,
}

impl<'a> Object<'a> {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y,
			children: RefCell::new(Vec::new()),
			parent: RefCell::new(None),
		}
	}

	pub fn add_child(&'a self, child: &'a Object<'a>) {
		if child.parent.clone().into_inner().is_some() {
			let child_parent = child.parent.clone().into_inner().unwrap();
			child_parent.remove_child(child);
		}
		child.parent.replace(Some(Box::new(self)));

		let mut children = self.children.clone().into_inner();
		children.push(Box::new(child));
		self.children.replace(children);
	}

	pub fn remove_child(&self, child: &'a Object<'a>) {
		let mut children = self.children.clone().into_inner();
		let children_len = children.len();
		children.retain(|x| **x != child);
		let removed = children.len() != children_len;
		if removed {
			self.children.replace(children);
			child.parent.replace(None);
		}
	}

	pub fn world_translation(&self) -> (u32, u32) {
		match self.parent.borrow().as_ref() {
			None => (self.x, self.y),
			Some(parent) => (parent.x + self.x, parent.y + self.y),
		}
	}

	fn number_of_children(&self) -> usize {
		self.children.clone().into_inner().len()
	}
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

	#[test]
	fn world_translation_with_parent() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);

		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());

		parent.add_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((5, 5), child.world_translation());
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
	}

	#[test]
	fn world_translation_with_parent_then_remove() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);

		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());

		parent.add_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((5, 5), child.world_translation());
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());

		parent.remove_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
	}

	#[test]
	fn world_translation_with_parent_then_remove_wrong_child() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);
		let child2 = Object::new(10, 11);

		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!((10, 11), child2.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());

		parent.add_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((5, 5), child.world_translation());
		assert_eq!((10, 11), child2.world_translation());
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());

		parent.remove_child(&child2);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((5, 5), child.world_translation());
		assert_eq!((10, 11), child2.world_translation());
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());

		parent.remove_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!((10, 11), child2.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
	}

	#[test]
	fn world_translation_with_another_parent() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);
		let parent2 = Object::new(10, 11);

		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!((10, 11), parent2.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, parent2.number_of_children());

		parent.add_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((5, 5), child.world_translation());
		assert_eq!((10, 11), parent2.world_translation());
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, parent2.number_of_children());

		parent2.add_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((14, 14), child.world_translation());
		assert_eq!((10, 11), parent2.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(1, parent2.number_of_children());

		parent2.remove_child(&child);
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 3), child.world_translation());
		assert_eq!((10, 11), parent2.world_translation());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, parent2.number_of_children());
	}

	// TODO Add grand_child test
}
