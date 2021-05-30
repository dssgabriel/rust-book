use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
	let f = File::open("hello.txt");

// Handle with panic!
	// let f = match f {
	// 	Ok(file) => file,
	// 	Err(error) => panic!("Problem opening the file: {:?}", error),
	// };

// Matching on different errors
	let f = match f {
		Ok(file) => file,
		Err(error) => match.error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}"),
			},
			other_error => {
				panic!("Problem opening the file: {:?}")
			}
		},
	};

// More concise way of handling different errors
	let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

// Shortcuts for panic on error: unwrap and expect
	// will call panic! if file doesn't exist
	let f = File::open("hello.txt").unwrap(); 

	// let's us choose the error message printed by panic!
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};
    
	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

