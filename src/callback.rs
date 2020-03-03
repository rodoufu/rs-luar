use std::rc::Rc;

type CallbackHandler = Rc<()>;

#[derive(Default)]
pub struct CallbackManager {
	manager: CallbackRegistry<()>,
}

impl CallbackManager {
	pub fn add(&mut self, callback: Box<dyn Fn()>) -> CallbackHandler {
		self.manager.add(Box::new(move |_| callback()))
	}

	pub fn run_all(&mut self) {
		self.manager.run_all(())
	}
}

///
/// CallbackRegistry tracks callbacks and then call all then when requested.
///
#[derive(Default)]
pub struct CallbackRegistry<ParamType: Copy> {
	/// It is using a Box for the futures here cause the vector needs a sized type.
	callbacks: Vec<(Box<dyn Fn(ParamType)>, CallbackHandler)>,
}

impl<ParamType: Copy> CallbackRegistry<ParamType> {
	pub fn add(&mut self, callback: Box<dyn Fn(ParamType)>) -> CallbackHandler {
		let resp = Rc::new(());
		self.callbacks.push((callback, resp.clone()));
		resp
	}

	pub fn run_all(&mut self, param: ParamType) {
		self.callbacks.retain(
			|(_, handler)| Rc::strong_count(handler) > 1);
		for (callback, _) in &self.callbacks {
			callback(param);
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::callback::{
		CallbackManager,
		CallbackRegistry,
	};

	#[test]
	fn no_param_callback() {
		let mut my_callback: CallbackManager = Default::default();
		println!("Out of callback no");

		let _first_handler = my_callback.add(Box::new(||
			println!("First callback no param")
		));
		let _second_handler = my_callback.add(Box::new(||
			println!("Second callback no param")
		));
		my_callback.add(Box::new(|| {
			println!("First dropped callback no param");
			assert!(false);
		}));
		{
			let _second_droped_handler = my_callback.add(Box::new(|| {
				println!("Second dropped callback no param");
				assert!(false);
			}));
		}

		my_callback.run_all();
	}

	#[test]
	fn empty_param_callback() {
		let mut my_callback: CallbackRegistry<()> = Default::default();
		println!("Out of callback");

		let _first_handler = my_callback.add(Box::new(|_| println!("First callback")));
		let _second_handler = my_callback.add(Box::new(|_| println!("Second callback")));
		my_callback.add(Box::new(|_| {
			println!("First dropped callback");
			assert!(false);
		}));
		{
			let _second_droped_handler = my_callback.add(Box::new(|_| {
				println!("Second dropped callback");
				assert!(false);
			}));
		}

		my_callback.run_all(());
	}

	#[test]
	fn u32_param_callback() {
		let mut my_callback: CallbackRegistry<u32> = Default::default();
		println!("Out of callback u32");

		let _first_handler = my_callback.add(Box::new(|x|
			println!("First callback: {}", x)
		));
		let _second_handler = my_callback.add(Box::new(|x|
			println!("Second callback: {}", x)
		));
		my_callback.add(Box::new(|x| {
			println!("First dropped callback: {}", x);
			assert!(false);
		}));
		{
			let _second_droped_handler = my_callback.add(Box::new(|x| {
				println!("Second dropped callback: {}", x);
				assert!(false);
			}));
		}

		my_callback.run_all(7);
	}
}
