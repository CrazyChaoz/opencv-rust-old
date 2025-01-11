impl {{rust_full}} {
	pub fn {{rust_as_raw_const}}(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn {{rust_as_raw_mut}}(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

tuple_extern! { {{inner_rust_full}},
	{{new_extern}}, {{delete_extern}},
	{{getters}}
}


