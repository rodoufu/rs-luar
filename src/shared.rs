use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter::IntoIterator;
use std::rc::Rc;

pub struct Shared<T> where T: Clone {
	data: Rc<RefCell<T>>,
}

impl<'a, K, T: IntoIterator<Item=K>> IntoIterator for &'a Shared<T>
	where T: Iterator<Item=K> + Clone {
	type Item = K;
	type IntoIter = T;

	fn into_iter(self) -> Self::IntoIter {
		let self_ref: &RefCell<T> = self.data.borrow();
		self_ref.clone().into_inner()
	}
}


#[cfg(test)]
mod tests {
	use crate::{
		object::Object,
		shared::Shared,
	};
	use std::iter::IntoIterator;
	use std::rc::Rc;
	use std::cell::RefCell;

	#[derive(Clone, Debug, Default, PartialEq, Eq)]
	struct Test {
		value: u32
	}

	impl Iterator for Test {
		type Item = u32;

		fn next(&mut self) -> Option<Self::Item> {
			if self.value == 3 {
				None
			} else {
				self.value += 1;
				Some(self.value)
			}
		}
	}

	#[test]
	fn world_translation_no_parent() {
		let test: Test = Default::default();
		let my_shared: Shared<Test> = Shared {
			data: Rc::new(RefCell::new(test))
		};

		{
			let mut x = 1;
			for it in &my_shared {
				assert_eq!(it, x);
				x += 1;
			}
			assert_eq!(x, 4);
		}
	}
}
