use std::cell::RefCell;

/// Object with 2D translation information.
///
/// If the `Object` contains itself it would be impossible the determinate its size, this makes
/// necessary to use a pointer to `Object`, here it is used a `Box` of a reference to `Object`,
/// which is going to make the children and parent to be heap allocated.
#[derive(PartialEq, Eq)]
pub struct Object<'a> {
	x: u32,
	y: u32,
	children: RefCell<Vec<Box<&'a Object<'a>>>>,
	parent: RefCell<Option<Box<&'a Object<'a>>>>,
}

impl<'a> Object<'a> {
	/// Creates a new object for the translation.
	/// Encapsulates the internal logic of the object creation.
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y,
			children: RefCell::new(Vec::new()),
			parent: RefCell::new(None),
		}
	}

	/// Adds `child` as a new child of object.
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

	/// Removes the `child` from the object children.
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

	/// The translation of the object relative to the position of its container (parent) in case
	/// it exists.
	pub fn world_translation(&self) -> (u32, u32) {
		match self.parent.borrow().as_ref() {
			None => self.translation(),
			Some(parent) => {
				let (parent_x, parent_y) = parent.world_translation();
				(parent_x + self.x, parent_y + self.y)
			}
		}
	}

	/// The translation of the object itself.
	pub fn translation(&self) -> (u32, u32) {
		(self.x, self.y)
	}

	/// The number of children of the object.
	fn number_of_children(&self) -> usize {
		self.children.clone().into_inner().len()
	}

	/// Indicates if the object has a parent.
	fn has_parent(&self) -> bool {
		self.parent.clone().into_inner().is_some()
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
	fn add_same_child_twice() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);

		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert!(!parent.has_parent());
		assert!(!child.has_parent());

		parent.add_child(&child);
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert!(!parent.has_parent());
		assert!(child.has_parent());

		parent.add_child(&child);
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert!(!parent.has_parent());
		assert!(child.has_parent());
	}

	#[test]
	fn add_a_child_that_looks_the_same() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);
		let child2 = Object::new(4, 3);

		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(!child.has_parent());
		assert!(!child2.has_parent());

		parent.add_child(&child);
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(child.has_parent());
		assert!(!child2.has_parent());

		parent.add_child(&child2);
		assert_eq!(2, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(child.has_parent());
		assert!(child2.has_parent());
	}

	#[test]
	fn remove_a_child_that_looks_the_same() {
		let parent = Object::new(1, 2);
		let child = Object::new(4, 3);
		let child2 = Object::new(4, 3);

		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(!child.has_parent());
		assert!(!child2.has_parent());

		parent.add_child(&child);
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(child.has_parent());
		assert!(!child2.has_parent());

		parent.remove_child(&child2);
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(child.has_parent());
		assert!(!child2.has_parent());

		parent.remove_child(&child);
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert_eq!(0, child2.number_of_children());
		assert!(!parent.has_parent());
		assert!(!child.has_parent());
		assert!(!child2.has_parent());
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

	#[test]
	fn world_translation_with_grand_parent() {
		let grand_parent = Object::new(7, 11);
		let parent = Object::new(1, 2);
		let child = Object::new(4, 9);

		assert_eq!((7, 11), grand_parent.world_translation());
		assert_eq!((1, 2), parent.world_translation());
		assert_eq!((4, 9), child.world_translation());
		assert_eq!(0, grand_parent.number_of_children());
		assert_eq!(0, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert!(!grand_parent.has_parent());
		assert!(!parent.has_parent());
		assert!(!child.has_parent());

		grand_parent.add_child(&parent);
		parent.add_child(&child);
		assert_eq!((7, 11), grand_parent.world_translation());
		assert_eq!((8, 13), parent.world_translation());
		assert_eq!((12, 22), child.world_translation());
		assert_eq!(1, grand_parent.number_of_children());
		assert_eq!(1, parent.number_of_children());
		assert_eq!(0, child.number_of_children());
		assert!(!grand_parent.has_parent());
		assert!(parent.has_parent());
		assert!(child.has_parent());
	}
}
