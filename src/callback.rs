use std::rc::Rc;


///
/// CallbackManager tracks callbacks and then call all then when requested.
///
#[derive(Default)]
pub struct CallbackManager<ParamType: Copy> {
	/// It is using a Box for the futures here cause the vector needs a sized type.
	callbacks: Vec<(Box<dyn Fn(ParamType)>, Rc<CallbackHandler>)>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct CallbackHandler {}

impl<ParamType: Copy> CallbackManager<ParamType> {
	pub fn add(&mut self, callback: Box<dyn Fn(ParamType)>) -> Rc<CallbackHandler> {
		let resp = Rc::new(CallbackHandler {});
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
	use crate::callback::CallbackManager;

	#[test]
	fn no_param_callback() {
		let mut my_callback: CallbackManager<()> = Default::default();
		println!("Out of callback");

		let _first_handler = my_callback.add(Box::new(|_| println!("First callback")));
		let _second_handler = my_callback.add(Box::new(|_| println!("Second callback")));
		my_callback.add(Box::new(|_| println!("First dropped callback")));
		{
			my_callback.add(Box::new(|_| println!("Second dropped callback")));
		}

		my_callback.run_all(());
	}

	#[test]
	fn u32_param_callback() {
		let mut my_callback: CallbackManager<u32> = Default::default();
		println!("Out of callback u32");

		let _first_handler = my_callback.add(Box::new(|x|
			println!("First callback: {}", x)
		));
		let _second_handler = my_callback.add(Box::new(|x|
			println!("Second callback: {}", x)
		));
		my_callback.add(Box::new(|x| println!("First dropped callback: {}", x)));
		{
			my_callback.add(Box::new(|x| println!("Second dropped callback: {}", x)));
		}

		my_callback.run_all(7);
	}
}
