//struct WithAge<T> {
//	wrapped: T,
//	age: u32,
//}
//
//struct WithName<T> {
//	wrapped: T,
//	name: String,
//}
//
//struct WithPhone<T> {
//	wrapped: T,
//	phone: String,
//}
//
//struct WithCar<T> {
//	wrapped: T,
//	car: String,
//}
//
//trait HasAge {
//	fn age(&self) -> u32;
//	fn set_age(&mut self, the_age: u32);
//}
//
//impl<T> HasAge for WithAge<T> {
//	fn age(&self) -> u32 { self.age }
//
//	fn set_age(&mut self, the_age: u32) {
//		self.age = the_age;
//	}
//}
//
//trait HasName {
//	fn name(&self) -> String;
//	fn set_name(&mut self, the_name: String);
//}
//
//impl<T> HasName for WithName<T> {
//	fn name(&self) -> String { self.name.clone() }
//
//	fn set_name(&mut self, the_name: String) {
//		self.name = the_name;
//	}
//}
//
//trait HasPhone {
//	fn phone(&self) -> String;
//	fn set_phone(&mut self, the_phone: String);
//}
//
//impl<T> HasPhone for WithPhone<T> {
//	fn phone(&self) -> String { self.phone.clone() }
//
//	fn set_phone(&mut self, the_phone: String) {
//		self.phone = the_phone;
//	}
//}
//
//trait HasCar {
//	fn car(&self) -> String;
//	fn set_car(&mut self, the_car: String);
//}
//
//impl<T> HasCar for WithCar<T> {
//	fn car(&self) -> String { self.car.clone() }
//
//	fn set_car(&mut self, the_car: String) {
//		self.car = the_car;
//	}
//}
//
//fn test<T: HasName + HasAge>(t: T) -> String {
//	let resp = format!("Hi! I'm {} and I'm {} years old", t.name(), t.age());
//	println!("{}", resp);
//	resp
//}
//
//#[cfg(test)]
//mod example {
//	trait Colored {
//		fn color(&self) -> String;
//	}
//
//	trait Numbered {
//		fn number(&self) -> u32;
//	}
//
//	trait Named {
//		fn name(&self) -> String;
//	}
//
//	struct DecorateColor<T> {
//		decorated: T,
//		color: String,
//	}
//
//	struct DecorateNumber<T> {
//		decorated: T,
//		number: u32,
//	}
//
//	struct DecorateName<T> {
//		decorated: T,
//		name: String,
//	}
//
//	impl<T> Colored for DecorateColor<T> where T: Colored {
//		fn color(&self) -> String { self.color.clone() }
//	}
//
//	impl<T> Numbered for DecorateNumber<T> where T: Numbered {
//		fn number(&self) -> u32 { self.number }
//	}
//
//	impl<T> Named for DecorateName<T> where T: Named {
//		fn name(&self) -> String { self.name.clone() }
//	}
//
//	struct Foo {}
//
//	#[test]
//	fn my_test() {
//		type ColoredFoo = DecorateName<DecorateColor<DecorateNumber<Foo>>>;
//		let f: ColoredFoo = DecorateName {
//			decorated: DecorateColor {
//				decorated: DecorateNumber {
//					decorated: Foo {},
//					number: 2,
//				},
//				color: "blue".to_string(),
//			},
//			name: "smith".to_string(),
//		};
////		assert_eq!("blue", f.color());
////		assert_eq!(2, f.number());
//	}
//}
//
//#[cfg(test)]
//mod tests {
//	use crate::extension::{
//		HasAge,
//		HasName,
//		HasCar,
//		WithAge,
//		WithName,
//		WithCar,
//		test,
//	};
//
//
//	#[test]
//	fn calling_test() {
//		/*
//		type Person = WithAge<WithName<WithCar<()>>>;
//		let mut p: Person;
//		p.set_age(10);
//		p.wrapped.name = "Fulano".to_string();
//		p.wrapped.wrapped.car = "Ford".to_string();
//		test(p);
//		*/
//	}
//}
