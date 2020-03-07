use std::borrow::Borrow;
use std::cell::RefCell;
use std::iter::IntoIterator;
use std::rc::Rc;

pub struct Shared<T> {
	data: Rc<RefCell<T>>,
}

#[cfg(not(feature = "iter"))]
//#[cfg(feature = "iter")]
impl<'a, K, T: IntoIterator<Item=K>> IntoIterator for &'a Shared<T>
	where T: IntoIterator<Item=K> + Clone {
	type Item = K;
	type IntoIter = T::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		let self_ref: &RefCell<T> = self.data.borrow();
		self_ref.clone().into_inner().into_iter()
	}
}

#[cfg(feature = "iter")]
impl<'a, K, T: Iterator<Item=K>> IntoIterator for &'a Shared<T>
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
	use crate::shared::Shared;
	use std::iter::IntoIterator;
	use std::rc::Rc;
	use std::cell::RefCell;
	use std::borrow::Borrow;

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

		let mut x = 1;
		for it in &my_shared {
			assert_eq!(it, x);
			x += 1;
		}
		assert_eq!(x, 4);

		let mut my_it = my_shared.borrow().into_iter();
		assert_eq!(my_it.next().unwrap(), 1);
		assert_eq!(my_it.next().unwrap(), 2);
		assert_eq!(my_it.next().unwrap(), 3);
		assert!(my_it.next().is_none());
	}
}
