struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}
// It’s possible for structs to store references to data owned by something else,
// but to do so requires the use of "lifetimes" (cf. ch10)

fn main() {
	// Create an instance of a struct
    let mut user1 = User {
	    email: String::from("someone@example.com"),
	    username: String::from("someusername123"),
	    active: true,
	    sign_in_count: 1,
	};

	user1.email = String::from("anotheremail@example.com"); // instance must be mutable

	// Struct update syntax
	let user2 = User {
	    email: String::from("another@example.com"),
	    username: String::from("anotherusername567"),
	    active: user1.active,
	    sign_in_count: user1.sign_in_count,
	};

	let user3 = User {
	    email: String::from("another@example.com"),
	    username: String::from("anotherusername567"),
	    ..user1
	};

	//Tuple Structs (no named fields)
	struct Color(i32, i32, i32);
	struct Point(i32, i32, i32);

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
}

// use a field init shorthand (when variables and fields have the same name)
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
