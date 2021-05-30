mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {} // needs to be public to call it
	}
}

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();
}
