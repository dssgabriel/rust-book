fn main() {
// Storing lists of values as vectors
	// Type annotation because no values are inserted into the vector
    let v: Vec<i32> = Vec::new();
    // Use of a macro
    let v = vec![1, 2, 3];

// Update a vector (needs to be mutable)
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

// Reading Elements of Vectors
	let v = vec![1, 2, 3, 4, 5];

	let third: &i32 = &v[2];
	println!("The third element is {}", third);

	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

// Iterating over the Values in a Vector
	let v = vec![100, 32, 57];
	for i in &v {
		println!("{}", i);
	}
	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}

// Using an Enum to store multiple Types
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];
} // <- v goes out of scope here
