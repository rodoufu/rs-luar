struct WithAge<T> {
	wrapped: T,
	age: u32,
}

struct WithName<T> {
	wrapped: T,
	name: String,
}

struct WithPhone<T> {
	wrapped: T,
	phone: String,
}