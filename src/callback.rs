use std::collections::HashMap;

///
/// CallbackManager tracks callbacks and then call all then when requested.
///
#[derive(Default)]
pub struct CallbackManager<ParamType: Copy> {
	/// It is using a Box for the futures here cause the vector needs a sized type.
	callbacks: HashMap<usize, Box<dyn Fn(ParamType)>>,
	size: usize,
}

pub struct CallbackHandler<'a, ParamType: Copy> {
	manager: &'a mut CallbackManager<ParamType>,
	index: usize,
}

impl<'a, ParamType: Copy> Drop for CallbackHandler<'a, ParamType> {
	fn drop(&mut self) {
		self.manager.remove_callback_at(self.index)
	}
}

impl<ParamType: Copy> CallbackManager<ParamType> {
	pub fn add(&mut self, callback: Box<dyn Fn(ParamType)>) -> CallbackHandler<ParamType> {
		let temp_size = self.size;
		self.callbacks.insert(self.size, callback);
		self.size += 1;
		CallbackHandler {
			manager: self,
			index: temp_size,
		}
	}

	pub fn run_all(&self, param: ParamType) {
		for (_, callback) in &self.callbacks {
			callback(param);
		}
	}

	fn remove_callback_at(&mut self, index: usize) {
		self.callbacks.remove(&index);
	}
}

#[cfg(test)]
mod tests {
	use crate::callback::CallbackManager;

	#[test]
	fn no_param_callback() {
		let mut my_callback: CallbackManager<()> = Default::default();
		println!("Out of callback");
		let first_handler = my_callback.add(Box::new(|_| {
			println!("First callback");
		}));
		let second_handler = my_callback.add(Box::new(|_| {
			println!("Second callback");
		}));
		my_callback.run_all(());
	}

	#[test]
	fn u32_param_callback() {
		let mut my_callback: CallbackManager<u32> = Default::default();
		println!("Out of callback u32");
		my_callback.add(Box::new(|x| {
			println!("First callback: {}", x);
		}));
		my_callback.add(Box::new(|x| {
			println!("Second callback: {}", x);
		}));
		my_callback.run_all(7);
	}
}
